/**
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import { test, expect, describe } from "unitry";
import { importVM } from "./helper/importVM.js";
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";

const { loadNapi } = await importVM();

// Set by scripts/test/test-ts.js to select which fallback scenario is
// currently staged on disk for this run: 'priority' (platform package +
// local fallback both present), 'fallback' (local fallback only), or
// 'reject-invalid-sig' (local fallback only, missing/invalid .sig).
const scenario = process.env.LIGHTVM_TEST_SCENARIO;

function runLoadNapiInSubprocess() {
  const distPath = resolve(process.cwd(), "dist/index.min.mjs");
  return spawnSync(
    process.execPath,
    ["--input-type=module", "-e", `import(${JSON.stringify(`file://${distPath}`)}).then((m) => m.loadNapi());`],
    { env: process.env, stdio: "ignore" },
  );
}

describe("loadNapi Utility", () => {

  test("loadNapi: should load and return native module (cached)", () => {

    const native1 = loadNapi();

    const native2 = loadNapi();

    expect(native1).toBe(native2);
    expect(native1).toBeDefined();
  });

  if (scenario === "priority") {
    test("loadNapi: should prioritize the platform package over the local lightvm-test fallback", () => {
      // The local fallback staged for this scenario is intentionally
      // corrupt, so a successful load proves the platform package (which
      // resolves via require.resolve) was used instead of the fallback.
      const native = loadNapi();
      expect(native).toBeDefined();
    });
  }

  if (scenario === "fallback") {
    test("loadNapi: should load the local lightvm-test/binaries/lightvm.node binary when the platform package is absent", () => {
      const native = loadNapi();
      expect(native).toBeDefined();
    });
  }

  if (scenario === "reject-invalid-sig") {
    test("loadNapi: should reject the local fallback binary when its signature is missing or invalid", () => {
      // loadNapi() calls process.exit() on verification failure, so it must
      // be exercised in a subprocess rather than in-process.
      const result = runLoadNapiInSubprocess();
      expect(result.status).not.toBe(0);
    });
  }
});
