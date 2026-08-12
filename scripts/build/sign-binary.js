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
import os from 'os';
import path from 'path';
import { execFileSync } from 'child_process';

// Mirrors the Ed25519 signing logic used in
// .github/scripts/publish/publish_npm.sh so locally built binaries are
// signed with the same algorithm and SIGNING_PRIVATE_KEY hex format.
const RUST_SIGNER_SOURCE = `use ed25519_dalek::{Signer, SigningKey};
use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 4 {
    eprintln!("Usage: sign_binary <private_key_hex> <binary_path> <sig_output_path>");
    std::process::exit(1);
  }

  let secret_hex = &args[1];
  let binary_path = &args[2];
  let sig_output_path = &args[3];

  let secret_bytes = hex::decode(secret_hex).expect("Invalid hex secret format");
  let secret_array: [u8; 32] = secret_bytes.try_into().expect("Private key must be 32 bytes");
  let signing_key = SigningKey::from_bytes(&secret_array);

  let binary_data = fs::read(binary_path).expect("Failed to read binary file");
  let signature = signing_key.sign(&binary_data);

  fs::write(sig_output_path, signature.to_bytes()).expect("Failed to write signature file");
  println!("Successfully signed: {}", binary_path);
}
`;

/**
 * Signs `binaryPath` with the Ed25519 `privateKeyHex` secret, writing the
 * raw 64-byte signature to `sigOutputPath`.
 */
export function signBinary(privateKeyHex, binaryPath, sigOutputPath) {
  if (!privateKeyHex) {
    throw new Error('SIGNING_PRIVATE_KEY is required to sign a binary');
  }

  if (!fs.existsSync(binaryPath)) {
    throw new Error(`Binary file not found at ${binaryPath} for signing`);
  }

  const signTempDir = fs.mkdtempSync(
    path.join(os.tmpdir(), 'lightvm-sign-'),
  );

  try {
    fs.writeFileSync(
      path.join(signTempDir, 'sign_binary.rs'),
      RUST_SIGNER_SOURCE,
    );

    execFileSync('cargo', ['new', '--bin', 'sign_temp', '--quiet'], {
      cwd: signTempDir,
      stdio: 'ignore',
    });

    const signProjectDir = path.join(signTempDir, 'sign_temp');
    fs.appendFileSync(
      path.join(signProjectDir, 'Cargo.toml'),
      'ed25519-dalek = "2.1"\nhex = "0.4"\n',
    );
    fs.copyFileSync(
      path.join(signTempDir, 'sign_binary.rs'),
      path.join(signProjectDir, 'src', 'main.rs'),
    );

    const absBinaryPath = path.resolve(binaryPath);
    const absSigPath = path.resolve(sigOutputPath);

    execFileSync(
      'cargo',
      ['run', '--release', '--quiet', '--', privateKeyHex, absBinaryPath, absSigPath],
      { cwd: signProjectDir, stdio: 'inherit' },
    );

    if (!fs.existsSync(sigOutputPath)) {
      throw new Error(`Signature file was not created at ${sigOutputPath}`);
    }
  } finally {
    fs.rmSync(signTempDir, { recursive: true, force: true });
  }
}