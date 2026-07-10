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

> [!WARNING]
> This documentation is based on the `alpha.9` nightly build.
> 
> Refer to the [alpha.8 documentation](https://github.com/soteenstudio/lightvm/tree/v0.1.0-alpha.8) for more details.

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
 * __Redundant Load Elimination__: Detects consecutive attempts to load identical values or variables onto the stack and replaces redundant operations with a high-performance Dup instruction to minimize memory access overhead.
 * __Jump Optimization__: Detects and removes redundant Jump instructions that point to the very next line of code.
 * __Jump Threading__: Optimizes control flow by collapsing chains of redirection, where a jump leads directly to another jump, ensuring the instruction pointer bypasses intermediate hops to reach the final destination immediately.

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
<summary>Installation with Yarn</summary>

```bash
yarn add lightvm

# or
yarn add lightvm@next
```
</details>

<details>
<summary>Installation with Cargo</summary>

```bash
cargo add lightvm

# or specific version
cargo add lightvm@0.1.0-alpha.6
```
</details>

### Quick Usage

<details>
<summary>Using TypeScript:</summary>

```typescript
import { LightVM, Capability } from 'lightvm';

const vm = new LightVM({ caps: [Capability.Observe, Capability.Control] })
  .withNightly(false) // To allow nightly features (default: false)
  .withBacktrace(false) // To display backtrace details in error messages (default: false)
  .withExplain(false) // To display a more detailed hint in the error message (default: false)
  .withHint(true); // To display a hint on error messages (default: true)

/** * Get the tools interface. 
 * Store this as a constant to reuse it for all upcoming tasks.
 */
const tools = vm.tools();
```
</details>

<details>
<summary>Using Rust:</summary>

```rust
use lightvm::LightVM;
use lightvm::types::{vmconfig::VmConfig, capability::Capability};

fn main() {
  let mut vm = LightVM::new(VmConfig {
    caps: vec![Capability::Control, Capability::Observe],
    ..Default::default()
  })
  .with_nightly(false) // To allow nightly features (default: false)
  .with_backtrace(false) // To display backtrace details in error messages (default: false)
  .with_explain(false) // To display a more detailed hint in the error message (default: false)
  .with_hint(true); // To display a hint on error messages (default: true)
  
  /* * Get the tools interface. 
   * Store this as a constant to reuse it for all upcoming tasks.
   */
  let tools = vm.tools();
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
  Function to start bytecode execution.  

    <details>
    <summary>TypeScript:</summary>
  
    ```typescript
    const raw = [
      ["push", 5],
      ["val", "x"],
      ["set", "x"]
    ];
    vm.load(tools.optimizeBytecode(raw))
      .run();
    ```
    </details>  
    
    <details>
    <summary>Rust:</summary>
    
    ```rust
    let raw = r#"[
      ["push", 5],
      ["val", "x"],
      ["set", "x"]
    ]"#;
    vm.load(tools.optimize_bytecode(raw).clone())
      .run(None);
    ```
    </details>
    
> [!NOTE]
> __Capability Required__: control  
> __Info__: parameters of ``load()`` can change bytecode directly or file path to .ltc
    
2. ``provide()`` method:  
  Function to inject data/variables into the VM.

    <details>
    <summary>TypeScript:</summary>
    
    ```typescript
    vm.provide({
      name: "John Doe", 
      force: 2021,
    });
    let raw = [
      ["get", "name"],
      ["println"],
      ["get", "force"],
      ["println"]
    ];
    ```
    </details>
    
    <details>
    <summary>Rust:</summary>
    
    ```rust
    vm.provide(serde_json::json!({
      "name": "John Doe",
      "force": 2021
    }));
    let raw = r#"[
      ["get", "name"],
      ["println"],
      ["get", "force"],
      ["println"]
    ]"#;
    ```
    </details>
    
> [!NOTE]
> __Capability Required__: no specific capability
    
3. ``inspect()`` method:  
  Function to view state, number of instructions, and capability.

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
  Function to force/manually stop VM.

    <details>
    <summary>TypeScript:</summary>
    
    ```typescript
    vm.halt();
    vm.run(); // will not be executed
    console.log("The VM has been terminated.")
    ```
    </details>
    
    <details>
    <summary>Rust:</summary>
    
    ```rust
    vm.halt();
    vm.run(None); // will not be executed
    println!("The VM has been terminated.");
    ```
    </details>
    
> [!NOTE]
> __Capability Required__: unsafe

5. ``on()`` method:
  Function to register listener to VM.

    <details>
    <summary>TypeScript:</summary>
    
    ```typescript
    vm.on("halt", (payload) => {
      console.log("Halted: ", payload);
    });
    ```
    </details>
    
    <details>
    <summary>Rust:</summary>
    
    ```typescript
    vm.on("halt", |payload| {
      println!("Halted: ", payload);
    });
    ```
    </details>
  
> [!NOTE]
> __Capability Required__: no specific capability  
> __Event__: tick, halt, and panic
    
6. ``export()`` method:  
  Function to export functions in the VM out.

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
    if let Some(result) = add_func(args) {
        println!("Result from VM: {}", result);
    }
    ```
    </details>
    
> [!NOTE]
> __Capability Required__: control

7. ``tools()`` method:  
  Functions used to call utilities  
  
    <ol type="a">
      <li>Optimizes raw JSON bytecode and serializes it to a string</li>
      <details>
      <summary>TypeScript:</summary>
      
      ```typescript
      const optimized = tools.optimizeBytecode(raw);
      console.log(optimized);
      ```
      
      </details>
      <details>
      <summary>Rust:</summary>
      
      ```rust
      let optimized = tools.optimize_bytecode(raw);
      println!(optimized.clone());
      ```
      
      </details>
      
      <li>Converts raw JSON bytecode into a readable LTC assembly string</li>
      <details>
      <summary>TypeScript:</summary>
      
      ```typescript
      const tools = tools.tools;
      const stringify = tools.stringifyLTC(raw);
      console.log(stringify);
      ```
      
      </details>
      <details>
      <summary>Rust:</summary>
      
      ```rust
      let stringify = tools.stringify_ltc(raw);
      println!("{:#}", stringify.clone());
      ```
      
      </details>
      
      <li>Parses LTC code and serializes the instructions to a JSON string</li>
      <details>
      <summary>TypeScript:</summary>
      
      ```typescript
      const parsed = tools.parseLTC(raw);
      console.log(parsed.clone());
      ```
      
      </details>
      <details>
      <summary>Rust:</summary>
      
      ```rust
      let parsed = tools.parse_ltc(raw);
      println!("{:#}", parsed.clone());
      ```
      
      </details>
      
      <li>Parses an LTC string into a JSON array</li>
      <details>
      <summary>TypeScript:</summary>
      
      ```typescript
      const json = tools.parseLTCArray(raw);
      console.log(json.clone());
      ```
      
      </details>
      <details>
      <summary>Rust:</summary>
      
      ```rust
      let json = tools.parse_ltc_array(raw);
      println!("{:#}", json.clone());
      ```
      
      </details>
      
    </ol>

> [!NOTE]
> __Capability Required__: no specific capability

## References
### Supported Primitive Types
LightVM requires explicit type definitions for certain instructions to maintain deterministic execution and peak performance.
| Type | Aliases | Reference | Target Value Type |
|------|---------|-----------|-------------------|
| sht | i16 | Short | 16-bit Integer (Int16) |
| int | i32 | Integer | 32-bit Integer (Int32) |
| lng | i64 | Long | 64-bit Integer (Int64) |
| oct | i128 | Octa | 128-bit Integer (Int128) |
| hlf | f16 | Half | 16-bit Floating Point (Float16) |
| flt | f32 | Float | 32-bit Floating Point (Float32) |
| dbl | f64 | Double | 64-bit Floating Point (Float64) |
| str | - | String | String / Text data |

### Bytecode Instructions
LightVM has a total of 40+ instructions for bytecode.
1. Stack & Variable Management  
A group of instructions for basic data manipulation and memory (variable) allocation.

| Opcode | Arguments | Operands (stack) | Description |
|--------|-----------|------------------|-------------|
| push   | value     | - |Inserting data into the stack |
| val    | name      | - | Declaring a new variable |
| set    | name      | val | Take the top stack and then save it to the variable ``name`` |
| get    | name      | - | Take the contents of the ``name`` variable and push it onto the stack |
| dup    | -         | val | Duplicate the top value in the stack |
| swap | - | val1, val2 | Swap the top stack with the bottom stack |
2. Arithmetic & Logic  
Instructions for calculations. Note that for optimization, these instructions require a ``PrimitiveTypes`` (``sht``, ``hlf``, ``int``, ``flt``, ``lng``, ``dbl``, ``oct``) to prevent the VM from guessing the data type during execution.

| Opcode    | Arguments  | Operands (stack) | Description |
|-----------|------------|------------------|-------------|
| add / sub | type       | val1, val2 | Addition or Subtraction |
| mul / div | type       | val1, val2 | Multiplication or Division |
| mod       | type       | val1, val2 | Modulo (Remainder) |
| neg | type | val | Negation (-5 to 5, or vice versa) |
| inc / dec | name, type | - | Directly add/remove variable contents (without going through the stack) |
| gt / lt   | type       | val1, val2 |Greater Than or Less Than |
| ge / le   | type       | val1, val2 | Greater/Less Than or Equal |
| eq / neq  | type       | val1, val2 | Equal or Not Equal |
| shl / shr | type       | val1, val2 | Shift Left or Shift Right bitwise operation based on data type |
| rol / ror | type       | val1, val2 | __Circular__ Shift Left or Right (Rotate) bitwise operation based on data type |
| and / or  | -          | val1, val2 | Boolean logic operations (``&&`` / ``\|\|``) |
| xor       | -          | val1, val2 | Bitwise __Exclusive OR__ operation between two values |
| not       | -          | val1, val2 | Bitwise __NOT__ (Inversion) operation on a single value |
| pow | type | val1, val2 | General power operation (x^y) |
| powi | type | val1, val2 | Power with integer exponent |
| powf | type | val1, val2 | Power with floating point exponent |
| sin / cos | type | val1, val2 | Sine or Cosine trigonometric operation |
| tan | type | val1, val2 | Tangent trigonometric operation |

3. Control Flow & Function  
Instructions for managing program flow, looping, and function calls.

| Opcode   | Arguments | Operands (stack) | Description |
|----------|-----------|------------------|-------------|
| jump     | target_ip | - | Jump to a specific instruction line (Instruction Pointer) |
| if_false | target_ip | cond | Jump if the value on the stack is false |
| func     | name, argc, start, end, [params] | - | Function block definition (scope) |
| call     | name, argc | - | Call a function with a specified number of arguments |
| return   | -          | val | Exit the function and return to the caller |
| stop     | -          | - | Kill all VM processes (Halt) |
4. Data Structures & Metadata  
Create complex data handles like JS Objects or Arrays, plus data type matters.

| Opcode     | Arguments | Operands (stack) | Description |
|------------|-----------|------------------|-------------|
| make_obj   | count     | k1, v1, ... kn, vn | Create Object from n key-value pairs in stack |
| make_array | count     | v1, v2, ... vn | Create an Array of n elements in a stack |
| access     | prop_name | target_obj | Access Object's properties |
| access_index | -       | target_arr, index | Access Array elements by index on the stack |
| length     | -         | val | Check the length of a string or the number of items in an array/object |
| typeof     | -         | val | Get the data type from the top value of the stack |
| concat     | -         | val1, val2 | Combine two values (usually strings) |

5. Type Casting (Conversion)  
For those of you who want to force a certain data type to ensure consistent performance.

| Opcode     | Operands (stack) | Description                 |
|------------|------------------|-----------------------------|
| to_short   | val | Change value to Short (16-bit) |
| to_integer | val | Change value to Integer (32-bit) |
| to_long    | val | Change the value to Long (64-bit) |
| to_octa | val | Change value to Octa (128-bit Integer) |
| to_half    | val | Change value to Half-precision (16-bit Float) |
| to_float   | val | Change value to Float (32-bit) |
| to_double  | val | Change the value to Double (64-bit) |
| to_string  | val | Change the value to String  |
6. Objects & OOP
Instructions for handling class instances and modifying object properties dynamically.

