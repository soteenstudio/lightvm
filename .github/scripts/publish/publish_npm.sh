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
if [[ "$VERSION" =~ -(proto|alpha|beta|rc) ]]; then TAG="next"; fi
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
      MAIN_FIELD="$BIN_NAME"
      FILES_FIELD=$(jq -nc --arg bin "$BIN_NAME" '[$bin, "README.md", "LICENSE"]')
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