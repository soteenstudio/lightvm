const npmValid = "${{ steps.check_npm.outputs.status }}" === "valid";
const cratesValid = "${{ steps.check_crates.outputs.status }}" === "valid";

const npmCode = "${{ steps.check_npm.outputs.code }}";
const cratesCode = "${{ steps.check_crates.outputs.code }}";

const npmStatus = npmValid ? "✅ VALID" : `❌ EXPIRED/INVALID (HTTP ${npmCode})`;
const cratesStatus = cratesValid ? "✅ VALID" : `❌ EXPIRED/INVALID (HTTP ${cratesCode})`;

await github.rest.issues.create({
  owner: context.repo.owner,
  repo: context.repo.repo,
  title: "⚠️ ALERT: GitHub Token Registry Expired!",
  assignees: ["claycuy"],
  labels: ["bug", "security"],
  body: `Hi, Maintainer. One of your tokens is no longer valid:

- NPM Token: ${npmStatus}
- Crates.io Token: ${cratesStatus}

Hurry up and change it in the repo settings!`
});