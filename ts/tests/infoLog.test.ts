/**
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import { describe, expect, test } from "unitry";
import { humanizeVersion } from "../src/utils/infoLog.js";

describe("humanizeVersion Utility", () => {
  test("humanizeVersion: should format a nightly version", () => {
    expect(humanizeVersion("0.1.0-alpha.9-nightly.20260828.d36cc1e")).toBe(
      "0.1.0-alpha.9 (Nightly 28 Aug 2026, d36cc1e)",
    );
  });

  const months = [
    ["01", "Jan"],
    ["02", "Feb"],
    ["03", "Mar"],
    ["04", "Apr"],
    ["05", "May"],
    ["06", "Jun"],
    ["07", "Jul"],
    ["08", "Aug"],
    ["09", "Sep"],
    ["10", "Oct"],
    ["11", "Nov"],
    ["12", "Dec"],
  ];

  for (const [month, name] of months) {
    test(`humanizeVersion: should format month ${month} as ${name}`, () => {
      expect(humanizeVersion(`1.0.0-nightly.2026${month}15.abc123`)).toBe(
        `1.0.0 (Nightly 15 ${name} 2026, abc123)`,
      );
    });
  }

  test("humanizeVersion: should preserve an unknown numeric month", () => {
    expect(humanizeVersion("1.0.0-nightly.20261315.abc123")).toBe(
      "1.0.0 (Nightly 15 13 2026, abc123)",
    );
  });

  test("humanizeVersion: should preserve a stable version", () => {
    expect(humanizeVersion("1.0.0")).toBe("1.0.0");
  });

  test("humanizeVersion: should preserve incomplete nightly metadata", () => {
    expect(humanizeVersion("1.0.0-nightly.20260828")).toBe(
      "1.0.0-nightly.20260828",
    );
  });

  test("humanizeVersion: should preserve a nightly date without eight characters", () => {
    expect(humanizeVersion("1.0.0-nightly.2026082.abc123")).toBe(
      "1.0.0-nightly.2026082.abc123",
    );
  });
});
