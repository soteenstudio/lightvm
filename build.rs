use ed25519_dalek::{Signer, SigningKey};
use std::env;
use std::fs;

fn main() {
  println!("cargo:rerun-if-changed=src/lib.rs");

  let Ok(secret_hex) = env::var("SIGNING_PRIVATE_KEY") else {
    println!("cargo:warning=SIGNING_PRIVATE_KEY not found, skipping binary signature.");
    return;
  };

  let secret_bytes = match hex::decode(secret_hex) {
    Ok(bytes) => bytes,
    Err(_) => {
      println!("cargo:warning=Invalid hex secret format!");
      return;
    }
  };

  let secret_array: [u8; 32] = match secret_bytes.try_into() {
    Ok(arr) => arr,
    Err(_) => {
      println!("cargo:warning=Private key length must be 32 bytes!");
      return;
    }
  };

  let signing_key = SigningKey::from_bytes(&secret_array);

  let target_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

  let binary_path = format!("{}/target/release/nama_library_lu.node", target_dir);

  if let Ok(binary_data) = fs::read(&binary_path) {
    let signature = signing_key.sign(&binary_data);

    let sig_output_path = format!("{}/nama_library_lu.node.sig", target_dir);
    if fs::write(&sig_output_path, signature.to_bytes()).is_ok() {
      println!("🚀 napi-rs .node file successfully signed automatically in CI/CD!");
    } else {
      println!("cargo:warning=Failed to write .sig file!");
    }
  } else {
    println!("cargo:warning=.node binary file not found in release path, skipping signing.");
  }
}
