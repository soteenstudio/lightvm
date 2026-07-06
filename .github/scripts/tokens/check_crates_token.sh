echo "Checking Crates.io token using Cargo CLI..."

if cargo owner --list lightvm --token "$CRATES_TOKEN" > /dev/null 2>&1; then
  echo "code=200" >> $GITHUB_OUTPUT
  echo "status=valid" >> $GITHUB_OUTPUT
else
  echo "code=401" >> $GITHUB_OUTPUT
  echo "status=invalid" >> $GITHUB_OUTPUT
fi