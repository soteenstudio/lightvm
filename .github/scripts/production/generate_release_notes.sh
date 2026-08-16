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
  REPO="${GITHUB_REPOSITORY}"
  if [ -n "$LAST_STABLE" ]; then
    COMPARE_LINK="https://github.com/$REPO/compare/$LAST_STABLE...v$VERSION"
    echo "Compare: $COMPARE_LINK"
    echo ""
  fi

  echo "Release based on Changelogs:"
  if [ -n "$FINAL_NIGHTLIES" ]; then
    SORTED_NIGHTLIES=""
    while IFS= read -r tag; do
      [ -z "$tag" ] && continue
      TAG_DATE=$(git log -1 --format=%ct "$tag" 2>/dev/null || echo 0)
      SORTED_NIGHTLIES="$SORTED_NIGHTLIES$TAG_DATE $tag\n"
    done <<EOF
$FINAL_NIGHTLIES
EOF
    
    SORTED_NIGHTLIES=$(echo -e "$SORTED_NIGHTLIES" | sed '/^$/d' | sort -n | cut -d' ' -f2-)
    
    while IFS= read -r tag; do
      [ -z "$tag" ] && continue
      CLEAN_NAME=$(echo "$tag" | sed -E 's/-nightly\.([0-9]{4})([0-9]{2})([0-9]{2})\..*/ (Nightly \1-\2-\3)/')
      echo "* [$CLEAN_NAME](https://github.com/$REPO/releases/tag/$tag)"
    done <<EOF
$SORTED_NIGHTLIES
EOF
  else
    echo "* No previous nightly builds."
  fi
  echo ""
  echo "***Lion Owl caught in apple net!***"
} > release_notes.md
