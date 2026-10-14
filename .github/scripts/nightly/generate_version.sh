set -x

RAW_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "0.0.0")

CLEAN_VERSION=${RAW_TAG#v}

BASE_VERSION=${CLEAN_VERSION%-*}

YYMM=$(date +'%yM%m')

N_COUNT=$(git tag -l "*-${YYMM}N*" 2>/dev/null | wc -l)
N_ITERATION=$((N_COUNT + 1))

VERSION_CLEAN="${BASE_VERSION}-${YYMM}N${N_ITERATION}"
VERSION_TAG="v${VERSION_CLEAN}"

echo "version_clean=$VERSION_CLEAN" >> $GITHUB_OUTPUT
echo "version_tag=$VERSION_TAG" >> $GITHUB_OUTPUT
echo "Generated version: $VERSION_CLEAN (Tag: $VERSION_TAG)"
