CLONES=$(curl -s -H "Authorization: token $LIGHTVM_PAT" \
"https://api.github.com/repos/$GITHUB_REPOSITORY/traffic/clones" \
| jq '.count // 0')

TRAFFIC_DATA=$(curl -s -H "Authorization: token $LIGHTVM_PAT" \
"https://api.github.com/repos/$GITHUB_REPOSITORY/traffic/views")

TOTAL_VIEWS=$(echo $TRAFFIC_DATA | jq '.count // 0')
UNIQUE_VISITORS=$(echo $TRAFFIC_DATA | jq '.uniques // 0')

curl -s --fail -X PATCH -H "Authorization: token $LIGHTVM_PAT" \
-H "Content-Type: application/json" \
-d "{\"files\":{\"stats.json\":{\"content\":\"{\\\"clones\\\": $CLONES, \\\"total_views\\\": $TOTAL_VIEWS, \\\"unique_visitors\\\": $UNIQUE_VISITORS}\"}}}" \
"https://api.github.com/gists/$GIST_ID"
