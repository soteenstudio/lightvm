/**
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import fs from 'fs';

const s = {
  reset: '\x1b[0m',
  bold: '\x1b[1m',
  dim: '\x1b[2m',
  cyan: '\x1b[36m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  red: '\x1b[31m',
};

const logger = {
  info: (msg) => console.log(`${s.bold}${s.cyan}⠋${s.reset} ${msg}`),
  success: (msg) => console.log(`${s.bold}${s.green}✔${s.reset} ${msg}`),
  warn: (msg) => console.log(`${s.bold}${s.yellow}⚠${s.reset} ${msg}`),
  error: (msg) => console.error(`${s.bold}${s.red}𐄂 ${msg}${s.reset}`),
};

try {
  logger.info(`${s.bold}Checking for local dependencies...${s.reset}`);

  const pkg = JSON.parse(fs.readFileSync('./package.json', 'utf8'));
  const allDeps = { ...pkg.dependencies, ...pkg.devDependencies };
  const badDeps = [];

  for (const [name, version] of Object.entries(allDeps)) {
    if (
      version.endsWith('.tgz') ||
      version.startsWith('./') ||
      version.startsWith('../')
    ) {
      badDeps.push(name);
    }
  }

  if (badDeps.length > 0) {
    logger.warn(
      `${s.bold}Found local/tgz dependencies:${s.reset} ${s.cyan}${badDeps.join(', ')}${s.reset}`,
    );
    logger.error(
      `Please uninstall with: ${s.dim}npm uninstall ${badDeps.join(' ')}${s.reset}`,
    );
    process.exit(1);
  } else {
    logger.success(`${s.bold}All dependencies are clean!${s.reset}`);
  }
} catch (err) {
  logger.error(`Failed to check dependencies: ${err.message}`);
  process.exit(1);
}
