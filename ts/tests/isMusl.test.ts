import { test, expect } from "unitry";
import { isMusl } from "../dist/index.min.mjs";

test('isMusl should return a boolean', () => {
  const result = isMusl();
  expect(typeof result === 'boolean').toBe(true);
});
 