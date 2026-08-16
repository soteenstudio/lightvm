VERSION="$1"
git config user.name "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"

git tag "v$VERSION"
git push origin "v$VERSION"

IS_PRERELEASE=""
if [[ "$VERSION" =~ alpha|beta|rc|- ]]; then
  IS_PRERELEASE="--prerelease"
fi

gh release create "v$VERSION" \
  $IS_PRERELEASE \
  --title "Release $VERSION" \
  --notes-file release_notes.md