| Opcode | Arguments | Operands (stack) | Description |
|--------|-----------|------------------|-------------|
| set_prop | prop_name | val, obj | Set the value of an object property (retrieve ``value`` and ``target_obj`` from the stack) |
| instantiate | class_name, argc | arg1, ... argN | Creates a new instance of a class with a specified number of constructor arguments |
| inspect_obj | - | obj | Prints the internal structure of an Object to the console |
| inspect_array | - | arr | Print the internal contents of an Array to the console |
> [!WARNING]
> __Nightly Opcode__: The `instantiate` instruction is still experimental. The API may change without notice in the `@next` version.

7. Module & Export System
Instructions for communication between modules or with external runtimes.

| Opcode | Arguments | Operands (stack) | Description |
|--------|-----------|------------------|-------------|
| import | module_name, alias_idx | target, length | Importing external libraries/modules into a specific variable index |
| export | name | - | Mark a function or variable to be accessible from outside the VM |
> [!WARNING]
> __Nightly Opcode__: The `export` and `import` instructions are still experimental. The API may change without notice in the `@next` version.

8. Basic I/O & Loop Control
Instructions for standard output and more specific iteration control.

| Opcode | Arguments | Operands (stack) | Description |
|--------|-----------|------------------|-------------|
| print | - | val | Prints the top value of the stack to the console without a newline |
| println | - | val | Prints the top value of the stack to the console with a newline |
| stdin | - | val | Reads a line from standard input, trims the trailing newline characters, and pushes the resulting string onto the stack |
| stdout | - | val | Pops the top value from the stack (must be a String) and prints it to the console without a newline |
| stdoutln | - | val | Pops the top value from the stack (must be a String) and prints it to the console followed by a newline |
| clear_screen | - | - | Clears the terminal screen and resets the cursor position to the top-left corner using ANSI escape codes (\x1B[2J\x1B[1H) |
| break | - | target_ip | Stops the loop and jumps to the specified target_ip |
| nop | - | - | Empty instructions (usually for placeholders or alignment) |

9. Advanced Stack & Memory Management
Special instructions for low-level control of the VM memory footprint in real-time.

| Opcode | Arguments | Operands (stack) | Description |
|--------|-----------|------------------|-------------|
| init_stack | size | - | Initialize the evaluation stack memory capacity at the beginning of VM execution to prevent reallocation |
| shrink | - | target, length | Reduces the capacity of the stack to fit its current length |
| truncate | - | target_size | Clear/reset the stack elements efficiently |

## Supported Architectures
LightVM supports a wide range of platforms and architectures to ensure maximum operational flexibility. Here's the current compatibility list:
| OS / Runtime | Architecture | Toolchain | Rust | Node.js | Web |
|--------------|--------------|-----------|-------|---------|-----|
| Windows      | x64, ia32    | MSVC      | ✓ | ✓ | - |
| Linux        | x64, ia32, arm64 | GNU (glibc) | ✓ | ✓ | - |
| Linux (musl) | x64, ia32, arm64 | musl      | ✓ | ✓ | - |
| macOS (Darwin) | x64      | Apple Clang | ✓ | ✓ | - |
| Android      | arm64, arm   | NDK       | ✓ | ✓ | - |
| FreeBSD      | x64          | Clang     | ✓ | ✓ | - |
| Web / Browser | wasm32 | wasm-pack / wasm-bindgen | - | - | ✓ |
> [!WARNING]
> __Nightly Support__: Support for `wasm32` for browsers is still in experimental stage.

## License & Changelog
 - This project is distributed using the [Apache-2.0 license](LICENSE).
 - See [credits](./CREDITS.md) for information regarding the project's origin and originality.
 - See [changelogs](./docs/CHANGELOG.md) for the latest updates and release history.