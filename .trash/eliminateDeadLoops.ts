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
import { isPureLoop } from "./isPureLoop.js";

export function eliminateDeadLoops(bytecode: Instruction[]): Instruction[] {
  const out: Instruction[] = [];
  // Kita perlu track index original ke index di 'out'
  const indexMap = new Map<number, number>();

  for (let i = 0; i < bytecode.length; i++) {
    const inst = bytecode[i];
    const [op, , , target] = inst;

    // Cek apakah ini instruksi jump yang balik ke belakang (Backwards Jump)
    // Biasanya ini penanda akhir dari sebuah loop
    if ((op === "jump" || op === "if_false" || op === "if_true") && typeof target === "number" && target < i) {
      const loopStart = target;
      const loopEnd = i;

      if (isPureLoop(bytecode, loopStart, loopEnd)) {
        // DAPET! Loop ini "pure" (gak ada side effect).
        // Kita hapus instruksi yang udah terlanjur masuk ke 'out' 
        // mulai dari posisi loopStart tadi.
        const outStartIndex = indexMap.get(loopStart);
        
        if (outStartIndex !== undefined) {
          // Potong array 'out' dari titik awal loop sampe sekarang
          out.splice(outStartIndex);
          // Jangan lupa lanjut ke instruksi berikutnya, instruksi jump ini jgn di-push
          continue; 
        }
      }
    }

    // Catat posisi index 'out' buat tiap index original bytecode
    indexMap.set(i, out.length);
    out.push(inst);
  }

  return out;
}
