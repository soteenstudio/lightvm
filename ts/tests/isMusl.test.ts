/**
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import { test, expect, describe, fn } from "unitry";
import { importVM } from "./helper/importVM.js";

const { isMusl } = await importVM();

describe("isMusl Utility", () => {
  test("isMusl: should return true when report has no glibc", () => {
    
    const mockReportProvider = {
      getReport: () => ({ header: {} }) 
    };

    expect(isMusl(mockReportProvider)).toBe(true);
  });

  test("isMusl: should handle error gracefully", () => {
    
    const brokenProvider = {
      getReport: () => { throw new Error("Failed"); }
    };
    
    const result = isMusl(brokenProvider);
    expect(typeof result).toBe('boolean');
  });
});
