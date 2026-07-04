VERSION="${{ github.event.inputs.version }}"
if git rev-parse "v$VERSION" >/dev/null 2>&1; then
  echo "Error: Versi $VERSION sudah ada tag-nya! Ganti versi dulu."
  exit 1
fi

FILE_VERSION=$(jq -r .version package.json)
if [ "$FILE_VERSION" != "$VERSION" ]; then
  echo "Error: Versi di package.json ($FILE_VERSION) tidak sama dengan input ($VERSION)."
  exit 1
fi