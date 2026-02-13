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
import { Usage } from "./Usage.js";

export function eliminateDeadStores(
  bytecode: Instruction[],
  usage: Usage
): Instruction[] {
  const result = [];
  let neededByNext = 0; 

  for (let i = bytecode.length - 1; i >= 0; i--) {
    const [op, arg] = bytecode[i];

    if (op === "set") {
      if (typeof arg === "string" && !usage.read.has(arg)) {
        continue; 
      }
      neededByNext++;
      result.unshift(bytecode[i]);
    } 
    
    else if (op === "add") {
      if (neededByNext > 0) {
        neededByNext--;
        neededByNext += 2;
        result.unshift(bytecode[i]);
      } else {
        continue;
      }
    }

    else {
      if ((op === "inc" || op === "dec") && typeof arg === "string" && !usage.read.has(arg)) {
        continue;
      }
      result.unshift(bytecode[i]);
    }
  }

  return result;
}