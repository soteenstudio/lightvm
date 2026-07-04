if [[ "${{ matrix.platform }}" == *"android"* ]]; then
  cargo build --release --target ${{ matrix.target }}
elif [[ "${{ matrix.platform }}" == *"musl"* ]]; then
  if [ ! -f "./cross" ]; then
    curl -L https://github.com/cross-rs/cross/releases/latest/download/cross-x86_64-unknown-linux-musl.tar.gz | tar xz
    chmod +x cross
  fi
  RUSTFLAGS="-C target-feature=-crt-static" ./cross build --release --target ${{ matrix.target }}
else
  if [ ! -f "./cross" ]; then
    curl -L https://github.com/cross-rs/cross/releases/latest/download/cross-x86_64-unknown-linux-musl.tar.gz | tar xz
    chmod +x cross
  fi
  ./cross build --release --target ${{ matrix.target }}
fi
