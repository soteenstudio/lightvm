/**
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import type { ErrorOptions } from './ErrorOptions.js';
import type { RuntimeConfig } from './RuntimeConfig.js';
import type { SecurityConfig } from './SecurityConfig.js';

export type VMConfig = {
  caps: Array<number>;
  errorOptions: ErrorOptions | null;
  runtimeConfig: RuntimeConfig | null;
  securityConfig: SecurityConfig | null;
};
