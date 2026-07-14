if [[ "$PLATFORM" == *"android"* ]]; then
  export TARGET_UPPER=$(echo $TARGET | tr '[:lower:]' '[:upper:]' | tr '-' '_')
  
  ARCH=${TARGET%%-*}

  export RUSTFLAGS="-L${ANDROID_NDK_LATEST_HOME}/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/${ARCH}-linux-android/ -Clinker=${ANDROID_NDK_LATEST_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/clang"
  
  cargo build --release --target $TARGET
  
elif [[ "$PLATFORM" == *"musl"* ]]; then
  if [ ! -f "./cross" ]; then
    curl -L https://github.com/cross-rs/cross/releases/latest/download/cross-x86_64-unknown-linux-musl.tar.gz | tar xz
    chmod +x cross
  fi
  RUSTFLAGS="-C target-feature=-crt-static" ./cross build --release --target $TARGET
else
  if [ ! -f "./cross" ]; then
    curl -L https://github.com/cross-rs/cross/releases/latest/download/cross-x86_64-unknown-linux-musl.tar.gz | tar xz
    chmod +x cross
  fi
  ./cross build --release --target $TARGET
fi
