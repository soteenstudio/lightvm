# ⚡ LightVM
> __Minimalist Execution. Maximal Security.__
> 

A capability-based virtual machine designed for __secure__, __predictable__, and __optimized bytecode execution__.
## The Philosophy: Deterministic & Lean
LightVM is built with a focus on execution transparency and resource efficiency:
 * __Zero Magic (Deterministic)__: Instruction execution is linear and completely predictable. The VM operates explicitly, executing instructions exactly as they are defined.
 * __AOT Optimized__: Bytecode goes through an Ahead-of-Time (AOT) optimization stage to eliminate redundant operations before execution begins, ensuring maximum efficiency at runtime.
 * __Resource Conscious__: Designed with a minimal memory footprint through the use of optimized data structures such as SmolStr and ahash for fast metadata management.
 * __Explicit Security__: Security is managed through a strict Capability system. Every VM access and operation must have permissions explicitly defined by the host from the outset.

## 🚀 Getting Started
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

### Capability
LightVM has 4 capabilities to grant specific permissions to virtual machines, such as: Control, Observe, Debug, and Unsafe.
- __Control__: Grants VM permission to view or retrieve data.
- __Observe__: Grants permission to manipulate flow or state within the VM.
- __Debug__: Opens access to internal states that are usually hidden.
- __Unsafe__: Opens security protection or VM limitations.

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
    
## Bytecode Instructions
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
Instructions for calculations. Note that for optimization, these instructions require a ``PrimitiveType`` (``int``, ``flt``, ``lng``, ``dbl``) to prevent the VM from guessing the data type during execution.

| Opcode    | Arguments  | Description |
|-----------|------------|-------------|
| add / sub | type       | Addition or Subtraction |
| mul / div | type       | Multiplication or Division |
| mod       | type       | Modulo (Remainder) |
| inc / dec | name, type | Directly add/remove variable contents (without going through the stack) |
| gt / lt   | type       | Greater Than or Less Than |
| ge / le   | type       | Greater/Less Than or Equal |
| eq / neq  | type       | Equal or Not Equal |
| shl       | type       | Shift Left bitwise operation based on data type |
| and / or  | -          | Boolean logic operations (&& / ||) |
> [!NOTE]
> **Specific Opcode**: `shl` only accepts `int` and `lng` types from `PrimitiveTypes`.

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
| to_integer | Change value to Integer (32-bit) |
| to_long    | Change the value to Long (64-bit) |
| to_float   | Change value to Float       |
| to_double  | Change the value to Double  |
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
