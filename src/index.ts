/*  
 * Copyright 2026 SoTeen Studio  
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

import {
  Instruction
} from "./Instruction.js";
import {
  loadNapi
} from "./utils/loadNapi.js";
import type {
  VMEvent,
  VMResult,
  Capability,
  Listener
} from "../typings/index.d.ts";
const native = loadNapi();
const CapabilityMap = {
  "Observe": 0,
  "Control": 1,
  "Debug": 2,
  "Unsafe": 3,
  "observe": 0,
  "control": 1,
  "debug": 2,
  "unsafe": 3
};
export class LightVM {
  private instance: any;
  private listeners = new Map < VMEvent, Listener[] > ();
  constructor(caps: Capability[] = ["observe"]) {
    const numericCaps = caps.map(cap => {
      const val = CapabilityMap[cap];
      if (val === undefined) {
        throw new Error(`Capability "${cap}" gak dikenal nih, Clay!`);
      }
      return val;
    });
    this.instance = new native.LightVM(numericCaps);
  }
  load(source: Instruction[] | string) {
    let payload: string;
    if (typeof source === "string") {
      if (source.trim().startsWith('[')) {
        payload = source;
      } else {
        payload = source;
      }
    } else {
      payload = JSON.stringify(source);
    }
    console.log("Payload: ", payload);
    this.instance.load(payload);
    return this;
  }
  run(options: any = {}) {
    this.instance.run(options);
  }
  export(name: string) {
    return (...args: any[]) => {
      const rawResult = this.instance.callExported(name, JSON.stringify(args));
      const parsed = JSON.parse(rawResult);
      if (!parsed || parsed === "Undefined") return undefined;
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
    this.listeners.get(event) !.push(fn);
    return this;
  }
  private emit(event: VMEvent, payload ? : any) {
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
      halted: true
    };
  }
  tools() {
    return {
      optimizeBytecode: native.LightVM.optimizeBytecode,
      loader: {
        stringifyLTC: (json) => {
          const input = typeof json === 'string' ? json : JSON.stringify(json);
          return native.LightVM.stringifyLtc(input);
        },
        parseLTC: (code) => native.LightVM.parseLtc(code),
        parseLTCArray: (code) => native.LightVM.parseLtcArray(code)
      }
    };
  }
}
export {
  Instruction
};