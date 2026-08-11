VERSION="$INPUT_VERSION"
if [ -z "$VERSION" ]; then
  RAW_VERSION="$INPUT_RAW_VERSION"
  VERSION=${RAW_VERSION#v}
fi

if [ -z "$VERSION" ]; then
  echo "Fetch latest release tag from GitHub API..."
  LATEST_TAG=$(gh release view --json tagName --template '{{.tagName}}' 2>/dev/null || echo "")
  if [ -n "$LATEST_TAG" ]; then
    VERSION=${LATEST_TAG#v}
    echo "Successfully resolved latest release version: $VERSION"
  else
    echo "No release found via API. Falling back to Cargo.toml..."
    VERSION=$(grep '^version =' Cargo.toml | head -n1 | cut -d '"' -f2)
  fi
fi

echo "Final Publishing Version: $VERSION"

TAG="latest"
if [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+-nightly\.[0-9]{8}\.[[:alnum:]]+$ ]]; then
    TAG="nightly"
elif [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+-(proto|alpha|beta|rc)(\.|$) ]]; then
    TAG="next"
fi
echo "Using NPM Tag: $TAG"

PLATFORMS=(
  "binary-linux-x64|linux-x64|index.linux.node|x64"
  "binary-linux-arm64|linux-arm64|index.linux-arm64.node|arm64"
  "binary-linux-ia32|linux-ia32|index.linux32.node|ia32"
  "binary-win32-x64|win32-x64|index.win.node|x64"
  "binary-win32-ia32|win32-ia32|index.win32.node|ia32"
  "binary-darwin-x64|darwin-x64|index.darwin.node|x64"
  "binary-android-arm64|android-arm64|index.android.node|arm64"
  "binary-android-arm|android-arm|index.android32.node|arm"
  "binary-linux-musl-x64|linux-musl-x64|index.musl-x64.node|x64"
  "binary-linux-musl-arm64|linux-musl-arm64|index.musl-arm64.node|arm64"
  "binary-linux-musl-ia32|linux-musl-ia32|index.musl-ia32.node|ia32"
  "binary-freebsd-x64|freebsd-x64|index.freebsd.node|x64"
  "binary-browser-wasm|browser-wasm|index.wasm|wasm"
)

sudo apt-get install -y jq

# Array to track all temporary signing directories for cleanup
SIGN_TEMP_DIRS=()
trap 'rm -rf "${SIGN_TEMP_DIRS[@]}"' EXIT

for item in "${PLATFORMS[@]}"; do
  IFS="|" read -r ARTIFACT PLATFORM BIN_NAME CPU <<< "$item"
  PKG_NAME="@lightvm/core-$PLATFORM"
  OS_VAL="${PLATFORM%-*}"
  mkdir -p "publish/$PLATFORM"
  
  if [[ "$PLATFORM" == "browser-wasm" ]]; then
      echo "=== Contents of binaries/$ARTIFACT ==="
      ls -la "binaries/$ARTIFACT"
      
      cp -r "binaries/$ARTIFACT"/. "publish/$PLATFORM/"
      
      cd "publish/$PLATFORM"
      
      WASM_JS_FILE=$(find . -maxdepth 1 -name "*.js" ! -name "index.js" | head -n 1)
      if [ -n "$WASM_JS_FILE" ]; then
          mv "$WASM_JS_FILE" "index.js"
      fi
      
      WASM_BG_FILE=$(find . -maxdepth 1 -name "*_bg.wasm" | head -n 1)
      if [ -n "$WASM_BG_FILE" ]; then
          mv "$WASM_BG_FILE" "index.wasm"
      fi
      
      cd ../..
      
      MAIN_FIELD="index.js" 
      FILES_FIELD=$(jq -nc '["index.wasm", "index.js", "*.d.ts", "README.md", "LICENSE"]')
  else
    find "binaries/$ARTIFACT" -type f \( -name "*.node" -o -name "*.dll" -o -name "*.so" -o -name "*.dylib" \) -exec cp {} "publish/$PLATFORM/$BIN_NAME" \;

    # Sign the native binary
    BINARY_PATH="publish/$PLATFORM/$BIN_NAME"

    if [ ! -f "$BINARY_PATH" ]; then
      echo "ERROR: Binary file not found at $BINARY_PATH for signing"
      exit 1
    fi

    # Check if signing is required (for release/nightly builds)
    if [[ "$EVENT_NAME" == "release" ]] || [[ "$EVENT_NAME" == "workflow_dispatch" && "$VERSION" == *"nightly"* ]]; then
      # Production release: signing is REQUIRED
      if [ -z "$SIGNING_PRIVATE_KEY" ]; then
        echo "ERROR: SIGNING_PRIVATE_KEY is required for production releases but not set"
        exit 1
      fi
    fi

    if [ -n "$SIGNING_PRIVATE_KEY" ]; then
      SIG_PATH="${BINARY_PATH}.sig"

      # Capture absolute paths before changing directories
      REPO_DIR="$(pwd)"
      ABS_BINARY_PATH="$REPO_DIR/$BINARY_PATH"
      ABS_SIG_PATH="$REPO_DIR/$SIG_PATH"

      # Create a unique temporary directory for signing
      SIGN_TEMP_DIR=$(mktemp -d)
      SIGN_TEMP_DIRS+=("$SIGN_TEMP_DIR")

      # Create a temporary Rust script to sign the binary
      cat > "$SIGN_TEMP_DIR/sign_binary.rs" << 'RUST_EOF'
