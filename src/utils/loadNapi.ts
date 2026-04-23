import { createRequire } from 'module';
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';

// Support buat ESM dan CJS sekaligus
const _dirname = typeof __dirname !== 'undefined' 
  ? __dirname 
  : dirname(fileURLToPath(import.meta.url));

const require = createRequire(import.meta.url);

// Cache di level module scope, biar cuma di-load 1x seumur hidup
let cachedNative: any = null;

export function loadNapi() {
  if (cachedNative) return cachedNative;

  // Pastiin nge-point ke RELEASE build biar kenceng!
  const NATIVE_PATH = resolve(_dirname, '../rust/target/release/liblightvm_rust.so');

  try {
    // require() pada file .so itu sangat cepat karena Node.js 
    // pake internal dlopen yang udah di-optimize
    cachedNative = require(NATIVE_PATH);
    return cachedNative;
  } catch (err) {
    // Fallback kalau require gagal (khusus beberapa environment)
    const moduleObj = { exports: {} };
    // @ts-ignore
    process.dlopen(moduleObj, NATIVE_PATH);
    cachedNative = moduleObj.exports;
    return cachedNative;
  }
}
