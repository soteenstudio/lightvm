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

export function analyzeUsage(bytecode: Instruction[]): Usage {
  const read = new Set<string>();
  const written = new Set<string>();

  for (const [op, arg] of bytecode) {
    if (op === "get") read.add(arg as string);
    if (op === "set" || op === "inc" || op === "dec") {
      written.add(arg as string);
    }
    if (op === "print" || op === "println") {
      read.add("*IO*");
    }
    if (op === "return") {
      read.add("*RETURN*");
    }
  }

  return { read, written };
}