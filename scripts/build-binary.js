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

const args = process.argv.slice(2);
const isLocal = args.includes('--local');
const isDebug = args.includes('--debug');
const isProduction = args.includes('--production');
const isPublish = args.includes('--publish');
const isSilent = args.includes('--silent');

const STDIO_MODE = isSilent ? 'ignore' : 'inherit';
const PKG_PATH = join(process.cwd(), 'package.json');
const RUST_BINARY_NAME = 'liblightvm.so';
const TARGET_NAME = 'lightvm.node';
const RUST_OUT_DIR = isDebug ? 'debug' : 'release';

const SOURCE_PATH = join(
  process.cwd(),
  'target',
  RUST_OUT_DIR,
  RUST_BINARY_NAME,
);
const DEST_DIR = join(process.cwd(), 'binaries');
const DEST_PATH = join(DEST_DIR, TARGET_NAME);
const DIST_DIR = join(process.cwd(), 'dist');

const ORIGINAL_PKG_DATA = JSON.parse(fs.readFileSync(PKG_PATH, 'utf-8'));

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
  warn: (msg) =>
    !isSilent && console.log(`${s.bold}${s.yellow}⚠${s.reset} ${msg}`),
  error: (msg, detail) =>
    console.error(`\n${s.bold}${s.red}𐄂 ${msg}${s.reset}`, detail || ''),
};

function modifyPackageJson(mode) {
  const pkg = JSON.parse(fs.readFileSync(PKG_PATH, 'utf-8'));
  if (!pkg.files) pkg.files = [];
  const targetDir = 'binaries';

  if (mode === 'local') {
    if (!pkg.files.includes(targetDir)) {
      pkg.files.push(targetDir);
    }

    pkg.name = 'lightvm-test';
    fs.writeFileSync(PKG_PATH, JSON.stringify(pkg, null, 2));
  } else {
    fs.writeFileSync(PKG_PATH, JSON.stringify(ORIGINAL_PKG_DATA, null, 2));
  }
}

function cleanup() {
  logger.cleanup('Cleaning up build directories...');

  if (fs.existsSync(DIST_DIR)) {
    fs.rmSync(DIST_DIR, { recursive: true, force: true });
    logger.cleanup('  ↳ Removed dist/');
  }

  if (fs.existsSync(DEST_DIR)) {
    fs.rmSync(DEST_DIR, { recursive: true, force: true });
    logger.cleanup('  ↳ Removed binaries/');
  }
}

try {
  logger.info(
    `${s.bold}Building Rust binary${s.reset} ${s.dim}(${RUST_OUT_DIR})...${s.reset}`,
  );
  const buildCmd = isDebug
    ? 'cargo build --features node'
    : 'cargo build --release --features node';
  execSync(buildCmd, {
    cwd: join(process.cwd()),
    stdio: STDIO_MODE,
  });
  logger.success(`${s.bold}Rust build success!${s.reset}`);

  logger.info(
    `${s.bold}Building JS project${s.reset} ${s.dim}(npm run build)...${s.reset}`,
  );
  execSync('npm run build', {
    stdio: STDIO_MODE,
  });
  logger.success(`${s.bold}JS build success!${s.reset}`);

  if (isLocal) {
    if (!fs.existsSync(DEST_DIR)) fs.mkdirSync(DEST_DIR);
    if (fs.existsSync(SOURCE_PATH)) {
      fs.copyFileSync(SOURCE_PATH, DEST_PATH);
      logger.step(
        `${s.dim}Binary linked to:${s.reset} ${s.bold}./binaries/${TARGET_NAME}${s.reset}\n`,
      );
    } else {
      throw new Error(`Binary not found at ${SOURCE_PATH}`);
    }

    modifyPackageJson('local');

    logger.info(
      `${s.bold}Packing local tarball${s.reset} ${s.dim}(with binaries as lightvm-test)...${s.reset}`,
    );
    execSync('npm pack', {
      stdio: 'inherit',
    });
    console.log('');

    modifyPackageJson('restore');
    logger.step(`${s.dim}Package.json restored to original.${s.reset}\n`);

    cleanup();
  } else if (isProduction) {
    modifyPackageJson('restore');

    if (fs.existsSync(DEST_PATH)) {
      fs.unlinkSync(DEST_PATH);
      logger.step(
        `${s.dim}Cleaned local binaries for production safety.${s.reset}`,
      );
    }

    if (isPublish) {
      logger.info(
        `${s.bold}${s.cyan}🚀 Publishing to NPM Registry...${s.reset}`,
      );
      execSync('npm publish --access public', {
        stdio: 'inherit',
      });
      logger.success(`${s.bold}Successfully published to NPM!${s.reset}`);
    } else {
      logger.info(
        `${s.bold}Packing production tarball${s.reset} ${s.dim}(clean, no binaries)...${s.reset}`,
      );
      execSync('npm pack', {
        stdio: 'inherit',
      });
      logger.success(`${s.bold}Production package pack complete.${s.reset}`);
    }

    cleanup();
  } else {
    logger.warn(`${s.bold}No workflow mode specified.${s.reset}`);
    console.log(
      `  ${s.dim}Usage:${s.reset} ${s.bold}--local${s.reset}, ${s.bold}--production${s.reset}, ${s.bold}--production --publish${s.reset}, or append ${s.bold}--silent${s.reset}\n`,
    );
  }
} catch (err) {
  modifyPackageJson('restore');
  cleanup();
  logger.error('Build workflow failed:', err.message);
  process.exit(1);
}
