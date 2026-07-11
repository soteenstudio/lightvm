PREV_TAG=$(git describe --tags --abbrev=0 --exclude="$VERSION_VAL" 2>/dev/null || echo "")

LOGS=$(git log $PREV_TAG..HEAD --pretty=format:"%s")

FEAT=$(echo "$LOGS" | grep -E "^feat(\(.*\))?: " | sed -E 's/^feat(\(.*\))?: /- /' || echo "")
FIX=$(echo "$LOGS" | grep -E "^fix(\(.*\))?: " | sed -E 's/^fix(\(.*\))?: /- /' || echo "")

COMPARE_LINK="https://github.com/$REPOSITORY/compare/${PREV_TAG}...$VERSION_VAL"

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

gh release create "$VERSION_VAL" \
  --title "Nightly Build $VERSION_VAL" \
  --notes-file release_notes.md \
  --prerelease