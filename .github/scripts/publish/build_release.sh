if [[ "$PLATFORM" == *"android"* ]]; then
  cargo ndk -t $TARGET -- build --release --features node

elif [[ "$PLATFORM" == *"musl"* ]]; then
  if [ ! -f "./cross" ]; then
    curl -L https://github.com/cross-rs/cross/releases/latest/download/cross-x86_64-unknown-linux-musl.tar.gz | tar xz
    chmod +x cross
  fi
  RUSTFLAGS="-C target-feature=-crt-static" ./cross build --release --target $TARGET  --features node
else
  if [ ! -f "./cross" ]; then
    curl -L https://github.com/cross-rs/cross/releases/latest/download/cross-x86_64-unknown-linux-musl.tar.gz | tar xz
    chmod +x cross
  fi
  ./cross build --release --target $TARGET  --features node
fi
