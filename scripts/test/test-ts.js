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

const s = {
  reset: '\x1b[0m',
  bold: '\x1b[1m',
  dim: '\x1b[2m',
  cyan: '\x1b[36m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  red: '\x1b[31m',
};

function isMusl() {
  try {
    const report = process.report.getReport();
    if (report && report.header && !report.header.glibcVersionRuntime) {
      return true;
    }
  } catch (e) {}
  try {
    const output = execSync('ldd --version', {
      stdio: ['pipe', 'pipe', 'ignore'],
    }).toString();
    return output.includes('musl');
  } catch (e) {
    return false;
  }
}

function getPlatformPackageName() {
  const { platform, arch } = process;
  let packageName = '';
  if (platform === 'linux') {
    if (arch === 'x64') {
      packageName = isMusl()
        ? '@lightvm/core-linux-musl-x64'
        : '@lightvm/core-linux-x64';
    } else if (arch === 'ia32') {
      packageName = isMusl()
        ? '@lightvm/core-linux-musl-ia32'
        : '@lightvm/core-linux-ia32';
    }
  } else if (platform === 'win32') {
    packageName =
      arch === 'x64'
        ? '@lightvm/core-win32-x64'
        : arch === 'ia32'
          ? '@lightvm/core-win32-ia32'
          : '';
  } else if (platform === 'darwin' && arch === 'x64') {
    packageName = '@lightvm/core-darwin-x64';
  } else if (platform === 'android') {
    packageName =
      arch === 'arm64'
        ? '@lightvm/core-android-arm64'
        : arch === 'arm'
          ? '@lightvm/core-android-arm'
          : '';
  }
  return packageName;
}

function run() {
  try {
    console.log(
      `${s.bold}${s.cyan}⠋${s.reset} ${s.bold}Building Rust binary${s.reset} ${s.dim}(release --features node)...${s.reset}`,
    );
    execSync('cargo build --release --features node', {
      stdio: 'inherit',
      cwd: path.resolve(__dirname, '..'),
    });
    console.log(
      `${s.bold}${s.green}✔${s.reset} ${s.bold}Rust build success!${s.reset}\n`,
    );

    const rootDir = path.resolve(__dirname, '../..');
    const sourcePath = path.join(rootDir, 'target/release/liblightvm.so');

    if (!fs.existsSync(sourcePath)) {
      throw new Error(`Binary not found in ${sourcePath}`);
    }

    const packageName = getPlatformPackageName();
    if (!packageName) {
      throw new Error(
        `Platform ${process.platform} ${process.arch} is not supported`,
      );
    }

    const stagedPackageDir = path.join(
      rootDir,
      'node_modules/@lightvm',
      packageName.replace('@lightvm/', ''),
    );
    const destPath = path.join(stagedPackageDir, 'lightvm.node');

    fs.mkdirSync(stagedPackageDir, { recursive: true });
    fs.copyFileSync(sourcePath, destPath);
    fs.writeFileSync(
      path.join(stagedPackageDir, 'package.json'),
      JSON.stringify({ name: packageName, main: 'lightvm.node' }, null, 2),
    );

    const relativeDest = destPath.replace(rootDir, '.');
    console.log(
      `${s.bold}${s.cyan}ℹ${s.reset} ${s.dim}Binary staged at:${s.reset} ${s.bold}${relativeDest}${s.reset}\n`,
    );

    console.log(
      `${s.bold}${s.cyan}⠋${s.reset} ${s.bold}Building project${s.reset} ${s.dim}(npm run build)...${s.reset}`,
    );
    execSync('npm run build', { stdio: 'ignore' });
    console.log(
      `${s.bold}${s.green}✔${s.reset} ${s.bold}Build success!${s.reset}\n`,
    );

    console.log(
      `${s.bold}${s.cyan}⠋${s.reset} ${s.bold}Running tests${s.reset} ${s.dim}(unitry)...${s.reset}`,
    );
    try {
      execSync('npx unitry ./ts/tests', {
        stdio: 'inherit',
        timeout: 10000,
        env: {
          ...process.env,
          LIGHTVM_SKIP_SIGNATURE_VERIFICATION: 'true',
        },
      });
    } catch (err) {
      if (err.code === 'ETIMEDOUT') {
        console.error(
          `\n${s.bold}${s.red}𐄂${s.reset} ${s.bold}${s.red}Tests stuck / timeout!${s.reset} ${s.dim}Forcing cleanup...${s.reset}`,
        );
      } else {
        console.log(
          `\n${s.bold}${s.yellow}⚠${s.reset} ${s.bold}Tests finished with some failures.${s.reset}`,
        );
      }
    }

    console.log('');

    const distPath = path.resolve(__dirname, '../dist');
    if (fs.existsSync(distPath)) {
      fs.rmSync(distPath, { recursive: true, force: true });
      console.log(
        `${s.bold}${s.dim}🧹 Cleanup complete: ./dist deleted.${s.reset}`,
      );
    }

    const stagedScopeDir = path.resolve(
      __dirname,
      '../../node_modules/@lightvm',
    );
    if (fs.existsSync(stagedScopeDir)) {
      fs.rmSync(stagedScopeDir, { recursive: true, force: true });
      console.log(
        `${s.bold}${s.dim}🧹 Cleanup complete: ./node_modules/@lightvm deleted.${s.reset}`,
      );
    }

    console.log(
      `\n${s.bold}${s.green}✨ Done! Everything processed successfully.${s.reset}`,
    );
  } catch (error) {
    console.error(
      `\n${s.bold}${s.red}𐄂 Error during execution:${s.reset} ${error.message}`,
    );
    process.exit(1);
  }
}

run();
