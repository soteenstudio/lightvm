/**
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import { test, expect, describe, suppressConsole } from "unitry";
import { spawnSync } from "node:child_process";
import { importVM } from "./helper/importVM.js";

const { LightVM, Capability, VMEvent } = await importVM();

describe("LightVM Suite", () => {
  
  const createVM = () => new LightVM([Capability.Observe, Capability.Control, Capability.Unsafe]);
  
  describe("Tools & Optimization", () => {
    test("optimizeBytecode should map inputs correctly", () => {
      const vm = createVM();
      const tools = vm.tools();
      const raw = [["push", 15], ["push", 5], ["add", "i16"], ["println"]];
      const result = tools.optimizeBytecode(raw);
      
      expect(result).toEqual([{ push_int16: 20 }, 'println']);
    });

    test("bench should run through the public tools wrapper", () => {
      const vm = new LightVM({ caps: [Capability.Debug] });
      const tools = vm.tools();

      expect(() =>
        tools.bench("wrapper-bench").samples(1).targetTime(1).run(
          () => 1,
          (state) => state + 1,
        ),
      ).not.toThrow();
    });
  });

  describe("VM Lifecycle", () => {
    test("load should return instance", () => {
      const vm = createVM();
      const res = vm.load([{ push: 10 }]);
      expect(res).toBeInstanceOf(LightVM);
    });

    test("embedded should return the N-API execution result", () => {
      const vm = createVM();
      vm.load([["push", 42], ["stop"]]);

      expect(vm.embedded()).toEqual({
        value: 42,
        outputs: [],
        halted: false,
      });
    });

    test("export should return handles for functions and variables", () => {
      const vm = new LightVM({
        caps: [Capability.Observe, Capability.Control],
        runtimeConfig: { nightly: true },
      });
      vm.load([
        ["jump", 7],
        ["func", "add", 2, 2, 6, "a", "b"],
        ["get", "a"],
        ["get", "b"],
        ["add", "int"],
        ["return"],
        ["stop"],
        ["export", "add"],
        ["val", "x"],
        ["push", 5],
        ["set", "x"],
        ["export", "x"],
        ["val", "unset"],
        ["export", "unset"],
      ]);

      const add = vm.export("add");
      const x = vm.export("x");
      const unset = vm.export("unset");

      expect(typeof add).toBe("object");
      expect(typeof add.call).toBe("function");
      expect(add.call(5, 6)).toBe(11);
      expect(x.call()).toBe(5);
      expect(unset.call()).toBe(undefined);
    });

    test("provide should accept key-value pairs", () => {
      const vm = createVM();
      
      expect(() => vm.provide({ test: 123 })).not.toThrow();
    });
  });

  describe("Event Emitter", () => {
    if (process.env.LIGHTVM_TEST_SCENARIO !== "reject-invalid-sig") {
      test("on should deliver tick event data", () => {
        const result = spawnSync(
          process.execPath,
          [
            "--input-type=module",
            "--eval",
            `import { Capability, LightVM, VMEvent } from './dist/index.min.mjs';
const vm = new LightVM({ caps: [Capability.Observe, Capability.Control] });
let delivered = false;
vm.load([["push", 1]]);
vm.on(VMEvent.Tick, (data) => {
  delivered = true;
  console.log(JSON.stringify(data));
});
vm.run();
setTimeout(() => {
  if (!delivered) process.exitCode = 1;
}, 50);`,
          ],
          { cwd: process.cwd(), encoding: "utf8", timeout: 1_000 },
        );

        expect(result.error).toBe(undefined);
        expect(result.status).toBe(0);
        expect(JSON.parse(result.stdout.trim())).toEqual({
          event: "Tick",
          payload: { state: "start" },
        });
      });

      test("on should deliver start and finish event data", () => {
        const result = spawnSync(
          process.execPath,
          [
            "--input-type=module",
            "--eval",
            `import { Capability, LightVM, VMEvent } from './dist/index.min.mjs';
const vm = new LightVM({ caps: [Capability.Observe, Capability.Control] });
const delivered = [];
vm.load([["push", 1]]);
const receive = (data) => {
  delivered.push(data);
  if (delivered.length === 2) console.log(JSON.stringify(delivered));
};
vm.on(VMEvent.Start, receive);
vm.on(VMEvent.Finish, receive);
vm.run();
setTimeout(() => {
  if (delivered.length !== 2) process.exitCode = 1;
}, 50);`,
          ],
          { cwd: process.cwd(), encoding: "utf8", timeout: 1_000 },
        );

        expect(result.error).toBe(undefined);
        expect(result.status).toBe(0);
        expect(JSON.parse(result.stdout.trim())).toEqual([
          { event: "Start", payload: { operation: "run" } },
          { event: "Finish", payload: { operation: "run" } },
        ]);
      });

      test("on should not keep the process alive without an event", () => {
        const result = spawnSync(
          process.execPath,
          [
            "--input-type=module",
            "--eval",
            `import { LightVM, VMEvent } from './dist/index.min.mjs';
const vm = new LightVM();
vm.on(VMEvent.Tick, () => {});`,
          ],
          { cwd: process.cwd(), timeout: 1_000 },
        );

        expect(result.error).toBe(undefined);
        expect(result.status).toBe(0);
      });
    }
  });
  
  describe("Capability Validation", () => {
    const testCases = [
      { cap: Capability.Observe, expected: true },
      { cap: Capability.Control, expected: true },
      { cap: Capability.Debug, expected: true },
    ];
  
    testCases.forEach(({ cap, expected }) => {
      test(`Should handle capability: ${cap}`, () => {
        const vm = new LightVM([cap]);
        expect(vm).toBeInstanceOf(LightVM);
      });
    });
  });
});
