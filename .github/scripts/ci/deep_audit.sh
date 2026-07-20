FILES=$(find . -type d \( -name "target" -o -name "node_modules" -o -name "dist" -o -name "generated" -o -name "docs" \) -prune -o -type f \( \( -name "*.ts" -o -name "*.rs" \) ! -name "*.sh" \) -print)

if [ -z "$FILES" ]; then
  echo "There are no files to check."
  exit 0
fi

MISSING_COPYRIGHT=$(echo "$FILES" | xargs grep -L "Copyright")
if [ -n "$MISSING_COPYRIGHT" ]; then
  echo "This file needs a Copyright header:"
  echo "$MISSING_COPYRIGHT"
  exit 1
fi

JUNK_CHECK=$(echo "$FILES" | xargs awk 'FNR==1 && /^\/\/ [A-Za-z0-9_-]+\.(ts|rs)$/ {print FILENAME}')
if [ -n "$JUNK_CHECK" ]; then
  echo "The first line is just the file name (Junk):"
  echo "$JUNK_CHECK"
  exit 1
fi