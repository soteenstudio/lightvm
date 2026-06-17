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
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const s = {
  reset: '\x1b[0m',
  bold: '\x1b[1m',
  dim: '\x1b[2m',
  cyan: '\x1b[36m',
  green: '\x1b[32m',
};

const filePath = join(__dirname, '../.testings/browser.html');

console.log(
  `${s.bold}${s.cyan}🧹 Starting HTML comment cleanup...${s.reset}\n`,
);

if (fs.existsSync(filePath)) {
  let html = fs.readFileSync(filePath, 'utf8');

  html = html.replace(/<!--[\s\S]*?-->/g, '');

  html = html.replace(
    /(<script\b[\s\S]*?>)([\s\S]*?)(<\/script\b[^>]*>)/gi,
    (match, openTag, scriptContent, closeTag) => {
      const cleanScript = scriptContent
        .replace(/\/\*[\s\S]*?\*\//g, '')
        .replace(/([^:]|^)\/\/.*$/gm, '$1');
      return openTag + cleanScript + closeTag;
    },
  );

  html = html.replace(
    /(<style\b[\s\S]*?>)([\s\S]*?)(<\/style\b[^>]*>)/gi,
    (match, openTag, scriptContent, closeTag) => {
      const cleanScript = scriptContent.replace(/\/\*[\s\S]*?\*\//g, '');
      return openTag + cleanScript + closeTag;
    },
  );

  fs.writeFileSync(filePath, html, 'utf8');

  console.log(
    `${s.bold}${s.green}✔ Cleanup complete!${s.reset} ${s.dim}Successfully removed comments from ./index.html${s.reset}`,
  );
} else {
  console.log(
    `${s.bold}${s.dim}✨ File index.html not found. Nothing to clean.${s.reset}`,
  );
}
