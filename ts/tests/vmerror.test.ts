/**
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import { test, expect, describe } from "unitry";
import { importVM } from "./helper/importVM.js";

const { VMError } = await importVM();

describe("VMError Class", () => {
  test("VMError: should correctly set properties and format message", () => {
    const msg = "Terjadi sesuatu yang salah";
    const err = new VMError(msg);
    
    expect(err.code).toBe("LVM500");
    expect(err.ip).toBe(0);
    
    expect(err).toBeInstanceOf(VMError);
    expect(err.name).toBe("SystemError");
    
    expect(err.message).toContain(msg);
    expect(err.message).toContain("LVM500");
  });

  test("VMError: should be throwable", () => {
    
    expect(() => {
      throw new VMError("Test error");
    }).toThrow();
  });
});
