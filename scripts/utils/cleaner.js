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
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const s = {
  reset: '\x1b[0m',
  bold: '\x1b[1m',
  dim: '\x1b[2m',
  cyan: '\x1b[36m',
  green: '\x1b[32m',
  red: '\x1b[31m',
};

function cleanProject() {
  const rootDir = path.resolve(__dirname, '..');

  console.log(`${s.bold}${s.cyan}🧹 Starting project cleanup...${s.reset}\n`);
  let deletedCount = 0;

  const targetsDir = ['dist', 'binaries', 'pkg', 'dist-test', 'publish'];

  targetsDir.forEach((dirName) => {
    const targetPath = path.join(rootDir, dirName);
    if (fs.existsSync(targetPath)) {
      try {
        fs.rmSync(targetPath, { recursive: true, force: true });
        console.log(
          `  ${s.bold}${s.dim}↳ Removed directory:${s.reset} ${s.bold}./${dirName}${s.reset}`,
        );
        deletedCount++;
      } catch (err) {
        console.error(
          `  ${s.bold}${s.red}𐄂 Failed to remove ./${dirName}:${s.reset}`,
          err.message,
        );
      }
    }
  });

  try {
    const files = fs.readdirSync(rootDir);
    const tgzFiles = files.filter((file) => file.endsWith('.tgz'));

    tgzFiles.forEach((file) => {
      const filePath = path.join(rootDir, file);
      try {
        fs.unlinkSync(filePath);
        console.log(
          `  ${s.bold}${s.dim}↳ Removed tarball:${s.reset} ${s.bold}./${file}${s.reset}`,
        );
        deletedCount++;
      } catch (err) {
        console.error(
          `  ${s.bold}${s.red}𐄂 Failed to remove ./|${file}:${s.reset}`,
          err.message,
        );
      }
    });
  } catch (err) {
    console.error(
      `  ${s.bold}${s.red}𐄂 Failed to read directory:${s.reset}`,
      err.message,
    );
  }

  console.log('');
  if (deletedCount > 0) {
    console.log(
      `${s.bold}${s.green}✔ Cleanup complete!${s.reset} ${s.dim}Successfully removed ${deletedCount} item(s).${s.reset}`,
    );
  } else {
    console.log(
      `${s.bold}${s.dim}✨ Project is already clean. Nothing to delete.${s.reset}`,
    );
  }
}

cleanProject();
