import { wasm } from "../runBytecode.js";
import { NumType } from "../../NunType.js";

export function mod(
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
      wasm.mod_i32(a, b) :
      arg === "lng" ?
      wasm.mod_i64(a, b) :
      arg === "flt" ? 
      wasm.mod_f32(a, b) :
      arg === "dbl" ?
      wasm.mod_f64(a, b) :
      a % b;
  } else {
    return a % b;
  }
}