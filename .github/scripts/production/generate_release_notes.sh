VERSION="${{ github.event.inputs.version }}"

LAST_STABLE=$(git tag --sort=-creatordate | grep -v "nightly" | head -n 1 || echo "")

NIGHTLIES=$(git tag | grep "nightly" | sort -V)

if [ -n "$LAST_STABLE" ]; then
   FINAL_NIGHTLIES=$(echo -e "$NIGHTLIES\n$LAST_STABLE" | sort -V | sed -n "/$LAST_STABLE/,\$p" | grep -v "$LAST_STABLE")
else
   FINAL_NIGHTLIES="$NIGHTLIES"
fi

{
  echo "Release based on Changelogs:"
  if [ -n "$FINAL_NIGHTLIES" ]; then
    echo "$FINAL_NIGHTLIES" | sed 's/^/* /'
  else
    echo "* No previous nightly builds."
  fi
  echo ""
  echo "***Lion Owl caught in apple net!***"
} > release_notes.md