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
import { execSync } from 'child_process';
import { join } from 'path';
import os from 'os';

const args = process.argv.slice(2);
const isSilent = args.includes('--silent');
const STDIO_MODE = isSilent ? 'ignore' : 'inherit';

const TARGET_NAME = 'lightvm.node';
const DEST_DIR = join(process.cwd(), 'binaries');
const DEST_PATH = join(DEST_DIR, TARGET_NAME);

let rustBinaryName = 'liblightvm.so';
if (os.platform() === 'win32') {
  rustBinaryName = 'lightvm.dll';
} else if (os.platform() === 'darwin') {
  rustBinaryName = 'liblightvm.dylib';
}

const SOURCE_PATH = join(process.cwd(), 'target', 'release', rustBinaryName);

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
  info: (msg) =>
    !isSilent && console.log(`${s.bold}${s.cyan}⠋${s.reset} ${msg}`),
  success: (msg) =>
    !isSilent && console.log(`${s.bold}${s.green}✔${s.reset} ${msg}\n`),
  step: (msg) =>
    !isSilent && console.log(`${s.bold}${s.cyan}ℹ${s.reset} ${msg}`),
  cleanup: (msg) =>
    !isSilent && console.log(`${s.bold}${s.dim}🧹 ${msg}${s.reset}`),
  error: (msg, detail) =>
    console.error(`\n${s.bold}${s.red}𐄂 ${msg}${s.reset}`, detail || ''),
};

function cleanupOldBinary() {
  if (fs.existsSync(DEST_PATH)) {
    fs.unlinkSync(DEST_PATH);
    logger.cleanup('Removed old target binary from binaries/');
  }
}

try {
  logger.info(`${s.bold}Bundling JS project via esbuild...${s.reset}`);
  execSync('node ./esbuild.config.js', {
    stdio: STDIO_MODE,
  });
  logger.success(`${s.bold}esbuild bundling success!${s.reset}`);

  logger.info(
    `${s.bold}Building Rust binary${s.reset} ${s.dim}(release)...${s.reset}`,
  );
  execSync('cargo build --release --features node', {
    cwd: process.cwd(),
    stdio: STDIO_MODE,
  });
  logger.success(`${s.bold}Rust release build success!${s.reset}`);

  logger.info(`${s.bold}Linking native binary to target...${s.reset}`);

  if (!fs.existsSync(DEST_DIR)) {
    fs.mkdirSync(DEST_DIR, { recursive: true });
  }

  if (fs.existsSync(SOURCE_PATH)) {
    cleanupOldBinary();
    fs.copyFileSync(SOURCE_PATH, DEST_PATH);
    logger.success(
      `${s.bold}Binary successfully moved to:${s.reset} ${s.bold}${s.cyan}./binaries/${TARGET_NAME}${s.reset}`,
    );
  } else {
    throw new Error(`Compiled Rust binary not found at: ${SOURCE_PATH}`);
  }
} catch (err) {
  logger.error('Build compilation workflow failed:', err.message);
  process.exit(1);
}
