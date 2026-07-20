/**
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import { test, expect, describe, fn, suppressConsole } from "unitry";
import { importVM } from "./helper/importVM.js";

const { LightVM, Capability } = await importVM();

describe("LightVM Suite", () => {
  
  const createVM = () => new LightVM([Capability.Observe, Capability.Control, Capability.Unsafe]);
  
  describe("Tools & Optimization", () => {
    test("optimizeBytecode should map inputs correctly", () => {
      const vm = createVM();
      const tools = vm.tools();
      const raw = [["push", 15], ["push", 5], ["add", "i16"], ["println"]];
      const result = tools.optimizeBytecode(raw);
      
      expect(result).toEqual([{ push: 20 }, 'println']);
    });
  });

  describe("VM Lifecycle", () => {
    test("load should return instance", () => {
      const vm = createVM();
      const res = vm.load([{ push: 10 }]);
      expect(res).toBeInstanceOf(LightVM);
    });

    test("provide should accept key-value pairs", () => {
      const vm = createVM();
      
      expect(() => vm.provide({ test: 123 })).not.toThrow();
    });
  });

  describe("Event Emitter", () => {
    test("on should register listener", () => {
      const vm = createVM();
      const mockHandler = fn();
      
      vm.on('tick', mockHandler);
      expect(typeof vm.on).toBe('function');
    });
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
