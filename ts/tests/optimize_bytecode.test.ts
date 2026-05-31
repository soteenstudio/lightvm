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
test('optimize bytecode test', () => {
  const result = JSON.stringify(tools.optimizeBytecode(raw));
  expect(result).toBe(JSON.stringify([ { push: 20 }, 'println' ]));
});