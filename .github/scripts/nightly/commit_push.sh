git config user.name "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"

git add Cargo.toml
[ -f "package.json" ] && git add package.json

if ! git diff --cached --quiet; then
  git commit -m "chore: nightly release ${{ steps.versioning.outputs.version_clean }}"
  git tag ${{ steps.versioning.outputs.version_tag }}
  git push origin main --tags
else
  echo "No file changes, skip push."
fi