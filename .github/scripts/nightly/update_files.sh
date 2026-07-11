VERSION="$VERSION_VAL"

sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

if [ -f "package.json" ]; then
  jq --arg ver "$VERSION" '.version = $ver' package.json > temp.json && mv temp.json package.json
fi

rustup toolchain install stable --profile minimal
cargo update