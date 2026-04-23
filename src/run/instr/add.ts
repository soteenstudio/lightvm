import { wasm } from "../runBytecode.js";
import { NumType } from "../../NunType.js";
import { loadNapi } from "../../utils/loadNapi.js";

//const napi = loadNapi();

export function add(
  a: number,
  b: number,
  ip: number,
  arg: NumType
): number | string {
  /*console.log(napi);
  return;*/
  
  return a + b;
  
  /*const input = new Float32Array();
  input[0] = a;
  input[1] = b;
    return arg === "int" ?
      napi.addI32(a, b) :
      arg === "lng" ?
      napi.addI64(a, b) :
      arg === "flt" ? 
      napi.addF32(input) :
      arg === "dbl" ?
      napi.addF64(a, b) :
      a + b;*/
}