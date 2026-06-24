## Changelog for 0.1.0-alpha.9

### 🚀 Features
* feat: Add IntoJsonValue trait to generalize VM input parsing (#83)
* feat: Add nightly feature gating for experimental opcodes (#79)

### 🐛 Bug Fixes
* chore: Fix labeler.yml (#89)
* fix: Error process exit
* chore(scripts): Fix potential race condition
* fix(native): Fix parse ltc
* fix(native): Fix stringify ltc

### 🧹 Maintenance
* chore: Delete external deps (#84)
* chore: Update cspell
* Merge pull request #82 from soteenstudio/dependabot/npm_and_yarn/typescript-eslint/parser-8.62.0
* chore(deps-dev): bump @typescript-eslint/parser from 8.61.1 to 8.62.0
* Merge pull request #78 from soteenstudio/alert-fix-5
* Merge pull request #73 from soteenstudio/dependabot/cargo/napi-3.9.3
* Merge pull request #76 from soteenstudio/dependabot/npm_and_yarn/types/node-26.0.0
* Merge pull request #77 from soteenstudio/dependabot/npm_and_yarn/lint-staged-17.0.8
* Merge pull request #75 from soteenstudio/chore/docstring
* chore(rust): Hide napi and wasm from rust doc
* chore(rust): Add docstring
* chore(deps-dev): bump lint-staged from 17.0.7 to 17.0.8
* chore(deps-dev): bump @types/node from 25.9.3 to 26.0.0
* Merge pull request #74 from soteenstudio/fix/tools
* fix(native): Parse unquoted semicolons and handle string escape sequences
* Merge pull request #72 from soteenstudio/development
* chore(deps): bump napi from 3.9.2 to 3.9.3
* chore: Rename pull_request_template.md to PULL_REQUEST_TEMPLATE.md
* chore: Add security.yml
* chore: Change pr template
* chore: Update cspell.config.js
* chore: Update Cargo.lock
* Merge pull request #71 from soteenstudio/development
* chore: Update ci.yml
* chore: Rename domloo-release.yml to release.yml
* chore: Rename PULL_REQUEST_TEMPLATE.yml to pull_request_template.yml
* ci: Update domloo-release.yml
* ci: Update publish.yml

## Changelog for 0.1.0-alpha.8

* fix: Fix split_whitespace to regex
* fix: Fixed parse_ltc on loader.rs
* Potential fix for code scanning alert no. 6: Incomplete multi-character sanitization
* Potential fix for code scanning alert no. 2: Bad HTML filtering regexp
* Merge branch 'main' into alert-autofix-3
* Potential fix for code scanning alert no. 3: Uncontrolled data used in path expression
* Potential fix for code scanning alert no. 4: Uncontrolled data used in path expression
* perf: Optimizing interface performance
* refactor: Rename accessindex to access_index
## Changelog for v0.1.0-alpha.7

* fix: Fixed undefined tools() bug
* fix: Fixed a bug in the wasm interface
* feat: Implement wasm interface and benchmark updates
* feat(tools): Accept raw json string in optimize_bytecode for better dx
* refactor(napi): Resolve compilation warnings in event handling
* refactor: Remove emit function
* fix: Fixed bug where function on cannot exit
* perf: Optimize opcode performance by sorting hot and cold opcodes
* refactor(vm): Optimize instruction parsing and serialization
## Changelog for v0.1.0-alpha.6

* fix(vm): Add bounds checking for access_index and optimize array shrink
* feat: Add runtime bounds checking for control flow
* feat: Added reverse size limit to init opcode
* refactor: Tidy up the code structure
* feat: Adding the on() function
* fix: Fixed Capability issue with napi_interface.rs
* refactor: Delete file Instruction.ts
* perf: Optimize the dispatch and opcode execution sections
* perf: Optimize performance in key areas to improve user experience (using unsafe)
* perf: Optimizing the code of the program execution section
* refactor: tidy up code with ESLint and Husky
* refactor: tidy up some code
* refactor: clean up code and warnings from clippy
* refactor: tidy up the code
* fix: fix DSE opcode return section
* fix: fix func opcode and call opcode
* refactor: simplify provide API and fix provide function bug
## Changelog for v0.1.0-alpha.5

* refactor: simplify internal API and update documentation
* refactor: tidy up the isMusl.ts file
* fix: fix implicit any and unknown type errors
* refactor(napi): unify error handling and refine capability mapping
* refactor: make code idiomatic by addressing clippy warnings
* refactor: make code idiomatic by addressing clippy warnings
* refactor: add license header to files in mod folder
* refactor: remove double spaces before license link
* refactor: removal of double spaces in license header
* refactor: tidy up code with synclean
* refactor: tidy up the code with prettier
* Merge branch 'development'
* fix: unify error messages and refactor N-API loading
* refactor(core): move interfaces to separate files and improve error messages
* refactor: tidy up check-tokens.yml and publish.yml
* refactor: migrate internal functions from String error to VMError
