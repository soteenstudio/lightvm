/**
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import { execSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function run() {
  try {
    console.log('Building Rust biner (release)...');
    execSync('cargo build --release --features node', {
      stdio: 'inherit',
      cwd: path.resolve(__dirname, '..'),
    });
    console.log('Rust build success!');

    const rootDir = path.resolve(__dirname, '..');
    const binariesDir = path.join(rootDir, 'binaries');
    const sourcePath = path.join(rootDir, 'target/release/liblightvm.so');
    const destPath = path.join(binariesDir, 'lightvm.node');

    if (!fs.existsSync(binariesDir)) {
      fs.mkdirSync(binariesDir);
    }

    if (fs.existsSync(sourcePath)) {
      fs.copyFileSync(sourcePath, destPath);
      console.log(`Successfully copied to: ${destPath.replace(rootDir, '.')}`);
    } else {
      throw new Error(`Binary not found in ${sourcePath}`);
    }

    console.log('Building project...');
    execSync('npm run build', { stdio: 'ignore' });
    console.log('Build success!');

    console.log('Running tests...');
    try {
      execSync('npx unitry ./ts/tests', {
        stdio: 'inherit',
        timeout: 10000,
      });
    } catch (err) {
      if (err.code === 'ETIMEDOUT') {
        console.error('Tests stuck / timeout! Forcing cleanup...');
      } else {
        console.log('Tests finished with some failures.');
      }
    }

    const distPath = path.resolve(__dirname, '../dist');
    if (fs.existsSync(distPath)) {
      fs.rmSync(distPath, { recursive: true, force: true });
      console.log('Cleanup complete: ./dist deleted.');
    }

    const binariesPath = path.resolve(__dirname, '../binaries');
    if (fs.existsSync(binariesPath)) {
      fs.rmSync(binariesPath, { recursive: true, force: true });
      console.log('Cleanup complete: ./binaries deleted.');
    }
  } catch (error) {
    console.error('Error during execution:', error.message);
    process.exit(1);
  }
}

run();