use ed25519_dalek::{Signer, SigningKey};
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
RUST_EOF

      # Compile and run the signing script
      cd "$SIGN_TEMP_DIR" || { echo "ERROR: Failed to change to signing temp directory"; exit 1; }
      cargo new --bin sign_temp --quiet
      cd sign_temp || { echo "ERROR: Failed to change to sign_temp directory"; exit 1; }
      echo 'ed25519-dalek = "2.1"' >> Cargo.toml
      echo 'hex = "0.4"' >> Cargo.toml
      cp "$SIGN_TEMP_DIR/sign_binary.rs" src/main.rs

      if ! cargo run --release --quiet -- "$SIGNING_PRIVATE_KEY" "$ABS_BINARY_PATH" "$ABS_SIG_PATH" 2>&1; then
        echo "ERROR: Failed to sign binary $BINARY_PATH"
        exit 1
      fi

      cd "$REPO_DIR" || { echo "ERROR: Failed to change back to repository directory"; exit 1; }

      if [ ! -f "$SIG_PATH" ]; then
        echo "ERROR: Signature file was not created at $SIG_PATH"
        exit 1
      fi

      echo "Binary signed successfully: $BINARY_PATH"
    else
      echo "WARNING: SIGNING_PRIVATE_KEY not set, skipping binary signature for $BIN_NAME (non-production build)"
    fi

    MAIN_FIELD="$BIN_NAME"
    if [ -n "$SIGNING_PRIVATE_KEY" ]; then
      FILES_FIELD=$(jq -nc --arg bin "$BIN_NAME" --arg sig "${BIN_NAME}.sig" '[$bin, $sig, "README.md", "LICENSE"]')
    else
      FILES_FIELD=$(jq -nc --arg bin "$BIN_NAME" '[$bin, "README.md", "LICENSE"]')
    fi
  fi
  
  cp -f README.md "publish/$PLATFORM/" || true
  cp -f LICENSE "publish/$PLATFORM/" || true
  
  jq -n \
    --arg name "$PKG_NAME" \
    --arg ver "$VERSION" \
    --arg os "$OS_VAL" \
    --arg cpu "$CPU" \
    --arg main "$MAIN_FIELD" \
    --argjson files "$FILES_FIELD" \
    '{
      name: $name,
      version: $ver,
      os: [$os],
      cpu: [$cpu],
      main: $main,
      files: $files,
      publishConfig: { access: "public" },
      license: "Apache-2.0"
    }' > "publish/$PLATFORM/package.json"

  cd "publish/$PLATFORM"
  
  if [[ "$EVENT_NAME" == "release" ]] || [[ "$EVENT_NAME" == "workflow_dispatch" && "$VERSION" == *"nightly"* ]]; then
    echo "Nightly Release Event Detected: Running Real Publish to NPM..."
    npm publish --tag $TAG --access public || echo "Skip existing"
  else
    echo "Manual (Non-Nightly) Event Detected: Running Dry-Run (NPM Pack)..."
    npm pack
    mkdir -p ../../dist-test
    mv *.tgz ../../dist-test/
  fi

  cd ../..
done

if [[ "$EVENT_NAME" == "release" || "$EVENT_NAME" == "workflow_dispatch" ]]; then
  jq --arg ver "$VERSION" \
    '.version = $ver | .optionalDependencies = {
      "@lightvm/core-linux-x64": $ver,
      "@lightvm/core-linux-arm64": $ver,
      "@lightvm/core-linux-ia32": $ver,
      "@lightvm/core-win32-x64": $ver,
      "@lightvm/core-win32-ia32": $ver,
      "@lightvm/core-darwin-x64": $ver,
      "@lightvm/core-android-arm64": $ver,
      "@lightvm/core-android-arm": $ver,
      "@lightvm/core-linux-musl-x64": $ver,
      "@lightvm/core-linux-musl-arm64": $ver,
      "@lightvm/core-linux-musl-ia32": $ver,
      "@lightvm/core-freebsd-x64": $ver,
      "@lightvm/core-browser-wasm": $ver
    }' package.json > temp.json && mv temp.json package.json

  npm publish --access public --tag $TAG
fi