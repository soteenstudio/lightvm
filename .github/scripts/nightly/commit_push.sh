#!/usr/bin/env bash
set -euo pipefail

# 1. Setup Identity
git config user.name "github-actions[bot]"
git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
git config commit.gpgsign true
git config tag.gpgsign true

git fetch origin main

git add -A
if git diff --cached --quiet; then
  echo "No file changes, skip push."
  exit 0
fi
git commit -m "chore: nightly release $VERSION_VAL"

git rebase origin/main

git tag -a "$VERSION_VAL" -m "Nightly Release $VERSION_VAL"
git push origin HEAD:main --follow-tags --no-verify
