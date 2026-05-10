<p align="center">
  <a href="https://git.io/typing-svg">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://readme-typing-svg.herokuapp.com?font=Fira+Code&weight=700&size=40&pause=1000&color=FFFFFF&center=true&vCenter=true&width=435&lines=LightVM">
      <source media="(prefers-color-scheme: light)" srcset="https://readme-typing-svg.herokuapp.com?font=Fira+Code&weight=700&size=40&pause=1000&color=000000&center=true&vCenter=true&width=435&lines=LightVM">
      <img alt="LightVM" src="https://readme-typing-svg.herokuapp.com?font=Fira+Code&weight=700&size=40&pause=1000&color=000000&center=true&vCenter=true&width=435&lines=LightVM">
    </picture>
  </a>
</p>

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://readme-typing-svg.herokuapp.com?font=Fira+Code&italic=true&weight=400&size=18&pause=1000&color=A9A9A9&center=true&vCenter=true&width=435&lines=Minimalist+Execution.+Maximal+Security.">
    <source media="(prefers-color-scheme: light)" srcset="https://readme-typing-svg.herokuapp.com?font=Fira+Code&italic=true&weight=400&size=18&pause=1000&color=4A4A4A&center=true&vCenter=true&width=435&lines=Minimalist+Execution.+Maximal+Security.">
    <img alt="Tagline" src="https://readme-typing-svg.herokuapp.com?font=Fira+Code&italic=true&weight=400&size=18&pause=1000&color=4A4A4A&center=true&vCenter=true&width=435&lines=Minimalist+Execution.+Maximal+Security.">
  </picture>
</p>

<p align="center">
  <a href="https://github.com/soteenstudio/lightvm/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/soteenstudio/lightvm/ci.yml?branch=main&style=flat-square&color=black&label=build&logo=github" alt="build status">
  </a>
  <a href="https://www.npmjs.com/package/lightvm">
    <img src="https://img.shields.io/npm/v/lightvm?style=flat-square&color=black&logo=npm&logoColor=white" alt="npm version">
  </a>
  <a href="https://www.npmjs.com/package/lightvm">
    <img src="https://img.shields.io/npm/v/lightvm/next?style=flat-square&color=orange&label=nightly&logo=npm&logoColor=white" alt="nightly version">
  </a>
  <a href="https://www.npmjs.com/package/lightvm">
    <img src="https://img.shields.io/npm/dt/lightvm?style=flat-square&color=black&logo=target&logoColor=white" alt="total downloads">
  </a>
  <a href="https://github.com/soteenstudio/lightvm/stargazers">
    <img src="https://img.shields.io/github/stars/soteenstudio/lightvm?style=flat-square&color=black&logo=github&logoColor=white" alt="github stars">
  </a>
  <a href="https://github.com/soteenstudio/lightvm">
    <img src="https://img.shields.io/github/repo-size/soteenstudio/lightvm?style=flat-square&color=black&logo=database&logoColor=white" alt="repo size">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-Apache--2.0-black?style=flat-square&logo=apache&logoColor=white" alt="license">
  </a>
</p>

A capability-based virtual machine designed for __secure__, __predictable__, and __optimized bytecode execution__.
## The Philosophy: Deterministic & Lean
LightVM is built with a focus on execution transparency and resource efficiency:
 * __Zero Magic (Deterministic)__: Instruction execution is linear and completely predictable. The VM operates explicitly, executing instructions exactly as they are defined.
 * __Resource Conscious__: Designed with a minimal memory footprint through the use of optimized data structures such as SmolStr and ahash for fast metadata management.
 * __Explicit Security__: Security is managed through a strict Capability system. Every VM access and operation must have permissions explicitly defined by the host from the outset.

