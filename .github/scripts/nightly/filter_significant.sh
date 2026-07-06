PREV_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")

if [ -n "$PREV_TAG" ]; then
  LOGS=$(git log $PREV_TAG..HEAD --pretty=format:"%s")
else
  LOGS=$(git log --pretty=format:"%s")
fi

HAS_SIGNIFICANT=$(echo "$LOGS" | grep -E "^(feat|fix)(\(.*\))?:" || echo "")

if [ -n "$HAS_SIGNIFICANT" ]; then
  echo "significant=true" >> $GITHUB_OUTPUT
  echo "There are significant changes, continue!"
else
  echo "significant=false" >> $GITHUB_OUTPUT
  echo "No feat/fix, skip build."
fi