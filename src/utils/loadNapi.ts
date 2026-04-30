import { createRequire } from 'module';
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';
const _dirname = typeof __dirname !== 'undefined' 
  ? __dirname 
  : dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
let cachedNative: any = null;
export function loadNapi() {
  if (cachedNative) return cachedNative;
  const NATIVE_PATH = resolve(_dirname, '../rust/target/release/liblightvm_rust.so');
  try {
    cachedNative = require(NATIVE_PATH);
    return cachedNative;
  } catch (err) {
    const moduleObj = { exports: {} };
    // @ts-ignore
    process.dlopen(moduleObj, NATIVE_PATH);
    cachedNative = moduleObj.exports;
    return cachedNative;
  }
}