## AOT Optimization Process
LightVM isn't just a simple interpreter. Before execution, your bytecode undergoes a multi-pass optimization to ensure maximum performance and minimal footprint:
 * __Constant Folding__: Pre-calculates math and logic operations (e.g., Add, Sub, Xor, Concat) if the values are known at compile-time.
 * __Conversion & Metadata Folding__: Pre-evaluates type casting (e.g., ToInteger, ToString) and metadata checks like TypeOf to eliminate redundant runtime work.
 * __Strength Reduction__: Replaces "heavy" operations with lighter ones, such as converting multiplication by powers of two into bitwise Shl (Shift Left).
 * __Dead Store Elimination__: Analyzes variable usage and automatically removes Push, Set, or Inc operations that don't contribute to the final program state.
 * __Dead Loop Elimination__: Identifies and prunes "pure" loops that have no side effects (no I/O, calls, or returns), preventing unnecessary CPU cycles.
 * __Jump Optimization__: Detects and removes redundant Jump instructions that point to the very next line of code.
## Getting Started
### Installation

<details>
<summary>Installation with NPM</summary>

```bash
npm install lightvm

# or
npm install lightvm@next
```
</details>

<details>
<summary>Installation with Cargo</summary>

```bash
cargo add lightvm
```
</details>

### Quick Usage

<details>
<summary>Using TypeScript:</summary>

```typescript
import { LightVM, Capability } from 'lightvm';

const caps = [Capability.Control, Capability.Observe];

const vm = new LightVM(caps);
```
</details>

<details>
<summary>Using Rust:</summary>

```rust
use lightvm::LightVM;
use lightvm::types::capability::Capability;

fn main() {
    let caps = vec![Capability::Control, Capability::Observe];
    
    let mut vm = LightVM::new(caps);
}
```
</details>

### Virtual Machine Capabilities
LightVM uses a strict capability-based security model. You must explicitly grant permissions when instantiating the VM.
| Capability | Level | Description |
|------------|-------|-------------|
| Control | Low | Grants permission to start/stop execution and export functions. |
| Observe | Medium | Allows the host to inspect internal states, variable stacks, and metrics. |
| Debug | High | Opens access to verbose internal logs and hidden states for troubleshooting. |
| Unsafe | Critical | Removes safety guards, allowing manual halts and raw memory/process access. |

### How to use
1. ``run()`` __method__:  
  Permission to start bytecode execution.  

    <details>
    <summary>TypeScript:</summary>
  
    ```typescript
    const raw = [
      ["push", 5],
      ["val", "x"],
      ["set", "x"]
    ];
    vm.load(vm.tools().optimizeBytecode(JSON.stringify(raw)))
      .run();
    ```
    </details>  
    
    <details>
    <summary>Rust:</summary>
    
    ```rust
    let raw = serde_json::json!([
      ["push", 5],
      ["val", "x"],
      ["set", "x"]
    ]);
    LightVM::tools().optimize_bytecode(raw)
      .map(|opt| vm.load(serde_json::from_str(&opt).unwrap()).run(None))
      .expect("Optimization failed");
    ```
    </details>
    
> [!NOTE]
> __Capability Required__: control  
> __Info__: parameters of ``load()`` can change bytecode directly or file path to .ltc
    
2. ``provide()`` method:  
  Permission to inject data/variables into the VM.

    <details>
    <summary>TypeScript:</summary>
    
    ```typescript
    vm.provide("identity", {
      name: "John Doe", 
      force: "2021",
    });
    ```
    </details>
    
    <details>
    <summary>Rust:</summary>
    
    ```rust
    vm.provide("identity".to_string(), serde_json::json!({
        "name": "John Doe",
        "force": "2021"
    }));
    ```
    </details>
    
> [!NOTE]
> __Capability Required__: no spesific capability
    
3. ``inspect()`` method:  
  Permission to view state, number of instructions, and capability.

    <details>
    <summary>TypeScript:</summary>
    
    ```typescript
    const report = vm.inspect();
    console.log(report);
    ```
    </details>
    
    <details>
    <summary>Rust:</summary>
    
    ```rust
    let report = vm.inspect();
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    ```
    </details>
    
