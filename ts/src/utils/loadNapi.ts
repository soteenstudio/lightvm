/**
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

import { createRequire } from 'module';
import { existsSync, readFileSync } from 'fs';
import { createPublicKey, verify } from 'crypto';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';
import { VMSystemError } from './vmerror.js';
import { isMusl } from './isMusl.js';
const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
let cachedNative: any = null;

const PUBLIC_KEY_BYTES = new Uint8Array([
  16, 241, 151, 48, 19, 252, 107, 117, 224, 89, 203, 89, 162, 96, 43, 50, 13,
  24, 97, 169, 163, 224, 167, 57, 130, 253, 237, 62, 84, 166, 179, 96,
]);

/**
 * Verifies the Ed25519 signature of a native binary BEFORE loading it.
 * This is the external trust anchor - verification happens before any untrusted code executes.
 */
function verifyBinarySignature(
  binaryPath: string,
  explain: boolean,
  hint: boolean,
): void {
  try {
    const binaryData = readFileSync(binaryPath);

    const sigPath = `${binaryPath}.sig`;
    let sigData: Buffer;
    try {
      sigData = readFileSync(sigPath);
    } catch (err) {
      const error = new VMSystemError(
        `Signature file not found for ${binaryPath}`,
        [
          'The native binary is missing its cryptographic signature (.sig file). This indicates an incomplete installation or potential tampering.',
          'Please reinstall the package from a trusted source (npm registry).',
        ],
      );
      error.print(explain, hint);
      process.exit(70);
    }

    const publicKey = createPublicKey({
      key: {
        kty: 'OKP',
        crv: 'Ed25519',
        x: Buffer.from(PUBLIC_KEY_BYTES).toString('base64url'),
      },
      format: 'jwk',
    });

    const isValid = verify(null, binaryData, publicKey, sigData);

    if (!isValid) {
      const error = new VMSystemError(
        `Binary signature verification failed for ${binaryPath}`,
        [
          'The native binary cryptographic signature is invalid. This indicates the binary has been modified, corrupted, or is from an untrusted source.',
          'Please reinstall the package from a trusted source (npm registry). If the problem persists, report this as a security issue.',
        ],
      );
      error.print(explain, hint);
      process.exit(70);
    }
  } catch (err: any) {
    if (err.code === 'ENOENT' && err.path && err.path.endsWith('.sig')) {
      const error = new VMSystemError(
        `Signature file not found for ${binaryPath}`,
        [
          'The native binary is missing its cryptographic signature (.sig file). This indicates an incomplete installation or potential tampering.',
          'Please reinstall the package from a trusted source (npm registry).',
        ],
      );
      error.print(explain, hint);
      process.exit(70);
    } else if (err.code === 'ENOENT') {
      const error = new VMSystemError(
        `Binary file not found at ${binaryPath}`,
        [
          'The native binary file is missing. This indicates an incomplete installation.',
          'Please reinstall the package from a trusted source (npm registry).',
        ],
      );
      error.print(explain, hint);
      process.exit(70);
    } else {
      const error = new VMSystemError(
        `Unexpected error during signature verification: ${err.message}`,
        [
          'An unexpected error occurred while verifying the binary signature.',
          'Please reinstall the package and report this issue if it persists.',
        ],
      );
      error.print(explain, hint);
      process.exit(70);
    }
  }
}
export function loadNapi(explain: boolean, hint: boolean) {
  if (cachedNative) return cachedNative;

  // Allow skipping signature verification during local development/testing
  const skipVerification =
    process.env.LIGHTVM_SKIP_SIGNATURE_VERIFICATION === 'true';

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
  } else {
    const error = new VMSystemError(
      `Platform ${platform} ${arch} is not supported`,
      [
        'The LightVM engine has not been ported to your current environment; this occurs when the operating system or processor architecture is not included in our prebuilt binary distribution, requiring a custom build from source to enable compatibility.',
        'Check the official documentation for a list of supported platforms and architectures.',
      ],
    );
    error.print(explain, hint);
    process.exit(65);
  }

  // Validate packageName before attempting resolution
  if (!packageName) {
    const error = new VMSystemError(
      `Platform ${platform} ${arch} is not supported`,
      [
        'The LightVM engine has not been ported to your current environment; this occurs when the operating system or processor architecture is not included in our prebuilt binary distribution, requiring a custom build from source to enable compatibility.',
        'Check the official documentation for a list of supported platforms and architectures.',
      ],
    );
    error.print(explain, hint);
    process.exit(65);
  }

  // Local testing fallback: only used when the platform package cannot be
  // resolved at all. Once a package resolves, the fallback is never
  // consulted again, even if verification or loading of that package fails.
  const fallbackPath = join(__dirname, '../binaries/lightvm.node');
  let usedFallback = false;

  try {
    let binaryPath: string;
    try {
      binaryPath = require.resolve(packageName);
    } catch (resolveErr: any) {
      if (!existsSync(fallbackPath)) {
        const error = new VMSystemError(
          `Failed to resolve binary package ${packageName}`,
          [
            `The system failed to locate the native binary package '${packageName}', and no local 'lightvm-test' testing binary was found at '${fallbackPath}'. This indicates a failed package installation or a registry synchronization error.`,
            `Run 'npm install ${packageName}' to verify your installation.`,
          ],
        );
        error.print(explain, hint);
        process.exit(69);
      }

      usedFallback = true;
      binaryPath = fallbackPath;
    }

    // CRITICAL: Verify signature BEFORE loading the binary
    // Only verify .node files (native addons), not JavaScript files
    if (!skipVerification && binaryPath.endsWith('.node')) {
      verifyBinarySignature(binaryPath, explain, hint);
    }

    cachedNative = require(binaryPath);
    return cachedNative;
  } catch (err) {
    const error = usedFallback
      ? new VMSystemError(
          `Failed to load local 'lightvm-test' testing binary at ${fallbackPath}.`,
          [
            'The local lightvm-test fallback binary could not be loaded. This indicates the binary is missing, corrupted, or was not built for this platform.',
            "Rebuild the local package with 'npm run build:release' (or 'npm run build:debug') with SIGNING_PRIVATE_KEY set.",
          ],
        )
      : new VMSystemError(
          `Failed to load binary for ${packageName}. Please ensure a secure connection during installation.`,
          [
            '    The system failed to load the necessary N-API bridge because the prebuilt binary module for your specific platform could not be resolved; this usually indicates a failed package installation, a registry synchronization error, or a platform mismatch between the installed dependencies and your current environment.',
            `Run 'npm install ${packageName}' to verify your installation.`,
          ],
        );
    error.print(explain, hint);
    process.exit(69);
  }
}
