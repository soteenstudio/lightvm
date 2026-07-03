RAW_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "0.0.0")

CLEAN_VERSION=${RAW_TAG#v}

BASE_VERSION=${CLEAN_VERSION%-nightly.*}

COMMIT_HASH=$(git rev-parse --short HEAD)
DATE=$(date +'%Y%m%d') # Format: 20260625

VERSION_CLEAN="${BASE_VERSION}-nightly.${DATE}.${COMMIT_HASH}"
VERSION_TAG="v${VERSION_CLEAN}"

echo "version_clean=$VERSION_CLEAN" >> $GITHUB_OUTPUT
echo "version_tag=$VERSION_TAG" >> $GITHUB_OUTPUT
echo "Generated version: $VERSION_CLEAN (Tag: $VERSION_TAG)"