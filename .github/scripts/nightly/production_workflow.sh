TODAY=$(date -u +'%Y-%m-%d')

RUNS=$(gh run list --workflow production.yml --limit 1 --json createdAt --jq '.[0].createdAt')

if [[ "$RUNS" == "$TODAY"* ]]; then
  echo "Production is underway today: $RUNS"
  echo "should_skip=true" >> $GITHUB_OUTPUT
else
  echo "Production has not started today."
  echo "should_skip=false" >> $GITHUB_OUTPUT
fi