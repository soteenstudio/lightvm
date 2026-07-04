VERSION="${{ github.event.inputs.version_override }}"

if [ -z "$VERSION" ]; then
  RAW_VERSION="${{ github.event.release.tag_name }}"
  VERSION=${RAW_VERSION#v}
fi

if [ -z "$VERSION" ]; then
  VERSION=$(grep '^version =' Cargo.toml | head -n1 | cut -d '"' -f2)
fi

echo "Publishing version: $VERSION"
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml