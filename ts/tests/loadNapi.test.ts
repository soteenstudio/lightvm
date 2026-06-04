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

const { loadNapi } = await importVM();

describe("loadNapi Utility", () => {
  
  test("loadNapi: should load and return native module (cached)", () => {
    
    const native1 = loadNapi();
    
    const native2 = loadNapi();
    
    expect(native1).toBe(native2);
    expect(native1).toBeDefined();
  });
});
