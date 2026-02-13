// index.d.ts
import { Instruction } from './Instruction.js';
import { runBytecode } from './runBytecode.js';
import { optimizeBytecode } from './optimizeBytecode.js';
import * as loader from './loader.js';

export type VMEvent = 'tick' | 'halt' | 'panic';
export type Listener = (payload?: any) => void;
export type Capability = 'control' | 'observe' | 'debug' | 'unsafe';

export interface VMResult {
  value: any;
  outputs: string[];
  halted: boolean;
}

export class LightVM {
  private bytecode: Instruction[];
  // ... rest of your class properties ...

  constructor(caps?: Capability[]);

  load(bytecode: Instruction[] | string): this;
  run(): void;
  embedded(): VMResult;
  export(name: string): (...args: any[]) => any;
  provide(name: string, value: any): this;
  halt(): void;
  on(event: VMEvent, fn: Listener): this;
  inspect(): { state: string; instructions: number; capabilities: Capability[] };
  debug(): { emitHotpath(ip: number): void };
  tools(): {
    runBytecode: typeof runBytecode;
    optimizeBytecode: typeof optimizeBytecode;
    loader: typeof loader;
  };
}

export { Instruction };
