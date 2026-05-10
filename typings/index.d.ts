/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import { Instruction } from './Instruction.js';
import { runBytecode } from './runBytecode.js';
import { optimizeBytecode } from './optimizeBytecode.js';
import * as loader from './loader.js';
export type VMEvent = 'Tick' | 'Halt' | 'Panic';
export type Listener = (payload?: any) => void;
export interface VMResult {
  value: any;
  outputs: string[];
  halted: boolean;
}
export class LightVM {
  private bytecode: Instruction[];
  constructor(caps?: Capability[]);
  load(bytecode: Instruction[] | string): this;
  run(): void;
  embedded(): VMResult;
  export(name: string): (...args: any[]) => any;
  provide(name: string, value: any): this;
  halt(): void;
  on(event: VMEvent, fn: Listener): this;
  inspect(): {
    state: string;
    instructions: number;
    capabilities: Capability[];
  };
  debug(): { emitHotpath(ip: number): void };
  tools(): {
    runBytecode: typeof runBytecode;
    optimizeBytecode: typeof optimizeBytecode;
    loader: typeof loader;
  };
}
export { Instruction };
