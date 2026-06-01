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
