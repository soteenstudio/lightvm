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
import { loadNapi } from './utils/loadNapi.js';
import { isMusl } from './utils/isMusl.js';
import { VMSystemError as VMError } from './utils/vmerror.js';
export type VMEvent = 'Tick' | 'Halt' | 'Panic';
export type Listener = (payload?: any) => void;
export interface VMResult {
    value: any;
    outputs: string[];
    halted: boolean;
}
export declare enum Capability {
    Observe = "OBSERVE",
    Control = "CONTROL",
    Debug = "DEBUG",
    Unsafe = "UNSAFE"
}
export declare class LightVM {
    private instance;
    private listeners;
    constructor(caps?: Capability[]);
    load(source: Instructions[] | string): this;
    run(options?: any): void;
    export(name: string): (...args: any[]) => any;
    provide(nameOrObj: string | any, value?: any): this;
    halt(): void;
    on(event: VMEvent, fn: Listener): this;
    private emit;
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
