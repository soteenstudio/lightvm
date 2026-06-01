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
test('load test', () => {
  const optimized = tools.optimizeBytecode(raw);
  const result = vm.load(optimized);
  const length = Object.keys(result).length;
  const type = typeof result;
  expect(length > 0 && type === 'object').toBe(true);
});
test('run test', () => {
  const optimized = tools.optimizeBytecode(raw);
  vm.load(optimized);
  expect(() => vm.run()).not.toThrow();
});
test('provide test', () => {
  const result = vm.provide({ author: 'SoTeen Studio', country: 'Indonesia' });
  const length = Object.keys(result).length;
  const type = typeof result;
  expect(length > 0 && type === 'object').toBe(true);
});
test('inspect test', () => {
  const result = vm.inspect();
  expect(result.capabilities.length > 0 && result.instructions !== 0).toBe(true);
});
test('optimize bytecode test', () => {
  const result = JSON.stringify(tools.optimizeBytecode(raw));
  expect(result).toBe(JSON.stringify([ { push: 20 }, 'println' ]));
});