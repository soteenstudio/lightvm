export default {
  "*.{js,ts,rs,md}": "npm run cspell",
  "{ts/src,ts/tests,scripts}**/*.{js,ts}": [
    () => "npm run typecheck",
    "eslint --fix",
    "prettier --write"
  ],
  ".testings/**/*.{js,ts,html}": [
    () => "npm run html:cleaner",
    () => "npm run typecheck",
    "eslint --fix",
    "prettier --write"
  ],
  "rust/**/*.rs": [
    "sh -c 'cargo fmt --'",
    "sh -c 'cargo clippy --color always -- -D warnings'"
  ],
  ".testings/**/*.rs": [
    "sh -c 'cargo fmt --'",
    "sh -c 'cargo clippy --color always -- -D warnings'"
  ]
}