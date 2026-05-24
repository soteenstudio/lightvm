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
    try {
      this.instance.run(options);
    } catch (err) {
      console.error(err.message);
      process.exit(1);
    }
  }
  export(name: string) {
    try {
      return (...args: any[]) => {
        const rawResult = this.instance.callExported(name, JSON.stringify(args));
        const parsed = JSON.parse(rawResult);
        if (!parsed || parsed === 'Undefined') return undefined;
        return typeof parsed === 'object' ? Object.values(parsed)[0] : parsed;
      };
    } catch (err) {
      console.error(err.message);
      process.exit(1);
    }
  }
  provide(name: string, value: any) {
    try {
      this.instance.provide(name, JSON.stringify(value));
      return this;
    } catch (err) {
      console.error(err.message);
      process.exit(1);
    }
  }
  halt() {
    try {
      this.instance.halt();
    } catch (err) {
      console.error(err.message);
      process.exit(1);
    }
  }
  on(event: VMEvent, fn: Listener) {
    try {
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
    } catch (err) {
      console.error(err.message);
      process.exit(1);
    }
  }
  private emit(event: VMEvent, payload?: any) {
    try {
      const list = this.listeners.get(event);
      if (list) {
        for (const fn of list) fn(payload);
      }
    } catch (err) {
      console.error(err.message);
      process.exit(1);
    }
  }
  inspect() {
    try {
      return JSON.parse(this.instance.inspect());
    } catch (err) {
      console.error(err.message);
      process.exit(1);
    }
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
      console.error(err.message);
      process.exit(1);
    }
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
