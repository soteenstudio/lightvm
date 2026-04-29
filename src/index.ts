/*  
 * Copyright 2026 SoTeen Studio  
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

import { Instruction } from "./Instruction.js";
import { loadNapi } from "./utils/loadNapi.js";
//import { optimizeBytecode } from "./optimizeBytecode.js";
//import * as loader from "./loader.js";
import type { VMEvent, VMResult, Capability, Listener } from "../typings/index.d.ts";

const native = loadNapi();

export class LightVM {
  private instance: any;
  private listeners = new Map<VMEvent, Listener[]>();

  constructor(caps: Capability[] = ["observe"]) {
    // Panggil constructor Rust
    const finalCaps = Array.isArray(caps) ? caps : [caps];
    this.instance = new native.LightVM(finalCaps);
  }

  load(source: Instruction[] | string) {
    // Kirim langsung ke Rust. Rust bakal handle mau itu path file atau JSON string.
    const payload = typeof source === "string" ? source : JSON.stringify(source);
    console.log(`source: ${source}, payload: ${payload}`);
    this.instance.load(payload);
    return this;
  }

  run(options: any = {}) {
    // Delegasi ke Rust run()
    this.instance.run(options);
  }

  /**
   * Panggil fungsi yang di-export dari bytecode
   */
  export(name: string) {
    return (...args: any[]) => {
      // Kita pake trik JSON stringify buat kirim args ke call_exported Rust
      const rawResult = this.instance.call_exported(name, JSON.stringify(args));
      const parsed = JSON.parse(rawResult);

      // Unbox otomatis hasil dari Value enum Rust
      if (!parsed || parsed === "Undefined") return undefined;
      return typeof parsed === 'object' ? Object.values(parsed)[0] : parsed;
    };
  }

  provide(name: string, value: any) {
    // Kirim value ke Rust imports
    this.instance.provide(name, JSON.stringify(value));
    return this;
  }

  halt() {
    this.instance.halt();
  }

  /* =====================
   * Observability
   * ===================== */
  on(event: VMEvent, fn: Listener) {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
      
      // Daftarkan listener ke Rust hanya sekali per event type
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

  /* =====================
   * Introspection
   * ===================== */
  inspect() {
    return JSON.parse(this.instance.inspect());
  }

  embedded(): VMResult {
    // Kita manfaatin get_outputs yang udah kita bikin di Rust
    this.instance.clear_outputs();
    this.instance.run({});
    
    return {
      value: undefined, // Bisa ambil dari lastValue di Rust kalau mau
      outputs: this.instance.get_outputs(),
      halted: true
    };
  }

  tools() {
    //console.log("Proto: ", Object.getOwnPropertyNames(Object.getPrototypeOf(this.instance)));
    return {
      optimizeBytecode: (json: string) => typeof json === "string" ? json : this.instance.optimizeBytecode(JSON.stringify(json)),
      loader: {
        stringifyLTC: (json: string) => typeof json === "string" ? json : this.instance.stringifyLtc(JSON.stringify(json)),
        parseLTC: (code: string) => JSON.parse(this.instance.parseLtc(code))
      }
    };
  }
}

export { Instruction };
