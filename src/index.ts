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

// Sesuaikan urutan ini sama yang ada di Rust (capability.rs)
const CapabilityMap = {
  "Observe": 0,
  "Control": 1,
  "Debug": 2,
  "Unsafe": 3,
  // Kalau di JS pake lowercase, tinggal ganti key-nya jadi kecil semua
  "observe": 0,
  "control": 1,
  "debug": 2,
  "unsafe": 3
};


export class LightVM {
  private instance: any;
  private listeners = new Map<VMEvent, Listener[]>();

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
    console.log("Instr: ", source);
    let payload: any;
    
    if (typeof source === "string") {
      try {
        // Coba parse, kalau ini string JSON (hasil parseLTC), 
        // kita ubah jadi object biar Rust masuk ke blok 'else' (serde_json::from_value)
        payload = JSON.parse(source);
      } catch {
        // Kalau bukan JSON, berarti ini path file atau raw LTC code
        payload = source;
      }
    } else {
      payload = JSON.stringify(source);
    }

    // Kirim ke Rust. Kalau payload berupa Object/Array, Rust bakal pake from_value.
    // Kalau payload berupa String (path), Rust bakal ngebaca filenya.
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
    return {
      optimizeBytecode: native.LightVM.optimizeBytecode,
      loader: {
        stringifyLTC: (json) => {
          const input = typeof json === 'string' ? json : JSON.stringify(json);
          return native.LightVM.stringifyLtc(input);
        },
        parseLTC: (code) => native.LightVM.parseLtc(code)
      }
    };
  }
}

export { Instruction };
