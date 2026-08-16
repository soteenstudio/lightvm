VERSION="$1"

LAST_STABLE=$(git tag --sort=-creatordate | grep -v "nightly" | head -n 1 || echo "")

if [ -n "$LAST_STABLE" ]; then
   STABLE_DATE=$(git log -1 --format=%ct "$LAST_STABLE" 2>/dev/null || echo 0)
   
   FINAL_NIGHTLIES=""
   for tag in $(git tag | grep "nightly" | sort -V); do
     TAG_DATE=$(git log -1 --format=%ct "$tag" 2>/dev/null || echo 0)
     if [ "$TAG_DATE" -gt "$STABLE_DATE" ]; then
       FINAL_NIGHTLIES="$FINAL_NIGHTLIES\n$tag"
     fi
   done
   FINAL_NIGHTLIES=$(echo -e "$FINAL_NIGHTLIES" | sed '/^$/d')
else
   FINAL_NIGHTLIES=$(git tag | grep "nightly" | sort -V)
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
