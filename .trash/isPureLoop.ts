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

export function isPureLoop(bytecode: Instruction[], start: number, end: number) {
  for (let i = start; i <= end; i++) {
    const [op] = bytecode[i];
    if (
      op === "print" ||
      op === "println" ||
      op === "call" ||
      op === "import" ||
      op === "return"
    ) {
      return false;
    }
  }
  return true;
}