> [!NOTE]
> __Capability Required__: observe
    
4. ``halt()`` method:  
  Permission to force/manually stop VM.

    <details>
    <summary>TypeScript:</summary>
    
    ```typescript
    vm.halt();
    console.log("The VM has been terminated.")
    ```
    </details>
    
    <details>
    <summary>Rust:</summary>
    
    ```rust
    vm.halt();
    println!("The VM has been terminated.");
    ```
    </details>
    
> [!NOTE]
> __Capability Required__: unsafe
    
5. ``export()`` method:  
  Permission to export functions in the VM out.

    <details>
    <summary>TypeScript:</summary>
    
    ```typescript
    const add = vm.export("add");
    console.log(add(5, 6));
    ```
    </details>
    
    <details>
    <summary>Rust:</summary>
    
    ```rust
    let mut add = vm.export("add".to_string());
    let args = vec![serde_json::json!(5), serde_json::json!(6)];
    if let Some(hasil) = add_func(args) {
        println!("Hasil dari VM: {}", hasil);
    }
    ```
    </details>
    
> [!NOTE]
> __Capability Required__: control

## References
### Supported Primitive Types
LightVM requires explicit type definitions for certain instructions to maintain deterministic execution and peak performance.
| Type | Reference | Target Value Type |
|------|-----------|-------------------|
| sht | Short | 16-bit Integer (Int16) |
| int | Integer | 32-bit Integer (Int32) |
| lng | Long | 64-bit Integer (Int64) |
| hlf | Half | 16-bit Floating Point (Float16) |
| flt | Float | 32-bit Floating Point (Float32) |
| dbl | Double | 64-bit Floating Point (Float64) |
| str | String | String / Text data |

> [!WARNING]
> __Nightly Type__: The `hlf` (Half-precision) type is still experimental. Support across different architectures may vary and is subject to change in `@next` releases.


### Bytecode Instructions
LightVM has a total of 40+ instructions for bytecode.
1. Stack & Variable Management  
A group of instructions for basic data manipulation and memory (variable) allocation.

| Opcode | Arguments | Description |
|--------|-----------|-------------|
| push   | value     | Inserting data into the stack |
| val    | name      | Declaring a new variable |
| set    | name      | Take the top stack and then save it to the variable ``name`` |
| get    | name      | Take the contents of the ``name`` variable and push it onto the stack |
| dup    | -         | Duplicate the top value in the stack |
2. Arithmetic & Logic  
Instructions for calculations. Note that for optimization, these instructions require a ``PrimitiveTypes`` (``int``, ``flt``, ``lng``, ``dbl``) to prevent the VM from guessing the data type during execution.

| Opcode    | Arguments  | Description |
|-----------|------------|-------------|
| add / sub | type       | Addition or Subtraction |
| mul / div | type       | Multiplication or Division |
| mod       | type       | Modulo (Remainder) |
| inc / dec | name, type | Directly add/remove variable contents (without going through the stack) |
| gt / lt   | type       | Greater Than or Less Than |
| ge / le   | type       | Greater/Less Than or Equal |
| eq / neq  | type       | Equal or Not Equal |
| shl / shr | type       | Shift Left or Shift Right bitwise operation based on data type |
| rol / ror | type       | __Circular__ Shift Left or Right (Rotate) bitwise operation based on data type |
| and / or  | -          | Boolean logic operations (``&&`` / ``\|\|``) |
| xor       | -          | Bitwise __Exclusive OR__ operation between two values |
| not       | -          | Bitwise __NOT__ (Inversion) operation on a single value |
> [!NOTE]
> **Specific Opcode**: `shl`, `shr`, `rol`, and `ror` only accepts `sht`, `int`, and `lng` types from `PrimitiveTypes`.

3. Control Flow & Function  
Instructions for managing program flow, looping, and function calls.

