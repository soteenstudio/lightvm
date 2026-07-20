/**
 * Copyright 2026 SoTeen Studio
 * 
 * Master Build Script: All-in-One & Storage Friendly
 */

import fs from 'fs';
import { execSync } from 'child_process';
import { join } from 'path';
import os from 'os';

const args = process.argv.slice(2);
const isLocal = args.includes('--local');
const isDebug = !args.includes('--release'); 
const isProduction = args.includes('--production');
const isPublish = args.includes('--publish');
const isSilent = args.includes('--silent');

const STDIO_MODE = isSilent ? 'ignore' : 'inherit';
const PKG_PATH = join(process.cwd(), 'package.json');
const TARGET_NAME = 'lightvm.node';
const RUST_OUT_DIR = isDebug ? 'debug' : 'release';

const RAM_TARGET = './target';
process.env.CARGO_TARGET_DIR = RAM_TARGET;

let rustBinaryName = 'liblightvm.so';
if (os.platform() === 'win32') rustBinaryName = 'lightvm.dll';
else if (os.platform() === 'darwin') rustBinaryName = 'liblightvm.dylib';

const SOURCE_PATH = join(RAM_TARGET, RUST_OUT_DIR, rustBinaryName);
const DEST_DIR = join(process.cwd(), 'binaries');
const DEST_PATH = join(DEST_DIR, TARGET_NAME);

const s = { reset: '\x1b[0m', bold: '\x1b[1m', dim: '\x1b[2m', cyan: '\x1b[36m', green: '\x1b[32m', yellow: '\x1b[33m', red: '\x1b[31m' };

const logger = {
  info: (msg) => !isSilent && console.log(`${s.bold}${s.cyan}⠋${s.reset} ${msg}`),
  success: (msg) => !isSilent && console.log(`${s.bold}${s.green}✔${s.reset} ${msg}`),
  step: (msg) => !isSilent && console.log(`${s.bold}${s.cyan}ℹ${s.reset} ${msg}`),
  error: (msg, detail) => console.error(`\n${s.bold}${s.red}𐄂 ${msg}${s.reset}`, detail || ''),
};

let originalPkgContent = null;
function modifyPackageJson(mode) {
  if (mode === 'local') {
    originalPkgContent = fs.readFileSync(PKG_PATH, 'utf-8');
    const pkg = JSON.parse(originalPkgContent);
    pkg.name = 'lightvm-test';
    pkg.files = pkg.files || [];
    if (!pkg.files.includes('binaries')) pkg.files.push('binaries');
    fs.writeFileSync(PKG_PATH, JSON.stringify(pkg, null, 2));
  } else if (originalPkgContent) {
    fs.writeFileSync(PKG_PATH, originalPkgContent);
  }
}

try {
  if (!fs.existsSync(PKG_PATH + '.bak')) fs.copyFileSync(PKG_PATH, PKG_PATH + '.bak');

  logger.info(`Building Rust (${RUST_OUT_DIR}) in RAM...`);
  execSync(isDebug ? 'cargo build --features node' : 'cargo build --release --features node', { stdio: STDIO_MODE });

  logger.info(`Building JS...`);
  execSync('npm run build', { stdio: STDIO_MODE });

  // --- Implementasi yang lebih kompatibel dengan Termux ---
  if (!fs.existsSync(DEST_DIR)) fs.mkdirSync(DEST_DIR);
  
  if (fs.existsSync(DEST_PATH)) fs.unlinkSync(DEST_PATH);
  
  try {
    // Tetap coba hard link dulu (paling efisien)
    fs.linkSync(SOURCE_PATH, DEST_PATH);
    logger.success(`Binary created via hard link.`);
  } catch (e) {
    // FALLBACK: Kalau hard link ditolak (EACCES/EXDEV), pakai copy.
    // Ini tetap 'proper' karena cuma dilakukan 1x per build.
    fs.copyFileSync(SOURCE_PATH, DEST_PATH);
    logger.success(`Hard link failed, fallback to copy.`);
  }
  // -------------------------------------------------------

  if (isLocal || (isProduction && !isPublish)) {
    modifyPackageJson('local');
    execSync('npm pack', { stdio: 'inherit' });
    modifyPackageJson('restore');
  } else if (isProduction) {
    if (isPublish) execSync('npm publish --access public', { stdio: 'inherit' });
    else execSync('npm pack', { stdio: 'inherit' });
  }

  logger.step('Build complete!');
} catch (err) {
  logger.error('Workflow failed:', err.message);
  process.exit(1);
}
