import { wasm } from "../runBytecode.js";
import { NumType } from "../../NunType.js";

export function lt(
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
      wasm.lt_i32(a, b) :
      arg === "lng" ?
      wasm.lt_i64(a, b) :
      arg === "flt" ? 
      wasm.lt_f32(a, b) :
      arg === "dbl" ?
      wasm.lt_f64(a, b) :
      a < b;
  } else {
    return a < b;
  }
}