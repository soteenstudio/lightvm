# LightVM

*Minimalist Execution. Maximal Security.*

[![Build status](https://img.shields.io/github/actions/workflow/status/soteenstudio/lightvm/ci.yml?branch=main&style=flat-square&color=black&label=build&logo=github)](https://github.com/soteenstudio/lightvm/actions)
[![npm version](https://img.shields.io/npm/v/lightvm?style=flat-square&color=black&logo=npm&logoColor=white)](https://www.npmjs.com/package/lightvm)
[![next version](https://img.shields.io/npm/v/lightvm/nightly?style=flat-square&color=orange&label=next&logo=npm&logoColor=white)](https://www.npmjs.com/package/lightvm)
[![GitHub stars](https://img.shields.io/github/stars/soteenstudio/lightvm?style=flat-square&color=black&logo=github&logoColor=white)](https://github.com/soteenstudio/lightvm/stargazers)
[![license](https://img.shields.io/badge/license-Apache--2.0-black?style=flat-square&logo=apache&logoColor=white)](https://www.apache.org/licenses/LICENSE-2.0)

A capability-based virtual machine designed for __secure__, __predictable__, and __optimized bytecode execution__.

## Documentation
Check out our official website for complete documentation.
 - **[Official Documentation](https://lightvm.vercel.app)**

## Templates
Use a template to make it easier when developing a new project.
 - [TypeScript Template](https://github.com/soteenstudio/lightvm-typescript)
 - [Rust Template](https://github.com/soteenstudio/lightvm-rust)

## Resource
 - This project is distributed using the [Apache-2.0 license](LICENSE).
 - See [credits](./CREDITS.md) for information regarding the project's origin and originality.
 - See [releases](https://github.com/soteenstudio/lightvm/releases) for the latest updates and release history.
 - See [contributing](./.github/CONTRIBUTING.md) to learn how to contribute.

## Rust benchmarks

The six existing benchmark targets per mode each run three named workloads, for 18 normal
and 18 optimized cases (36 total). Both modes use the same raw bytecode for each case:

| Target pair | Workloads (each runs in `normal` and `optimize` mode) |
| --- | --- |
| `add_bench` | Integer order totals; floating-point price and tax; numeric conversion and arithmetic pipeline |
| `assign_bench` | Shopping-cart quantity updates; account-balance updates; multi-variable application-state updates |
| `concat_bench` | User-facing message; structured status text; multi-step string assembly |
| `dead_code_bench` | Conditional with unreachable work; jump-heavy workflow with removable instructions; redundant intermediate calculations |
| `function_call_bench` | Exported two-argument calculation; exported multi-argument business calculation; sequential exported-function workflow |
| `io_bench` | Array construction and indexed access; object construction and property access; collection inspection and length processing |

Each case's benchmark name appends `_normal` or `_optimize`. VM construction and
`vm.load(...)` take place in the setup closure, outside `time per op`. Optimized cases
run `tools.optimize_bytecode(...)` before setup and timing; optimization is not measured.
The timed operation is only `vm.run(None)` or an exported-function call. Timed cases
perform no terminal I/O (`println`, `stdout`, `stdin`, or `clear_screen`). The
`io_bench` target name is retained for compatibility, but its workloads use collections.

Run individual target pairs or all registered benchmarks from the repository root:

```bash
cargo bench --bench add_bench
cargo bench --bench add_bench_optimize
cargo bench
```
