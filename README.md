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
  vm.load([
    ["val", "x"],
    ["push", 5],
    ["set", "x"]
    ]) // or path to file .ltc
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
5. ``halt()`` method:
  Permission to force/manually stop VM.
  ```typescript
  const add = vm.export("add"); // Capability: control
  console.log(add(5, 6));
  ```
## 📜 License
This project is distributed using the **Apache-2.0 license**
