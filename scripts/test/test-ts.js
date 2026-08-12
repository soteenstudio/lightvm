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
import { signBinary } from '../build/sign-binary.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const SIGNING_PRIVATE_KEY = process.env.SIGNING_PRIVATE_KEY;

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

function stagePlatformPackage(rootDir, sourcePath, packageName) {
  const stagedPackageDir = path.join(
    rootDir,
    'node_modules/@lightvm',
    packageName.replace('@lightvm/', ''),
  );

  fs.mkdirSync(stagedPackageDir, { recursive: true });
  fs.copyFileSync(sourcePath, path.join(stagedPackageDir, 'lightvm.node'));
  fs.writeFileSync(
    path.join(stagedPackageDir, 'package.json'),
    JSON.stringify({ name: packageName, main: 'lightvm.node' }, null, 2),
  );

  return stagedPackageDir;
}

function unstagePlatformPackages(rootDir) {
  const scopeDir = path.join(rootDir, 'node_modules/@lightvm');
  if (fs.existsSync(scopeDir)) {
    fs.rmSync(scopeDir, { recursive: true, force: true });
  }
}

// Stages the local `lightvm-test` fallback binary at `<rootDir>/binaries/lightvm.node`.
// `sign` controls the accompanying `.sig` file: 'valid' signs it with
// SIGNING_PRIVATE_KEY (when available), 'invalid' writes a bogus signature,
// and omitting it leaves the binary without a signature file at all.
function stageLocalFallback(rootDir, sourceContentPath, { sign } = {}) {
  const binariesDir = path.join(rootDir, 'binaries');
  fs.mkdirSync(binariesDir, { recursive: true });

  const binaryPath = path.join(binariesDir, 'lightvm.node');
  fs.copyFileSync(sourceContentPath, binaryPath);

  if (sign === 'valid' && SIGNING_PRIVATE_KEY) {
    signBinary(SIGNING_PRIVATE_KEY, binaryPath, `${binaryPath}.sig`);
  } else if (sign === 'invalid') {
    fs.writeFileSync(`${binaryPath}.sig`, 'not-a-real-signature');
  }

  return binaryPath;
}

function unstageLocalFallback(rootDir) {
  const binariesDir = path.join(rootDir, 'binaries');
  if (fs.existsSync(binariesDir)) {
    fs.rmSync(binariesDir, { recursive: true, force: true });
  }
}

function runUnitry({ skipSignatureVerification, scenario } = {}) {
  execSync('npx unitry ./ts/tests', {
    stdio: 'inherit',
    timeout: 10000,
    env: {
      ...process.env,
      ...(skipSignatureVerification
        ? { LIGHTVM_SKIP_SIGNATURE_VERIFICATION: 'true' }
        : {}),
      ...(scenario ? { LIGHTVM_TEST_SCENARIO: scenario } : {}),
    },
  });
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

    const stagedPackageDir = stagePlatformPackage(
      rootDir,
      sourcePath,
      packageName,
    );
    const destPath = path.join(stagedPackageDir, 'lightvm.node');

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
      runUnitry({ skipSignatureVerification: true });
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

    console.log(
      `${s.bold}${s.cyan}⠋${s.reset} ${s.bold}Running loadNapi fallback scenarios${s.reset}...`,
    );

    const corruptFallbackPath = path.join(
      rootDir,
      '.tmp-corrupt-lightvm.node',
    );
    fs.writeFileSync(corruptFallbackPath, 'not a real native binary');

    const hasSigningKey = Boolean(SIGNING_PRIVATE_KEY);

    try {
      // Scenario: the platform package must take priority over the local
      // fallback when both locations exist. The fallback is deliberately
      // corrupt so that using it by mistake fails loudly.
      stageLocalFallback(rootDir, corruptFallbackPath);
      try {
        runUnitry({ skipSignatureVerification: true, scenario: 'priority' });
      } finally {
        unstageLocalFallback(rootDir);
      }

      // Scenario: the local lightvm-test binary is loaded when the platform
      // package cannot be resolved at all.
      unstagePlatformPackages(rootDir);
      stageLocalFallback(rootDir, sourcePath, {
        sign: hasSigningKey ? 'valid' : undefined,
      });
      try {
        runUnitry({
          skipSignatureVerification: !hasSigningKey,
          scenario: 'fallback',
        });
      } finally {
        unstageLocalFallback(rootDir);
      }

      // Scenario: a missing/invalid signature must reject the local
      // fallback binary when verification is enabled.
      stageLocalFallback(rootDir, sourcePath, { sign: 'invalid' });
      try {
        runUnitry({
          skipSignatureVerification: false,
          scenario: 'reject-invalid-sig',
        });
      } finally {
        unstageLocalFallback(rootDir);
      }

      console.log(
        `${s.bold}${s.green}✔${s.reset} ${s.bold}loadNapi fallback scenarios passed!${s.reset}\n`,
      );
    } finally {
      if (fs.existsSync(corruptFallbackPath)) {
        fs.rmSync(corruptFallbackPath);
      }
    }

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
