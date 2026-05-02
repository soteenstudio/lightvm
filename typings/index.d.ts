import { Instruction } from './Instruction.js';
import { runBytecode } from './runBytecode.js';
import { optimizeBytecode } from './optimizeBytecode.js';
import * as loader from './loader.js';
export type VMEvent = 'Tick' | 'Halt' | 'Panic'
export type Listener = (payload?: any) => void;
export type Capability = 'Control' | 'Observe' | 'Debug' | 'Unsafe';
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
  inspect(): { state: string; instructions: number; capabilities: Capability[] };
  debug(): { emitHotpath(ip: number): void };
  tools(): {
    runBytecode: typeof runBytecode;
    optimizeBytecode: typeof optimizeBytecode;
    loader: typeof loader;
  };
}
export { Instruction };