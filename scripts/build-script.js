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
  console.log('Cleaning up build directories...');

  if (fs.existsSync(DIST_DIR)) {
    fs.rmSync(DIST_DIR, { recursive: true, force: true });
    console.log('- Removed dist/');
  }

  if (fs.existsSync(DEST_DIR)) {
    fs.rmSync(DEST_DIR, { recursive: true, force: true });
    console.log('- Removed binaries/');
  }
}

try {
  console.log(`Building Rust (${RUST_OUT_DIR})...`);
  const buildCmd = isDebug
    ? 'cargo build --features node'
    : 'cargo build --release --features node';
  execSync(buildCmd, {
    cwd: join(process.cwd()),
    stdio: STDIO_MODE,
  });

  console.log('Building JS...');
  execSync('npm run build', {
    stdio: STDIO_MODE,
  });

  if (isLocal) {
    if (!fs.existsSync(DEST_DIR)) fs.mkdirSync(DEST_DIR);
    if (fs.existsSync(SOURCE_PATH)) {
      fs.copyFileSync(SOURCE_PATH, DEST_PATH);
      console.log(`Copied binary to ${DEST_PATH}`);
    } else {
      throw new Error(`Binary not found at ${SOURCE_PATH}`);
    }

    modifyPackageJson('local');

    console.log('Packing local tarball (with binaries as lightvm-test)...');
    execSync('npm pack', {
      stdio: 'inherit',
    });

    modifyPackageJson('restore');
    console.log('Package.json restored to original.');

    cleanup();
  } else if (isProduction) {
    modifyPackageJson('restore');

    if (fs.existsSync(DEST_PATH)) {
      fs.unlinkSync(DEST_PATH);
      console.log('Cleaned local binaries for production safety.');
    }

    if (isPublish) {
      console.log('Publishing to NPM Registry...');
      execSync('npm publish --access public', {
        stdio: 'inherit',
      });
    } else {
      console.log('Packing production tarball (clean, no binaries)...');
      execSync('npm pack', {
        stdio: 'inherit',
      });
      console.log('Production build complete.');
    }

    cleanup();
  } else {
    console.log(
      'Usage: --local, --production, --production --publish, or add --silent',
    );
  }
} catch (err) {
  modifyPackageJson('restore');

  cleanup();

  console.error('Build failed:', err.message);
  process.exit(1);
}
