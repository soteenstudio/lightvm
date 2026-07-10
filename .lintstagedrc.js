export default {
  "*.{js,ts,rs,md,sh,yml}": (filenames) => {
    const filtered = filenames.filter(file => !file.includes('docs/'));
    return filtered.length ? `npm run cspell ${filtered.join(' ')}` : [];
  },
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
    "sh -c 'cargo clippy --color always -- -D warnings'",
  ],
  ".testings/**/*.rs": [
    "sh -c 'cargo fmt --'",
    "sh -c 'cargo clippy --color always -- -D warnings'"
  ]
}