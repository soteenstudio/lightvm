#!/usr/bin/env bash
set -euo pipefail

VERSION="${INPUT_VERSION:-}"

if [ -z "$VERSION" ]; then
  RAW_VERSION="${INPUT_RAW_VERSION:-}"
  VERSION="${RAW_VERSION#v}"
fi

if [ -z "$VERSION" ]; then
  VERSION=$(grep '^version =' Cargo.toml | head -n1 | cut -d '"' -f2)
fi

echo "Publishing version: $VERSION"

sed -i "0,/^version = \".*\"/s//version = \"$VERSION\"/" Cargo.toml

cargo publish --allow-dirty --token "$TOKEN"
