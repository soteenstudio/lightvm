#!/usr/bin/env bash
set -euo pipefail

: "${VERSION_VAL:?VERSION_VAL must be set}"

git config user.name "github-actions[bot]"
git config user.email "41898282+github-actions[bot]@users.noreply.github.com"

git config commit.gpgsign true
git config tag.gpgsign true

git fetch --no-recurse-submodules --depth=1 origin

git add Cargo.toml
[ -f "package.json" ] && git add package.json
git add -A

if git diff --cached --quiet; then
  echo "No file changes, skip push."
  exit 0
fi

git commit -m "chore: nightly release ${VERSION_VAL}"

git tag "$VERSION_VAL"
git push origin HEAD --tags --no-verify
