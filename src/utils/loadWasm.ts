/*  
 * Copyright 2026 SoTeen Studio  
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

import fs from "fs";
import { createRequire } from "module";
import { CustomError } from "../error.js";

let dirnamePath = null;

try {
  dirnamePath = import.meta.url;
} catch {
  dirnamePath = __dirname;
}

const require = createRequire(dirnamePath);

export function loadWasm(filename: string) {
  try {
    const packageName = "lightvm"; 
    const targetPath = `${packageName}/assembly/dist/${filename}`;
    
    const filePath = require.resolve(targetPath);
    
    const buffer = fs.readFileSync(filePath);
    const module = new WebAssembly.Module(buffer);

    const importObject = {
      env: {
        abort: (m: number, f: number, l: number, c: number) => console.error(`[Abort] ${f}:${l}:${c}`),
        trace: (m: number, n: number) => console.log(`[Trace] ${n}`)
      }
    };

    const instance = new WebAssembly.Instance(module, importObject);
    return instance.exports;
  } catch (err) {
    throw new CustomError(
      "Internal",
      "Not found system file main.wasm",
      null,
      null
    );
  }
}
