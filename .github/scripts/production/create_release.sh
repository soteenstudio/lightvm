VERSION="${{ github.event.inputs.version }}"
git config user.name "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"

git tag "v$VERSION"
git push origin "v$VERSION"

gh release create "v$VERSION" \
  --title "Release $VERSION" \
  --notes-file release_notes.md