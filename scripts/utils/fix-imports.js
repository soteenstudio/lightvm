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
import path from 'path';

const s = {
  reset: '\x1b[0m',
  bold: '\x1b[1m',
  cyan: '\x1b[36m',
  green: '\x1b[32m',
  red: '\x1b[31m',
};

const logger = {
  info: (msg) => console.log(`${s.bold}${s.cyan}⠋${s.reset} ${msg}`),
  success: (msg) => console.log(`${s.bold}${s.green}✔${s.reset} ${msg}`),
  error: (msg) => console.error(`${s.bold}${s.red}𐄂 ${msg}${s.reset}`),
};

const sourceDir = path.join(process.cwd(), 'ts', 'src', 'generated');
const targetDir = path.join(process.cwd(), 'ts', 'src', 'generated');

if (!fs.existsSync(targetDir)) {
  fs.mkdirSync(targetDir, { recursive: true });
}

function processFile(filePath) {
  let content = fs.readFileSync(filePath, 'utf8');

  content = content.replace(/from\s+["'](\.\.?\/.*?)["']/g, (match, p1) => {
    if (p1.endsWith('.js') || p1.endsWith('.ts')) return match;
    return `from '${p1}.js'`;
  });

  const fileName = path.basename(filePath);
  const targetPath = path.join(targetDir, fileName);

  fs.writeFileSync(targetPath, content);
  logger.success(`Successfully processed: ${fileName}`);
}

try {
  logger.info(`${s.bold}Processing generated TS files...${s.reset}`);

  fs.readdirSync(sourceDir).forEach((file) => {
    if (file.endsWith('.ts')) {
      processFile(path.join(sourceDir, file));
    }
  });
} catch (err) {
  logger.error(`Failed to process files: ${err.message}`);
  process.exit(1);
}
