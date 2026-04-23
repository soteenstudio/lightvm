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
import { optimizeBytecode } from "./optimizeBytecode.js";
import * as loader from "./loader.js";
import fs from "fs";
import type { VMEvent, VMResult, Capability, Listener } from "../typings/index.d.ts";

const napi = loadNapi();

export class LightVM {
  private bytecode: Instruction[] = [];
  private listeners = new Map<VMEvent, Listener[]>();
  private caps = new Set<Capability>();
  private state: "idle" | "running" | "halted" = "idle";

  private outputs: string[] = [];
  private lastValue: any = undefined;
  
  private functions = new Map<string, {
    start: number;
    params: number;
  }>();
  private exported = new Set<string>();
  private imports = new Map<string, any>();
  
  constructor(caps: Capability[] = ["observe"]) {
    caps.forEach(c => this.caps.add(c));
  }

  private require(cap: Capability) {
    if (!this.caps.has(cap)) {
      throw new Error(`Capability '${cap}' not granted`);
    }
  }

  private indexFunctions() {
    this.functions.clear();
  
    for (let i = 0; i < this.bytecode.length; i++) {
      const [op, name, params, start] = this.bytecode[i];
      if (op === "func") {
        this.functions.set(name as string, {
          start: start as number,
          params: params as number
        });
      }
    }
  }

  private indexExports() {
    this.exported.clear();
  
    for (const inst of this.bytecode) {
      const [op, name] = inst;
      if (op === "export") {
        this.exported.add(name as string);
      }
    }
  }

  load(bytecode: Instruction[] | string) {
    if (typeof bytecode === "string") {
      const code = fs.existsSync(bytecode)
        ? fs.readFileSync(bytecode, "utf8")
        : bytecode;
  
      this.bytecode = loader.parseLTC(code);
    } else {
      this.bytecode = bytecode;
    }
  
    this.indexFunctions();
    this.indexExports();
    return this;
  }

  run() {
    this.require("control");

    if (!this.bytecode.length) {
      throw new Error("No bytecode loaded");
    }

    this.state = "running";
    this.emit("tick", { state: "start" });

    try {
      napi.runBytecode(JSON.stringify(this.bytecode), {
        __imports: this.imports
      });
      this.state = "halted";
      this.emit("halt");
    } catch (e) {
      this.emit("panic", e);
      throw e;
    }
  }

  embedded(): VMResult {
    this.require("control");

    this.outputs = [];
    this.lastValue = undefined;

    const originalLog = console.log;
    const originalWrite = process.stdout.write;

    console.log = (...args: any[]) => {
      this.outputs.push(args.join(" "));
    };

    process.stdout.write = ((chunk: any) => {
      this.outputs.push(String(chunk));
      return true;
    }) as any;

    try {
      this.state = "running";
      runBytecode(this.bytecode);
      this.state = "halted";
    } finally {
      console.log = originalLog;
      process.stdout.write = originalWrite;
    }

    return {
      value: this.lastValue,
      outputs: [...this.outputs],
      halted: true
    };
  }

  export(name: string) {
    this.require("control");
    if (!this.exported.has(name)) throw new Error(`Function '${name}' is not exported`);
  
    const fn = this.functions.get(name);
    if (!fn) throw new Error(`Function '${name}' not found`);
  
    return (...args: any[]) => {
      this.state = "running";
      try {
        // Langsung stringify array mentah ["push", 10], gak pake mapping sampah!
        const payload = JSON.stringify(this.bytecode);
        
        const options = JSON.stringify({
          entry: fn.start,
          args: args.map(arg => {
              if (typeof arg === 'number') return Number.isInteger(arg) ? { Int64: arg } : { Float64: arg };
              if (typeof arg === 'boolean') return { Bool: arg };
              return { String: String(arg) };
          }),
          capture_return: true
        });
  
        // Tetep manggil napi.runBytecode sesuai instruksi lo
        const rawResult = napi.runBytecode(payload, options); 
        const parsed = JSON.parse(rawResult);
  
        // Unbox hasil otomatis berdasarkan 4 tipe data lo
        if (!parsed || parsed === "Undefined") return undefined;
        return typeof parsed === 'object' ? Object.values(parsed)[0] : parsed;
      } finally {
        this.state = "halted";
      }
    };
  }

  provide(name: string, value: any) {
    this.require("control");
    this.imports.set(name, value);
    return this;
  }

  halt() {
    this.require("unsafe");
    this.state = "halted";
    this.emit("halt");
  }

  /* =====================
   * Observability
   * ===================== */
  on(event: VMEvent, fn: Listener) {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
    }
    this.listeners.get(event)!.push(fn);
    return this;
  }

  private emit(event: VMEvent, payload?: any) {
    const list = this.listeners.get(event);
    if (!list) return;
    for (const fn of list) fn(payload);
  }

  /* =====================
   * Introspection (SAFE)
   * ===================== */
  inspect() {
    this.require("observe");
    return Object.freeze({
      state: this.state,
      instructions: this.bytecode.length,
      capabilities: [...this.caps]
    });
  }

  /* =====================
   * Debug Surface (NO CORE ACCESS)
   * ===================== */
  debug() {
    this.require("debug");
    return Object.freeze({
      emitHotpath(ip: number) {
        console.log("🔥 hot ip:", ip);
      }
    });
  }
  
  tools() {
    return {
      runBytecode: napi.runBytecode,
      optimizeBytecode,
      loader: {
        stringifyLTC: loader.stringifyLTC,
        parseLTC: loader.parseLTC
      }
    }
  }
}

export { Instruction };