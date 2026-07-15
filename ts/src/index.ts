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
  Observe = 'OBSERVE',
  Control = 'CONTROL',
  Debug = 'DEBUG',
  Unsafe = 'UNSAFE',
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

  constructor(
    config: Partial<Omit<VMConfig, 'caps'>> & {
      caps?: (Capability | string | number)[];
    } = {
      caps: [Capability.Observe],
      runtimeConfig: {
        nightly: false,
      },
      errorOptions: {
        backtrace: false,
        explain: false,
        hint: true,
      },
    },
  ) {
    this.config = {
      caps: config.caps ?? [Capability.Observe],
      runtimeConfig: {
        nightly: config.runtimeConfig?.nightly ?? false,
      },
      errorOptions: {
        backtrace: config?.errorOptions?.backtrace ?? false,
        explain: config?.errorOptions?.explain ?? false,
        hint: config?.errorOptions?.hint ?? true,
      },
    } as VMConfig;
    const runtimeConfig = config?.runtimeConfig;
    const errorOptions = config?.errorOptions;

    const capsList = config.caps || [Capability.Observe];
    const numericCaps = capsList.map((cap) => {
      if (typeof cap === 'number') return cap;

      switch (cap.toUpperCase()) {
        case 'OBSERVE':
          return 0;
        case 'CONTROL':
          return 1;
        case 'DEBUG':
          return 2;
        case 'UNSAFE':
          return 3;
        default:
          throw new Error(`Unknown capability ${cap}`);
      }
    });

    this.native = loadNapi(
      errorOptions?.explain ?? false,
      errorOptions?.hint ?? true,
    );
    this.instance = new this.native.LightVM({
      capsRaw: numericCaps,
      runtimeConfig: {
        nightly: runtimeConfig?.nightly ?? false,
      },
      errorOptions: {
        backtrace: errorOptions?.backtrace ?? false,
        explain: errorOptions?.explain ?? false,
        hint: errorOptions?.hint ?? true,
      },
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

  withNightly(enabled: boolean) {
    this.instance.withNightly(enabled);
    if (this.config.runtimeConfig) {
      this.config.runtimeConfig.nightly = enabled;
    }
    return this;
  }

  withBacktrace(enabled: boolean) {
    this.instance.withBacktrace(enabled);
    if (this.config.errorOptions) {
      this.config.errorOptions.backtrace = enabled;
    }
    return this;
  }

  withExplain(enabled: boolean) {
    this.instance.withExplain(enabled);
    if (this.config.errorOptions) {
      this.config.errorOptions.explain = enabled;
    }
    return this;
  }

  withHint(enabled: boolean) {
    this.instance.withHint(enabled);
    if (this.config.errorOptions) {
      this.config.errorOptions.hint = enabled;
    }
    return this;
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
    this.wrap(() => {
      this.instance.on(event, (payload: string) => {
        let data;
        try {
          data = JSON.parse(payload);
        } catch {
          data = payload;
        }
        fn(data);
      });
    });
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
        return this.wrap(
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
