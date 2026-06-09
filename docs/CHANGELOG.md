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
