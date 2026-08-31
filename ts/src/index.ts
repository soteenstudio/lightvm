/**
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import { Instructions } from './generated/Instructions.js';
import { VMConfig } from './generated/VMConfig.js';
import { CompileConfig } from './generated/CompileConfig.js';
import { loadNapi } from './utils/loadNapi.js';
import { isMusl } from './utils/isMusl.js';
import { VMSystemError as VMError } from './utils/vmerror.js';
import { formatInfoVM } from './utils/infoLog.js';
export type Listener = (payload?: any) => void;
export interface VMResult {
  value: any;
  outputs: string[];
  halted: boolean;
}
export interface ExportedHandle {
  call: (...args: any[]) => any;
}
export enum Capability {
  Control = 0,
  Observe = 1,
  Debug = 2,
  Unsafe = 3,
}
export enum VMEvent {
  Tick = 0,
  Halt = 1,
  Panic = 2,
  Start = 3,
  Finish = 4,
}
export enum TargetArch {
  AArch64 = 0,
}
export enum FileType {
  Assembly = 0,
  Binary = 1,
}
export enum TimeBudget {
  Cheap = 0,
  Normal = 1,
  Expensive = 2,
}
export class LightVM {
  private native: any;
  private instance: any;
  private config: VMConfig;

  private static readonly DEFAULTS: VMConfig = {
    caps: [Capability.Observe],
    runtimeConfig: { nightly: false },
    errorOptions: { backtrace: false, explain: false, hint: true },
    securityConfig: {
      maxIo: 100,
      maxImport: 3,
      maxAlloc: 50,
      maxCall: 200,
      maxJump: 100,
      maxTicks: 1_000_000,
      maxStackSize: 128,
      allowedImports: [],
      unsafeMode: false,
    },
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
      securityConfig: {
        ...LightVM.DEFAULTS.securityConfig,
        ...config.securityConfig,
      },
    } as VMConfig;

    this.native = loadNapi(
      this.config.errorOptions?.explain ?? false,
      this.config.errorOptions?.hint ?? true,
    );
    this.instance = this.wrap(
      () =>
        new this.native.LightVM({
          capsRaw: this.config.caps,
          runtimeConfig: this.config.runtimeConfig,
          errorOptions: this.config.errorOptions,
        }),
    );
  }

  private wrap<T>(fn: () => T): T {
    try {
      return fn();
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      console.error(message);
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

  private updateConfig(
    key: 'runtimeConfig' | 'errorOptions' | 'securityConfig',
    sub: string,
    val: boolean | number | Array<string>,
    methodName?: string,
  ) {
    const method = methodName ?? `with${sub[0].toUpperCase() + sub.slice(1)}`;
    this.wrap(() => this.instance[method](val));
    (this.config[key] as any)[sub] = val;
    return this;
  }

  setMaxIo = (val: number) =>
    this.updateConfig('securityConfig', 'maxIo', val, 'setMaxIo');
  setMaxImport = (val: number) =>
    this.updateConfig('securityConfig', 'maxImport', val, 'setMaxImport');
  setMaxAlloc = (val: number) =>
    this.updateConfig('securityConfig', 'maxAlloc', val, 'setMaxAlloc');
  setMaxCall = (val: number) =>
    this.updateConfig('securityConfig', 'maxCall', val, 'setMaxCall');
  setMaxJump = (val: number) =>
    this.updateConfig('securityConfig', 'maxJump', val, 'setMaxJump');
  setMaxTicks = (val: number) =>
    this.updateConfig('securityConfig', 'maxTicks', val, 'setMaxTicks');
  setMaxStackSize = (val: number) =>
    this.updateConfig('securityConfig', 'maxStackSize', val, 'setMaxStackSize');
  setAllowedImports = (val: string[]) =>
    this.updateConfig(
      'securityConfig',
      'allowedImports',
      val,
      'setAllowedImports',
    );
  setTimeBudget = (val: number) =>
    this.updateConfig('securityConfig', 'timeBudget', val, 'setTimeBudget');
  withUnsafeMode = (en: boolean) =>
    this.updateConfig('securityConfig', 'unsafeMode', en, 'withUnsafeMode');
  withNightly = (en: boolean) =>
    this.updateConfig('runtimeConfig', 'nightly', en);
  withBacktrace = (en: boolean) =>
    this.updateConfig('errorOptions', 'backtrace', en);
  withExplain = (en: boolean) =>
    this.updateConfig('errorOptions', 'explain', en);
  withHint = (en: boolean) => this.updateConfig('errorOptions', 'hint', en);

  info() {
    return formatInfoVM(this.wrap(() => this.instance.info()));
  }

  load(source: Instructions[] | string) {
    const payload =
      typeof source === 'string' ? source : JSON.stringify(source);

    this.wrap(() => this.instance.load(payload));
    return this;
  }

  run(options: any = {}) {
    return this.wrap(() => this.instance.run(options));
  }

  compile(config: CompileConfig) {
    return this.wrap(() =>
      this.instance.compile(config.targetArch, config.fileType, config.path),
    );
  }

  export(name: string): ExportedHandle {
    return {
      call: (...args: any[]) =>
        this.wrap(() => {
          const rawResult = this.instance.callExport(name, args);

          if (rawResult == null || rawResult === 'Undefined') return undefined;

          return rawResult;
        }),
    };
  }

  provide(nameOrObj: string | Record<string, any>, value?: any) {
    this.wrap(() => {
      if (typeof nameOrObj === 'object') {
        Object.entries(nameOrObj).forEach(([k, v]) =>
          this.instance.provide(k, v),
        );
      } else {
        this.instance.provide(nameOrObj, value);
      }
    });
    return this;
  }

  halt() {
    this.wrap(() => this.instance.halt());
  }

  on(event: VMEvent, fn: Listener) {
    this.wrap(() =>
      this.instance.on(event, (payload: string) => {
        fn(this.parseSafe(payload));
      }),
    );

    return this;
  }

  inspect() {
    return this.wrap(() => this.instance.inspect());
  }

  embedded(): VMResult {
    return this.wrap(() => {
      this.instance.clear_outputs();
      this.instance.run({});
      return {
        value: undefined,
        outputs: this.instance.get_outputs(),
        halted: true,
      };
    });
  }

  tools() {
    const securityConfig = this.config?.securityConfig;
    const runtimeConfig = this.config?.runtimeConfig;
    const errorOptions = this.config?.errorOptions;
    return {
      blackBox: (value: any) => {
        return this.wrap(() => this.native.LightVM.blackBox(value));
      },
      bench: (name: string) => {
        let b: number | null = null,
          samples: number | null = null,
          targetTime: number | null = null;
        const builder = {
          bytes: (val: number) => ((b = val), builder),
          samples: (val: number) => ((samples = val), builder),
          targetTime: (val: number) => ((targetTime = val), builder),
          run: (setup: () => any, fn: (state: any) => any) =>
            this.wrap(() =>
              this.instance.bench(name, setup, fn, b, samples, targetTime),
            ),
        };
        return builder;
      },
      optimizeBytecode: (bytecode: any) => {
        return this.wrap(() =>
          this.instance.optimizeBytecode(
            bytecode,
            securityConfig?.maxIo ?? 100,
            securityConfig?.maxImport ?? 3,
            securityConfig?.maxAlloc ?? 50,
            securityConfig?.maxCall ?? 200,
            securityConfig?.maxJump ?? 100,
            securityConfig?.maxTicks ?? 1_000_000,
            securityConfig?.maxStackSize ?? 128,
            securityConfig?.allowedImports ?? [],
            securityConfig?.unsafeMode ?? true,
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
