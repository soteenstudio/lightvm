git config user.name "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"

git config commit.gpgsign true
git config tag.gpgsign true

git add Cargo.toml
[ -f "package.json" ] && git add package.json

if ! git diff --cached --quiet; then
  git commit -m "chore: nightly release $VERSION_VAL"
  git tag "$VERSION_VAL"
  git push origin main --tags
else
  echo "No file changes, skip push."
fi