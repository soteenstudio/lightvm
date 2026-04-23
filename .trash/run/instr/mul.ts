import { wasm } from "../runBytecode.js";
import { NumType } from "../../NunType.js";

export function mul(
  a: number,
  b: number,
  ip: number,
  hitCounter: Record<number, number>,
  HOT_THRESHOLD: number,
  ctx: { JITCF: boolean },
  arg: NumType
): number {
  if (hitCounter[ip] >= HOT_THRESHOLD) {
    ctx.JITCF = true;
    return arg === "int" ?
      wasm.mul_i32(a, b) :
      arg === "lng" ?
      wasm.mul_i64(a, b) :
      arg === "flt" ? 
      wasm.mul_f32(a, b) :
      arg === "dbl" ?
      wasm.mul_f64(a, b) :
      a * b;
  } else {
    return a * b;
  }
}