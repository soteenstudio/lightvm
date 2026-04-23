import { wasm } from "../runBytecode.js";
import { NumType } from "../../NunType.js";

export function ge(
  a: number,
  b: number,
  ip: number,
  hitCounter: Record<number, number>,
  HOT_THRESHOLD: number,
  ctx: { JITCF: boolean },
  arg: NumType
): boolean {
  if (hitCounter[ip] >= HOT_THRESHOLD) {
    ctx.JITCF = true;
    return arg === "int" ?
      wasm.ge_i32(a, b) :
      arg === "lng" ?
      wasm.ge_i64(a, b) :
      arg === "flt" ? 
      wasm.ge_f32(a, b) :
      arg === "dbl" ?
      wasm.ge_f64(a, b) :
      a >= b;
  } else {
    return a >= b;
  }
}