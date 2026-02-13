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
        abort: (m, f, l, c) => console.error(`[Abort] ${f}:${l}:${c}`),
        trace: (m, n) => console.log(`[Trace] ${n}`)
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
