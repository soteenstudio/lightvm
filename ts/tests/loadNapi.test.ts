import { test, expect } from "unitry";
import { loadNapi } from "../dist/index.min.mjs";

test('loadNapi should load the native module without error', () => {
  const native = loadNapi();
  expect(native !== undefined).toBe(true);
});
 