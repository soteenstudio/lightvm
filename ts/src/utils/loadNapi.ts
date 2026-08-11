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
import { join } from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';
import { readFileSync } from 'fs';
import { createPublicKey, verify } from 'crypto';
import { VMSystemError } from './vmerror.js';
import { isMusl } from './isMusl.js';
const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
let cachedNative: any = null;

// Ed25519 public key (matches public.key and rust/src/lib.rs PUBLIC_KEY_BYTES)
const PUBLIC_KEY_BYTES = new Uint8Array([
  16, 241, 151, 48, 19, 252, 107, 117, 224, 89, 203, 89, 162, 96, 43, 50, 13,
  24, 97, 169, 163, 224, 167, 57, 130, 253, 237, 62, 84, 166, 179, 96,
]);

/**
 * Verifies the Ed25519 signature of a native binary BEFORE loading it.
 * This is the external trust anchor - verification happens before any untrusted code executes.
 * Returns true if verification succeeds, false if it fails.
 */
function verifyBinarySignature(
  binaryPath: string,
  explain: boolean,
  hint: boolean,
): boolean {
  try {
    // Read the binary file
    const binaryData = readFileSync(binaryPath);

    // Read the signature file
    const sigPath = `${binaryPath}.sig`;
    let sigData: Buffer;
    try {
      sigData = readFileSync(sigPath);
    } catch (err) {
      return false;
    }

    // Create Ed25519 public key using Node's crypto module
    const publicKey = createPublicKey({
      key: {
        kty: 'OKP',
        crv: 'Ed25519',
        x: Buffer.from(PUBLIC_KEY_BYTES).toString('base64url'),
      },
      format: 'jwk',
    });

    // Verify the signature
    const isValid = verify(null, binaryData, publicKey, sigData);

    return isValid;
  } catch (err: any) {
    // Any error during verification is treated as a failed verification
    return false;
  }
}
export function loadNapi(explain: boolean, hint: boolean) {
  if (cachedNative) return cachedNative;

  // Allow skipping signature verification during local development/testing
  const skipVerification = process.env.LIGHTVM_SKIP_SIGNATURE_VERIFICATION === 'true';

  try {
    const localPath = join(__dirname, '../binaries/lightvm.node');
    // Try to verify and load local binary - non-fatal if it fails (for development)
    // Allow falling through to platform-package resolution if local binary is missing/unsigned
    if (skipVerification || verifyBinarySignature(localPath, explain, hint)) {
      cachedNative = require(localPath);
      return cachedNative;
    }
  } catch (err) {}
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
  try {
    // For platform-specific packages, the main field points directly to the .node file
    // We need to get the full path to verify it BEFORE loading
    let binaryPath: string;
    try {
      binaryPath = require.resolve(packageName);
    } catch (resolveErr: any) {
      // If we can't resolve the package, it's a fatal error - do not load unverified code
      const error = new VMSystemError(
        `Failed to resolve binary package ${packageName}`,
        [
          'The system failed to locate the native binary package. This indicates a failed package installation or a registry synchronization error.',
          `Run 'npm install ${packageName}' to verify your installation.`,
        ],
      );
      error.print(explain, hint);
      process.exit(69);
    }

    // CRITICAL: Verify signature BEFORE loading the binary
    // Only verify .node files (native addons), not JavaScript files
    if (binaryPath.endsWith('.node') && !skipVerification) {
      if (!verifyBinarySignature(binaryPath, explain, hint)) {
        const error = new VMSystemError(
          `Binary signature verification failed for ${binaryPath}`,
          [
            'The native binary cryptographic signature is invalid or missing. This indicates the binary has been modified, corrupted, or is from an untrusted source.',
            'Please reinstall the package from a trusted source (npm registry). If the problem persists, report this as a security issue.',
          ],
        );
        error.print(explain, hint);
        process.exit(70);
      }
    }

    // Only after verification passes (or if it's not a .node file), load the module
    cachedNative = require(packageName);
    return cachedNative;
  } catch (err) {
    const error = new VMSystemError(
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
