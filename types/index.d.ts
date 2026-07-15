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
export declare enum Capability {
    Observe = 0,
    Control = 1,
    Debug = 2,
    Unsafe = 3
}
export declare enum VMEvent {
    Tick = 0,
    Halt = 1,
    Panic = 2
}
export declare class LightVM {
    private native;
    private instance;
    private config;
    private static readonly DEFAULTS;
    constructor(config?: Partial<VMConfig> & {
        caps?: Capability[];
    });
    private wrap;
    private parseSafe;
    private updateConfig;
    withNightly(enabled: boolean): this;
    withBacktrace(enabled: boolean): this;
    withExplain(enabled: boolean): this;
    withHint(enabled: boolean): this;
    load(source: Instructions[] | string): this;
    run(options?: any): void;
    export(name: string): (...args: any[]) => any;
    provide(nameOrObj: string | any, value?: any): this;
    halt(): void;
    on(event: VMEvent, fn: Listener): this;
    inspect(): any;
    embedded(): VMResult;
    tools(): {
        optimizeBytecode: (bytecode: any) => any;
        stringifyLTC: (json: Instructions[]) => any;
        parseLTC: (code: string) => any;
        parseLTCArray: (code: string) => any;
    };
}
export { Instructions, loadNapi, isMusl, VMError };
