/**
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

export type SecurityConfig = {
  maxIo: number | null;
  maxImport: number | null;
  maxAlloc: number | null;
  maxCall: number | null;
  maxJump: number | null;
  maxTicks: number | null;
  maxStackSize: number | null;
  allowedImports: Array<string> | null;
  unsafeMode: boolean | null;
};
