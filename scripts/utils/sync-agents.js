/**
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

import fs from 'fs';

const AGENTS_PATH = 'AGENTS.md';
const CODERABBIT_PATH = '.coderabbit.yml';

const s = {
  reset: '\x1b[0m',
  bold: '\x1b[1m',
  cyan: '\x1b[36m',
  green: '\x1b[32m',
  red: '\x1b[31m',
};

const logger = {
  success: (msg) => console.log(`${s.bold}${s.green}✔${s.reset} ${msg}`),
  error: (msg, detail) =>
    console.error(`\n${s.bold}${s.red}𐄂 ${msg}${s.reset}`, detail || ''),
};

try {
  const agentsContent = fs.readFileSync(AGENTS_PATH, 'utf8');

  const indentedAgents = agentsContent
    .split('\n')
    .map((line) => `    ${line}`)
    .join('\n');

  let crConfig = fs.readFileSync(CODERABBIT_PATH, 'utf8');

  const regex = /# AGENTS_START[\s\S]*?# AGENTS_END/;
  const replacement = `# AGENTS_START\n${indentedAgents}\n    # AGENTS_END`;

  if (regex.test(crConfig)) {
    crConfig = crConfig.replace(regex, replacement);
    fs.writeFileSync(CODERABBIT_PATH, crConfig);
    logger.success('AGENTS.md successfully synced to .coderabbit.yml!');
  } else {
    throw new Error(
      'Placeholder # AGENTS_START / # AGENTS_END not found in .coderabbit.yml',
    );
  }
} catch (err) {
  logger.error('Sync process failed:', err.message);
  process.exit(1);
}
