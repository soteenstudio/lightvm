/**
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import { Instructions } from './generated/Instructions.js';
import { VMConfig } from './generated/VMConfig.js';
import { loadNapi } from './utils/loadNapi.js';
import { isMusl } from './utils/isMusl.js';
import { VMSystemError as VMError } from './utils/vmerror.js';
export type Listener = (payload?: any) => void;
export interface VMResult {
  value: any;
  outputs: string[];
  halted: boolean;
}
export enum Capability {
  Observe = 0,
  Control = 1,
  Debug = 2,
  Unsafe = 3,
}
export enum VMEvent {
  Tick = 0,
  Halt = 1,
  Panic = 2,
}
export class LightVM {
  private native: any;
  private instance: any;
  private config: VMConfig;

  private static readonly DEFAULTS: VMConfig = {
    caps: [Capability.Observe],
    runtimeConfig: { nightly: false },
    errorOptions: { backtrace: false, explain: false, hint: true },
  };

  constructor(config: Partial<VMConfig> & { caps?: Capability[] } = {}) {
    this.config = {
      caps: config.caps ?? LightVM.DEFAULTS.caps,
      runtimeConfig: {
        ...LightVM.DEFAULTS.runtimeConfig,
        ...config.runtimeConfig,
      },
      errorOptions: {
        ...LightVM.DEFAULTS.errorOptions,
        ...config.errorOptions,
      },
    } as VMConfig;

    this.native = loadNapi(
      this.config.errorOptions?.explain ?? false,
      this.config.errorOptions?.hint ?? true,
    );
    this.instance = new this.native.LightVM({
      capsRaw: this.config.caps,
      runtimeConfig: this.config.runtimeConfig,
      errorOptions: this.config.errorOptions,
    });
  }

  private wrap<T>(fn: () => T): T {
    try {
      return fn();
    } catch (err) {
      console.error((err as Error).message);
      process.exit(1);
    }
  }

  private parseSafe(payload: string): any {
    try {
      return JSON.parse(payload);
    } catch {
      return payload;
    }
  }

  private updateConfig(key: keyof VMConfig, subKey: string, value: boolean) {
    this.instance[`with${subKey.charAt(0).toUpperCase() + subKey.slice(1)}`](
      value,
    );
    if (this.config[key]) {
      (this.config[key] as any)[subKey] = value;
    }
    return this;
  }

  withNightly(enabled: boolean) {
    return this.updateConfig('runtimeConfig', 'nightly', enabled);
  }

  withBacktrace(enabled: boolean) {
    return this.updateConfig('errorOptions', 'backtrace', enabled);
  }

  withExplain(enabled: boolean) {
    return this.updateConfig('errorOptions', 'explain', enabled);
  }

  withHint(enabled: boolean) {
    return this.updateConfig('errorOptions', 'hint', enabled);
  }

  load(source: Instructions[] | string) {
    const payload =
      typeof source === 'string' ? source : JSON.stringify(source);

    this.wrap(() => this.instance.load(payload));
    return this;
  }

  run(options: any = {}) {
    this.wrap(() => this.instance.run(options));
  }

  export(name: string) {
    return (...args: any[]) => {
      return this.wrap(() => {
        const rawResult = this.instance.callExported(
          name,
          JSON.stringify(args),
        );
        const parsed = JSON.parse(rawResult);

        if (!parsed || parsed === 'Undefined') return undefined;
        return typeof parsed === 'object' ? Object.values(parsed)[0] : parsed;
      });
    };
  }

  provide(nameOrObj: string | any, value?: any) {
    if (typeof nameOrObj === 'object') {
      for (const [key, val] of Object.entries(nameOrObj)) {
        this.instance.provide(key, val);
      }
    } else {
      this.instance.provide(nameOrObj, value);
    }
    return this;
  }

  halt() {
    this.wrap(() => this.instance.halt());
  }

  on(event: VMEvent, fn: Listener) {
    this.wrap(() =>
      this.instance.on(event, (payload: string) => fn(this.parseSafe(payload))),
    );
    return this;
  }

  inspect() {
    return this.wrap(() => this.instance.inspect());
  }

  embedded(): VMResult {
    try {
      this.instance.clear_outputs();
      this.instance.run({});
      return {
        value: undefined,
        outputs: this.instance.get_outputs(),
        halted: true,
      };
    } catch (err) {
      console.error((err as Error).message);
      process.exit(1);
    }
  }

  tools() {
    const runtimeConfig = this.config?.runtimeConfig;
    const errorOptions = this.config?.errorOptions;
    return {
      optimizeBytecode: (bytecode: any) => {
        return this.wrap(() =>
          this.native.LightVM.optimizeBytecode(
            bytecode,
            runtimeConfig?.nightly ?? false,
            errorOptions?.backtrace ?? false,
            errorOptions?.explain ?? false,
            errorOptions?.hint ?? true,
          ),
        );
      },
      stringifyLTC: (json: Instructions[]) => {
        return this.wrap(() => this.native.LightVM.stringifyLtc(json));
      },
      parseLTC: (code: string) => {
        return this.wrap(() => this.native.LightVM.parseLtc(code));
      },
      parseLTCArray: (code: string) => {
        return this.wrap(() => this.native.LightVM.parseLtcArray(code));
      },
    };
  }
}
export { Instructions, loadNapi, isMusl, VMError };
