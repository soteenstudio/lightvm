/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#[doc(hidden)]
pub(crate) mod codegen;
#[doc(hidden)]
#[allow(clippy::type_complexity)]
pub(crate) mod instructions;
#[doc(hidden)]
pub(crate) mod interfaces;
#[doc(hidden)]
pub(crate) mod modules;
#[doc(hidden)]
pub(crate) mod traits;
#[doc(hidden)]
pub mod types;
#[doc(hidden)]
pub(crate) mod utils;
#[doc(hidden)]
pub(crate) mod vm;
#[doc(hidden)]
#[cfg(feature = "node")]
pub use crate::interfaces::napi_interface::NodeLightVM;
#[doc(hidden)]
#[cfg(feature = "wasm")]
pub use crate::interfaces::wasm_interface::WasmLightVM;
#[cfg(feature = "node")]
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
#[cfg(not(feature = "node"))]
pub use interfaces::interface::LightVM;
#[cfg(feature = "node")]
use napi_derive::napi;
#[cfg(feature = "node")]
use std::fs;
pub use types::value::RunOptions;
#[cfg(feature = "node")]
const PUBLIC_KEY_BYTES: [u8; 32] = [
  16, 241, 151, 48, 19, 252, 107, 117, 224, 89, 203, 89, 162, 96, 43, 50, 13, 24, 97, 169, 163,
  224, 167, 57, 130, 253, 237, 62, 84, 166, 179, 96,
];
#[cfg(feature = "node")]
#[napi]
pub fn verify_binary_integrity(file_path: String) -> napi::Result<()> {
  let verifying_key = VerifyingKey::from_bytes(&PUBLIC_KEY_BYTES)
    .map_err(|_| napi::Error::from_reason("Public key is invalid!"))?;
  let binary_data = fs::read(&file_path)
    .map_err(|_| napi::Error::from_reason("Failed to read own .node library file!"))?;
  let sig_path = format!("{}.sig", file_path);
  let sig_bytes = fs::read(&sig_path)
    .map_err(|_| napi::Error::from_reason("Signature file (.sig) not found! Unofficial binary."))?;
  let signature = Signature::from_bytes(
    sig_bytes
      .as_slice()
      .try_into()
      .map_err(|_| napi::Error::from_reason("Signature format is corrupted!"))?,
  );
  verifying_key.verify(&binary_data, &signature).map_err(|_| {
    napi::Error::from_reason("CRITICAL WARNING: The .node file has been modified or forged!")
  })
}
