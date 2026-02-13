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

export function parseLTC(code: string): Instruction[] {
  return code
    .replace(/\s\;\;\sIP\=(\d+)/g, "")
    .split(";")
    .map(s => s.trim())
    .filter(Boolean)
    .map(line => {
      const parts = line.split(/\s+/);
      const op = parts[0] as Instruction[0];
      const args = parts.slice(1).map(arg => {
        const num = Number(arg);
        return isNaN(num) ? arg : num;
      });

      while (args.length < 4) args.push("");
      return [op, ...args] as Instruction;
    });
}

export function stringifyLTC(code: Instruction): string {
  return code
    .map((item: any, index) => item.join(' ') + `; ;; IP=${index}`)
    .join('\n');
}