/*  
 * Copyright 2026 SoTeen Studio  
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

import { analyzeUsage } from "./analyzeUsage.js";
import { eliminateDeadStores } from "./eliminateDeadStores.js";
import { eliminateDeadLoops } from "./eliminateDeadLoops.js";
import { Instruction } from "./Instruction.js";

export function optimizeBytecode(bytecode: Instruction[]) {
  let out = [...bytecode];

  for (let i = 0; i < out.length - 2; i++) {
    const inst1 = out[i];
    const inst2 = out[i + 1];
    const inst3 = out[i + 2];

    if (inst1[0] === "push" && inst2[0] === "push") {
      const a = inst1[1];
      const b = inst2[1];
      
      if (typeof a === "number" && typeof b === "number") {
        let result: number | null = null;
        
        if (inst3[0] === "add") result = a + b;
        else if (inst3[0] === "sub") result = a - b;
        else if (inst3[0] === "mul") result = a * b;
        else if (inst3[0] === "div" && b !== 0) result = a / b;

        if (result !== null) {
          out.splice(i, 3, ["push", result]);
          i--;
        }
      }
    }
  }

  const usage = analyzeUsage(out);
  out = eliminateDeadStores(out, usage);
  out = eliminateDeadLoops(out);

  for (let i = 0; i < out.length - 1; i++) {
    if (out[i][0] === "jump" && out[i][1] === i + 1) {
      out.splice(i, 1);
      i--;
    }
  }

  return out;
}
