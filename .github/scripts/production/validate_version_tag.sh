VERSION="$1"
if [ -z "$VERSION" ]; then
  echo "Error: Version argument is required."
  exit 1
fi
if git rev-parse "refs/tags/v$VERSION" >/dev/null 2>&1; then
  echo "Error: $VERSION already has a tag! Change the version first."
  exit 1
fi

FILE_VERSION=$(jq -r .version package.json)
if [ "$FILE_VERSION" != "$VERSION" ]; then
  echo "Error: The version in package.json ($FILE_VERSION) is not the same as the input ($VERSION)."
  exit 1
fi