echo "Checking NPM token..."
RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" -H "Authorization: Bearer $NPM_TOKEN" https://registry.npmjs.org/-/whoami)
echo "code=$RESPONSE" >> $GITHUB_OUTPUT
if [ "$RESPONSE" -eq 200 ]; then
  echo "status=valid" >> $GITHUB_OUTPUT
else
  echo "status=invalid" >> $GITHUB_OUTPUT
fi