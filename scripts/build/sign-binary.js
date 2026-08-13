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

// Diubah agar mendukung pembacaan JWK JSON (field 'd')
const RUST_SIGNER_SOURCE = `use ed25519_dalek::{Signer, SigningKey};
use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 {
    eprintln!("Usage: sign_binary <binary_path> <sig_output_path>");
    std::process::exit(1);
  }

  let secret_json_str = env::var("LIGHTVM_SIGNING_KEY").expect("LIGHTVM_SIGNING_KEY environment variable is required");
  let binary_path = &args[1];
  let sig_output_path = &args[2];

  // Parse JSON JWK sederhana untuk mengambil field "d" (private key base64url)
  let json: serde_json::Value = serde_json::from_str(&secret_json_str).expect("Invalid JWK JSON format");
  let d_b64url = json["d"].as_str().expect("JWK missing 'd' parameter");

  // Decode base64url ke bytes
  let secret_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
      .decode(d_b64url)
      .expect("Failed to decode base64url secret");

  let secret_array: [u8; 32] = secret_bytes.try_into().expect("Private key must be 32 bytes");
  let signing_key = SigningKey::from_bytes(&secret_array);

  let binary_data = fs::read(binary_path).expect("Failed to read binary file");
  let signature = signing_key.sign(&binary_data);

  fs::write(sig_output_path, signature.to_bytes()).expect("Failed to write signature file");
  println!("Successfully signed: {}", binary_path);
}
`;

export function signBinary(privateKeyJwkString, binaryPath, sigOutputPath) {
  if (!privateKeyJwkString) {
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
    // Tambahkan dependency serde_json dan base64 untuk parsing JWK
    fs.appendFileSync(
      path.join(signProjectDir, 'Cargo.toml'),
      'ed25519-dalek = "2.1"\nserde_json = "1.0"\nbase64 = "0.22"\n',
    );
    fs.copyFileSync(
      path.join(signTempDir, 'sign_binary.rs'),
      path.join(signProjectDir, 'src', 'main.rs'),
    );

    const absBinaryPath = path.resolve(binaryPath);
    const absSigPath = path.resolve(sigOutputPath);

    execFileSync(
      'cargo',
      ['run', '--release', '--quiet', '--', absBinaryPath, absSigPath],
      {
        cwd: signProjectDir,
        stdio: 'inherit',
        env: { ...process.env, LIGHTVM_SIGNING_KEY: privateKeyJwkString },
      },
    );

    if (!fs.existsSync(sigOutputPath)) {
      throw new Error(`Signature file was not created at ${sigOutputPath}`);
    }
  } finally {
    fs.rmSync(signTempDir, { recursive: true, force: true });
  }
}
