/**
 * Copyright 2026 SoTeen Studio
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
import { VMSystemError } from './vmerror.js';
import { isMusl } from './isMusl.js';
const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
let cachedNative: any = null;
export function loadNapi(explain: boolean, hint: boolean) {
  if (cachedNative) return cachedNative;
  try {
    const localPath = join(__dirname, '../binaries/lightvm.node');
    cachedNative = require(localPath);
    return cachedNative;
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
        "The LightVM engine has not been ported to your current environment; this occurs when the operating system or processor architecture is not included in our prebuilt binary distribution, requiring a custom build from source to enable compatibility.",
        "Check the official documentation for a list of supported platforms and architectures."
      ]
    );
    error.print(explain, hint);
    process.exit(65);
  }
  try {
    cachedNative = require(packageName);
    return cachedNative;
  } catch (err) {
    const error = new VMSystemError(
      `Failed to load binary for ${packageName}. Please ensure a secure connection during installation.`,
      [
        "    The system failed to load the necessary N-API bridge because the prebuilt binary module for your specific platform could not be resolved; this usually indicates a failed package installation, a registry synchronization error, or a platform mismatch between the installed dependencies and your current environment.",
        `Run 'npm install ${packageName}' to verify your installation.`
      ]
    );
    error.print(explain, hint);
    process.exit(69);
  }
}
