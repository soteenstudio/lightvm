export PATH="$TOOLCHAIN:$PATH"
export LD="$TOOLCHAIN/ld.lld"

echo "ANDROID_NDK_HOME=$ANDROID_NDK_LATEST_HOME" >> $GITHUB_ENV

TOOLCHAIN="$ANDROID_NDK_LATEST_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin"

mkdir -p .cargo
if [ "${{ matrix.platform }}" == "android-arm64" ]; then
  LINKER="aarch64-linux-android33-clang"
  echo "CC=$TOOLCHAIN/aarch64-linux-android33-clang" >> $GITHUB_ENV
else
  LINKER="armv7a-linux-androideabi33-clang"
  echo "CC=$TOOLCHAIN/armv7a-linux-androideabi33-clang" >> $GITHUB_ENV
fi

echo "AR=$TOOLCHAIN/llvm-ar" >> $GITHUB_ENV

echo "[target.${{ matrix.target }}]" > .cargo/config.toml
echo "ar = \"$TOOLCHAIN/llvm-ar\"" >> .cargo/config.toml
echo "linker = \"$TOOLCHAIN/$LINKER\"" >> .cargo/config.toml
