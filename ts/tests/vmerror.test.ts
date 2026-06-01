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
import { VMError } from "../dist/index.min.mjs";

test('VMError should have correct properties and message', () => {
  const detail = "An error occurred";
  const error = new VMError(detail);
  expect(error instanceof Error).toBe(true);
  expect(error instanceof VMError).toBe(true);
  expect(error.code).toBe('LVM500');
  expect(error.ip).toBe(0);
  expect(error.message).toSatisfy((msg: string) => msg.includes(detail));
});
  