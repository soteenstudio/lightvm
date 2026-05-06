# ⚡ LightVM
> **Fast. Dumb. Efficient.**
> 
A Virtual Machine designed to achieve maximum performance by removing all the "smart" features that actually only create *overhead*.
## 🛠 Why "Dumb"?
Most modern VMs are over-engineered. LightVM takes a shortcut:
 * **No Over-abstraction:** Langsung ke poinnya.
 * **Low Memory Footprint:** Gak makan RAM buat fitur yang gak lu pake.
 * **Instruction-focused:** Fokus ke eksekusi, bukan validasi yang berlebihan.
## 🚀 Getting Started
### Installation
Installation with NPM
```bash
npm install lightvm

# or
npm install lightvm@next
```
### Quick Usage
```typescript
import { LightVM } from 'lightvm';

const vm = new LightVM([/** Capability **/]);
```
## How to use
1. ``run()`` method:  
  Permission to start bytecode execution.
    ```typescript
    const raw = [
      ["push", 5],
      ["val", "x"],
      ["set", "x"]
    ];
    vm.load(vm.tools().optimizeBytecode(JSON.stringify(raw))) // or path to file .ltc
      .run(); // Capability: control
    ```
2. ``provide()`` method:  
  Permission to inject data/variables into the VM.
    ```typescript
    vm.provide("identity", {
      name: "John Doe", 
      force: "2021",
    }); // Capability: no specific capability
    ```
3. ``inspect()`` method:  
  Permission to view state, number of instructions, and capability.
    ```typescript
    const report = vm.inspect(); // Capability: observe
    console.log(report);
    ```
4. ``halt()`` method:  
  Permission to force/manually stop VM.
    ```typescript
    vm.halt(); // Capability: unsafe
    console.log("The VM has been terminated.")
    ```
5. ``export()`` method:  
  Permission to export functions in the VM out.
    ```typescript
    const add = vm.export("add"); // Capability: control
    console.log(add(5, 6));
    ```
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
| gt / lt   | type       |
| ge / le   | type       |
| eq / neq  | type       |
| and / or  | -          |
## Supported Architectures
LightVM supports a wide range of platforms and architectures to ensure maximum operational flexibility. Here's the current compatibility list:
| OS / Runtime | Architecture | Toolchain |
|--------------|--------------|-----------|
| Windows      | x64, ia32    | MSVC      |
| Linux        | x64, ia32  | GNU (glibc) |
| Linux (musl) | x64, ia32    | musl      |
| macOS (Darwin) | x64      | Apple Clang |
| Android      | arm64, arm   | NDK       |
## 📜 License
This project is distributed using the **Apache-2.0 license**
