#!/usr/bin/env bash
set -euo pipefail

VERSION=$(grep '^version =' Cargo.toml | head -n1 | cut -d '"' -f2)
echo "Publishing version: $VERSION"

cargo publish --allow-dirty --token "$TOKEN"
