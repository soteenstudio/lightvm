import { analyzeUsage } from "./analyzeUsage.js";
import { eliminateDeadStores } from "./eliminateDeadStores.js";
import { eliminateDeadLoops } from "./eliminateDeadLoops.js";
import { Instruction } from "./Instruction.js";

export function optimizeBytecode(bytecode: Instruction[]) {
  let out = [...bytecode];

  // 1. Constant Folding (Simpel: push + push + add/sub/mul/div)
  // Kita scan manual instruksi yang bisa digabung
  for (let i = 0; i < out.length - 2; i++) {
    const inst1 = out[i];
    const inst2 = out[i + 1];
    const inst3 = out[i + 2];

    if (inst1[0] === "push" && inst2[0] === "push") {
      const a = inst1[1];
      const b = inst2[1];
      
      if (typeof a === "number" && typeof b === "number") {
        let result: number | null = null;
        
        // Cek op ketiga
        if (inst3[0] === "add") result = a + b;
        else if (inst3[0] === "sub") result = a - b;
        else if (inst3[0] === "mul") result = a * b;
        else if (inst3[0] === "div" && b !== 0) result = a / b;

        if (result !== null) {
          // Ganti 3 instruksi jadi 1 push hasil
          out.splice(i, 3, ["push", result]);
          i--; // Re-check dari posisi ini
        }
      }
    }
  }

  // 2. Dead Code Elimination yang lu udah punya
  const usage = analyzeUsage(out);
  out = eliminateDeadStores(out, usage);
  out = eliminateDeadLoops(out);

  // 3. Peephole: Buang double jump atau jump + next
  for (let i = 0; i < out.length - 1; i++) {
    if (out[i][0] === "jump" && out[i][1] === i + 1) {
      out.splice(i, 1);
      i--;
    }
  }

  return out;
}
