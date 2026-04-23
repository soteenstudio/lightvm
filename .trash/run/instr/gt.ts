import { wasm } from "../runBytecode.js";
import { NumType } from "../../NunType.js";

export function gt(
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
      wasm.gt_i32(a, b) :
      arg === "lng" ?
      wasm.gt_i64(a, b) :
      arg === "flt" ? 
      wasm.gt_f32(a, b) :
      arg === "dbl" ?
      wasm.gt_f64(a, b) :
      a > b;
  } else {
    return a > b;
  }
}