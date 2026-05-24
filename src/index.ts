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
import { loadNapi } from './utils/loadNapi.js';
import type { VMEvent, VMResult, Listener } from '../typings/index.d.ts';
export enum Capability {
  Observe = 'OBSERVE',
  Control = 'CONTROL',
  Debug = 'DEBUG',
  Unsafe = 'UNSAFE',
}
const native = loadNapi();
export class LightVM {
  private instance: any;
  private listeners = new Map<VMEvent, Listener[]>();
  constructor(caps: Capability[] = [Capability.Observe]) {
    const numericCaps = caps.map((cap) => {
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
    this.instance = new native.LightVM(numericCaps);
  }
  load(source: Instruction[] | string) {
    try {
      let payload: string;
      if (typeof source === 'string') {
        if (source.trim().startsWith('[')) {
          payload = source;
        } else {
          payload = source;
        }
      } else {
        payload = JSON.stringify(source);
      }
      this.instance.load(payload);
      return this;
    } catch (err) {
      console.error(err.message);
      process.exit(1);
    }
  }
  run(options: any = {}) {
    this.instance.run(options);
  }
  export(name: string) {
    return (...args: any[]) => {
      const rawResult = this.instance.callExported(name, JSON.stringify(args));
      const parsed = JSON.parse(rawResult);
      if (!parsed || parsed === 'Undefined') return undefined;
      return typeof parsed === 'object' ? Object.values(parsed)[0] : parsed;
    };
  }
  provide(name: string, value: any) {
    this.instance.provide(name, JSON.stringify(value));
    return this;
  }
  halt() {
    this.instance.halt();
  }
  on(event: VMEvent, fn: Listener) {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
      this.instance.on(event, (payload: string) => {
        let data;
        try {
          data = JSON.parse(payload);
        } catch {
          data = payload;
        }
        this.emit(event, data);
      });
    }
    this.listeners.get(event)!.push(fn);
    return this;
  }
  private emit(event: VMEvent, payload?: any) {
    const list = this.listeners.get(event);
    if (list) {
      for (const fn of list) fn(payload);
    }
  }
  inspect() {
    return JSON.parse(this.instance.inspect());
  }
  embedded(): VMResult {
    this.instance.clear_outputs();
    this.instance.run({});
    return {
      value: undefined,
      outputs: this.instance.get_outputs(),
      halted: true,
    };
  }
  tools() {
    return {
      optimizeBytecode: (bytecode: any) => {
        try {
          return native.LightVM.optimizeBytecode(bytecode);
        } catch (err) {
          console.error(err.message);
          process.exit(1);
        }
      },
      loader: {
        stringifyLTC: (json) => {
          try {
            const input = typeof json === 'string' ? json : JSON.stringify(json);
            return native.LightVM.stringifyLtc(input);
          } catch (err) {
            console.error(err.message);
            process.exit(1);
          }
        },
        parseLTC: (code) => {
          try {
            return native.LightVM.parseLtc(code);
          } catch (err) {
            console.error(err.message);
            process.exit(1);
          }
        },
        parseLTCArray: (code) => {
          try {
            return native.LightVM.parseLtcArray(code);
          } catch (err) {
            console.error(err.message);
            process.exit(1);
          }
        },
      },
    };
  }
}
export { Instruction };
