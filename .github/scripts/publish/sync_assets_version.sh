#!/usr/bin/env bash
set -euo pipefail

VERSION="${INPUT_VERSION:-}"

if [ -z "$VERSION" ]; then
  VERSION="${INPUT_RAW_VERSION:-}"
fi

if [ -z "$VERSION" ]; then
  VERSION=$(grep '^version =' Cargo.toml | head -n1 | cut -d '"' -f2)
fi

VERSION="${VERSION#v}"

echo "Syncing version: $VERSION"

sed -i "0,/^version = \".*\"/s//version = \"$VERSION\"/" Cargo.toml