| Opcode   | Arguments | Description |
|----------|-----------|-------------|
| jump     | target_ip | Jump to a specific instruction line (Instruction Pointer) |
| if_false | target_ip | Jump if the value on the stack is false |
| func     | name, argc, start, end, [params] | Function block definition (scope) |
| call     | name, argc | Call a function with a specified number of arguments |
| return   | -          | Exit the function and return to the caller |
| stop     | -          | Kill all VM processes (Halt) |
4. Data Structures & Metadata  
Create complex data handles like JS Objects or Arrays, plus data type matters.

| Opcode     | Arguments | Description |
|------------|-----------|-------------|
| make_obj   | count     | Create Object from n key-value pairs in stack |
| make_array | count     | Create an Array of n elements in a stack | Access properties of Object |
| access     | prop_name | Access Object's properties |
| access_index | -       | Access Array elements by index on the stack |
| length     | -         | Check the length of a string or the number of items in an array/object |
| typeof     | -         | Get the data type from the top value of the stack |
| concat     | -         | Combine two values (usually strings) |
5. Type Casting (Conversion)  
For those of you who want to force a certain data type to ensure consistent performance.

| Opcode     | Description                 |
|------------|-----------------------------|
| to_string  | Change the value to String  |
| to_short   | Change value to Short (16-bit) |
| to_integer | Change value to Integer (32-bit) |
| to_long    | Change the value to Long (64-bit) |
| to_half    | Change value to Half-precision (16-bit Float) |
| to_float   | Change value to Float (32-bit) |
| to_double  | Change the value to Double (64-bit) |
6. Objects & OOP
Instructions for handling class instances and modifying object properties dynamically.

| Opcode | Arguments | Description |
|--------|-----------|-------------|
| set_prop | prop_name | Set the value of an object property (retrieve ``value`` and ``target_obj`` from the stack) |
| instantiate | class_name, argc | Creates a new instance of a class with a specified number of constructor arguments |
| inspect_obj | - | Prints the internal structure of an Object to the console |
| inspect_array | - | Print the internal contents of an Array to the console |
> [!WARNING]
> __Nightly Opcode__: The `instantiate` instruction is still experimental. The API may change without notice in the `@next` version.

7. Module & Export System
Instructions for communication between modules or with external runtimes.

| Opcode | Arguments | Description |
|--------|-----------|-------------|
| import | module_name, alias_idx | Importing external libraries/modules into a specific variable index |
| export | name | Mark a function or variable to be accessible from outside the VM |
> [!WARNING]
> __Nightly Opcode__: The `export` and `import` instructions are still experimental. The API may change without notice in the `@next` version.

8. Basic I/O & Loop Control
Instructions for standard output and more specific iteration control.

| Opcode | Arguments | Description |
|--------|-----------|-------------|
| print | - | Prints the top value of the stack to the console without a newline |
| println | - | Prints the top value of the stack to the console with a newline |
| break | target_ip | Stops the loop and jumps to the specified target_ip |
| nop | - | Empty instructions (usually for placeholders or alignment) |
## Supported Architectures
LightVM supports a wide range of platforms and architectures to ensure maximum operational flexibility. Here's the current compatibility list:
| OS / Runtime | Architecture | Toolchain | Rust | Node.js |
|--------------|--------------|-----------|-------|---------|
| Windows      | x64, ia32    | MSVC      | ✓ | ✓ |
| Linux        | x64, ia32, arm64 | GNU (glibc) | ✓ | ✓ |
| Linux (musl) | x64, ia32, arm64 | musl      | ✓ | ✓ |
| macOS (Darwin) | x64      | Apple Clang | ✓ | ✓ |
| Android      | arm64, arm   | NDK       | ✓ | ✓ |
| FreeBSD      | x64          | Clang     | ✓ | ✓ |
## 📜 License
This project is distributed using the __Apache-2.0 license__
