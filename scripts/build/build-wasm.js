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
const isSilent = args.includes('--silent');
const STDIO_MODE = isSilent ? 'ignore' : 'inherit';

const PKG_NAME = '@lightvm/core-browser-wasm';
const OUTPUT_DIR = join(process.cwd(), 'dist-test');
const STAGING_DIR = join(process.cwd(), 'publish', 'browser-wasm');
const WASM_PACK_PKG_DIR = join(process.cwd(), 'pkg');

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
  cleanup: (msg) =>
    !isSilent && console.log(`${s.bold}${s.dim}🧹 ${msg}${s.reset}`),
  error: (msg, detail) =>
    console.error(`\n${s.bold}${s.red}𐄂 ${msg}${s.reset}`, detail || ''),
};

function ensureCleanDir(dir) {
  if (fs.existsSync(dir)) {
    fs.rmSync(dir, { recursive: true, force: true });
  }
  fs.mkdirSync(dir, { recursive: true });
}

try {
  logger.info('Fetching version from Cargo.toml...');
  const cargoContent = fs.readFileSync(
    join(process.cwd(), 'Cargo.toml'),
    'utf-8',
  );
  const versionMatch = cargoContent.match(/^version\s*=\s*"([^"]+)"/m);
  const VERSION = versionMatch ? versionMatch[1] : '0.1.0-alpha.local';
  logger.info(`Resolved Version: ${s.bold}${s.yellow}${VERSION}${s.reset}`);

  logger.info(`${s.bold}Building Rust WASM via wasm-pack...${s.reset}`);
  execSync('wasm-pack build --target web --release -- --features wasm', {
    cwd: process.cwd(),
    stdio: STDIO_MODE,
  });
  logger.success(`${s.bold}wasm-pack build success!${s.reset}`);

  logger.info('Preparing staging directory...');
  ensureCleanDir(STAGING_DIR);
  if (!fs.existsSync(OUTPUT_DIR)) fs.mkdirSync(OUTPUT_DIR, { recursive: true });

  if (fs.existsSync(WASM_PACK_PKG_DIR)) {
    fs.cpSync(WASM_PACK_PKG_DIR, STAGING_DIR, { recursive: true });
    logger.info('Assets copied to staging workspace.');
  } else {
    throw new Error(
      `wasm-pack target directory not found at: ${WASM_PACK_PKG_DIR}`,
    );
  }

  logger.info('Detecting artifact filenames...');
  const files = fs.readdirSync(STAGING_DIR);

  const realJsFile = files.find(
    (f) => f.endsWith('.js') && !f.includes('esbuild') && f !== 'index.js',
  );
  if (!realJsFile) {
    throw new Error(
      'Could not find the generated JS entry point file from wasm-pack.',
    );
  }
  logger.info(`Detected JS Entry: ${s.dim}${realJsFile}${s.reset}`);

  if (fs.existsSync('README.md'))
    fs.copyFileSync('README.md', join(STAGING_DIR, 'README.md'));
  if (fs.existsSync('LICENSE'))
    fs.copyFileSync('LICENSE', join(STAGING_DIR, 'LICENSE'));

  logger.info('Generating package.json boilerplate...');
  const packageJsonContent = {
    name: PKG_NAME,
    version: VERSION,
    main: realJsFile,
    files: ['*.wasm', '*.js', '*.d.ts', 'README.md', 'LICENSE'],
    publishConfig: { access: 'public' },
    license: 'Apache-2.0',
  };

  fs.writeFileSync(
    join(STAGING_DIR, 'package.json'),
    JSON.stringify(packageJsonContent, null, 2),
  );

  logger.info(
    `${s.bold}Packing workspace into tarball via npm pack...${s.reset}`,
  );
  execSync('npm pack', { cwd: STAGING_DIR, stdio: STDIO_MODE });

  const stagedFiles = fs.readdirSync(STAGING_DIR);
  const tarballName = stagedFiles.find((f) => f.endsWith('.tgz'));

  if (tarballName) {
    const finalDest = join(OUTPUT_DIR, tarballName);
    fs.renameSync(join(STAGING_DIR, tarballName), finalDest);
    logger.success(
      `${s.bold}WASM Tarball successfully packed to:${s.reset} ${s.bold}${s.green}./dist-test/${tarballName}${s.reset}`,
    );
  } else {
    throw new Error('NPM pack failed to generate a .tgz file.');
  }

  ensureCleanDir(join(process.cwd(), 'publish'));
  logger.cleanup('Staging directory cleaned.');
} catch (err) {
  logger.error('WASM Build script compilation failed:', err.message);
  process.exit(1);
}
