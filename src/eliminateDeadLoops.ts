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

  for (let i = 0; i < bytecode.length; i++) {
    const [op, , , target] = bytecode[i];

    if (op === "if_false" && typeof target === "number") {
      const loopStart = target;
      const loopEnd = i;

      if (isPureLoop(bytecode, loopStart, loopEnd)) {
        i = loopEnd;
        continue;
      }
    }

    out.push(bytecode[i]);
  }

  return out;
}