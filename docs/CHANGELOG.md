## Changelog for 0.1.0-nightly.20260701.305f0b7-alpha.1

### 🐛 Bug Fixes
* fix: Backtrace options and VMError handling (#111)

### 🧹 Maintenance
* chore(deps-dev): bump @commitlint/cli from 21.1.0 to 21.2.0 (#110)
* chore(deps-dev): bump @types/node from 26.0.1 to 26.1.0 (#112)
* chore(deps): bump napi from 3.9.4 to 3.10.0 (#113)
* chore(deps): bump napi-derive from 3.5.7 to 3.5.8 (#114)
* chore: Update cargo.lock

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
