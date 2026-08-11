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
export enum TargetArch {
  AArch64 = 0,
}
export enum FileType {
  Assembly = 0,
  Binary = 1,
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
      throw err instanceof Error ? err : new Error(String(err));
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
    this.instance[method](val);
    (this.config[key] as any)[sub] = val;
    return this;
  }

  setMaxIo(value: number) {
    return this.updateConfig('securityConfig', 'maxIo', value, 'setMaxIo');
  }

  setMaxImport(value: number) {
    return this.updateConfig(
      'securityConfig',
      'maxImport',
      value,
      'setMaxImport',
    );
  }

  setMaxAlloc(value: number) {
    return this.updateConfig(
      'securityConfig',
      'maxAlloc',
      value,
      'setMaxAlloc',
    );
  }

  setMaxCall(value: number) {
    return this.updateConfig('securityConfig', 'maxCall', value, 'setMaxCall');
  }

  setMaxJump(value: number) {
    return this.updateConfig('securityConfig', 'maxJump', value, 'setMaxJump');
  }

  setMaxTicks(value: number) {
    return this.updateConfig(
      'securityConfig',
      'maxTicks',
      value,
      'setMaxTicks',
    );
  }

  setMaxStackSize(value: number) {
    return this.updateConfig(
      'securityConfig',
      'maxStackSize',
      value,
      'setMaxStackSize',
    );
  }

  setAllowedImports(value: Array<string>) {
    return this.updateConfig(
      'securityConfig',
      'allowedImports',
      value,
      'setAllowedImports',
    );
  }

  withUnsafeMode(enabled: boolean) {
    return this.updateConfig(
      'securityConfig',
      'unsafeMode',
      enabled,
      'withUnsafeMode',
    );
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
    return this.wrap(() => this.instance.run(options));
  }

  compile(config: CompileConfig) {
    return this.wrap(() =>
      this.instance.compile(config.targetArch, config.fileType, config.path),
    );
  }

  export(name: string) {
    return (...args: any[]) => {
      return this.wrap(() => {
        const rawResult = this.instance.callExported(
          name,
          JSON.stringify(args),
        );

        if (rawResult == null || rawResult === 'Undefined') return undefined;

        return typeof rawResult === 'object' && !Array.isArray(rawResult)
          ? Object.values(rawResult)[0]
          : rawResult;
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
        let bytesVal: number | undefined = undefined;
        return {
          bytes: (b: number) => {
            bytesVal = b;
            return {
              run: (setup: () => any, fn: (state: any) => any) => {
                return this.wrap(() =>
                  this.native.LightVM.bench(name, setup, fn, bytesVal ?? null),
                );
              },
            };
          },
          run: (setup: () => any, fn: (state: any) => any) => {
            return this.wrap(() =>
              this.native.LightVM.bench(name, setup, fn, null),
            );
          },
        };
      },
      optimizeBytecode: (bytecode: any) => {
        return this.wrap(() =>
          this.native.LightVM.optimizeBytecode(
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
