PREV_TAG=$(git tag --sort=-creatordate | sed -n '2p')

if [ -z "$PREV_TAG" ]; then
  echo "PREV_TAG not found. Cannot create release notes because this is the first build."
  exit 1
fi

if [ -z "$PREV_TAG" ]; then
  LOGS=$(git log --pretty=format:"%s")
else
  LOGS=$(git log $PREV_TAG..HEAD --pretty=format:"%s" | grep -v "chore: nightly release")
fi

FEAT=$(echo "$LOGS" | grep -E "^feat(\(.*\))?: " | sed -E 's/^feat(\(.*\))?: /- /' || echo "")
FIX=$(echo "$LOGS" | grep -E "^fix(\(.*\))?: " | sed -E 's/^fix(\(.*\))?: /- /' || echo "")
PERF=$(echo "$LOGS" | grep -E "^perf(\(.*\))?: " | sed -E 's/^perf(\(.*\))?: /- /' || echo "")

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
  
  if [ -n "$PERF" ]; then
    echo "### Performance"
    echo "$PERF"
    echo ""
  fi
  
  if [ -z "$FEAT" ] && [ -z "$FIX" ] && [ -z "$PERF" ]; then
    echo "_No significant changes in this build._"
    echo ""
  fi
  
  echo "***Nightly Owl has fallen out of bed tonight!***"
} > release_notes.md

gh release create "$VERSION_VAL" \
  --title "Nightly Build $VERSION_VAL" \
  --notes-file release_notes.md \
  --prerelease