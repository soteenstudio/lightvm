/**
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import { resolve } from 'node:path';

export const importVM = async () => {
  const path = resolve(process.cwd(), 'dist/index.min.mjs');
  return await import(`file://${path}`);
};
