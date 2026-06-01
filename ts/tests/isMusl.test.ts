/**
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import { test, expect } from "unitry";
import { isMusl } from "../dist/index.min.mjs";

test('isMusl should return a boolean', () => {
  const result = isMusl();
  expect(typeof result === 'boolean').toBe(true);
});
  