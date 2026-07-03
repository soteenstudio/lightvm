PREV_TAG=$(git describe --tags --abbrev=0 --exclude="${{ steps.versioning.outputs.version_tag }}" 2>/dev/null || echo "")

LOGS=$(git log $PREV_TAG..HEAD --pretty=format:"%s")

FEAT=$(echo "$LOGS" | grep -E "^feat(\(.*\))?: " | sed -E 's/^feat(\(.*\))?: /- /' || echo "")
FIX=$(echo "$LOGS" | grep -E "^fix(\(.*\))?: " | sed -E 's/^fix(\(.*\))?: /- /' || echo "")

COMPARE_LINK="https://github.com/${{ github.repository }}/compare/${PREV_TAG}...${{ steps.versioning.outputs.version_tag }}"

{
  echo "## What's Changed"
  echo "Compare: $COMPARE_LINK"
  echo ""
  
  if [ -n "$FEAT" ]; then
    echo "### Features"
    echo "$FEAT"
    echo ""
  fi
  
  if [ -n "$FIX" ]; then
    echo "### Fixes"
    echo "$FIX"
    echo ""
  fi
  
  if [ -z "$FEAT" ] && [ -z "$FIX" ]; then
    echo "_No significant changes in this build._"
    echo ""
  fi
  
  echo "***Nightly Owl has fallen out of bed tonight!***"
} > release_notes.md

gh release create "${{ steps.versioning.outputs.version_tag }}" \
  --title "Nightly Build ${{ steps.versioning.outputs.version_tag }}" \
  --notes-file release_notes.md \
  --prerelease