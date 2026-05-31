import { test, expect } from "unitry";
import { LightVM, Capability } from "../dist/index.min.mjs";

const vm = new LightVM([Capability.Observe, Capability.Control]);
const tools = vm.tools();
const raw = [
  ["push", 15],
  ["push", 5],
  ["add", "i16"],
  ["println"]
];
const optimized = tools.optimizeBytecode(raw);
const result = JSON.stringify(vm.load(optimized));
test('load test(not undefined)', () => {
  expect(result).not.toBe(undefined);
});
test('load test(not null)', () => {
  expect(result).not.toBe(null);
});