# Advanced Stack & Memory Management
These instructions manage the evaluation stack's memory footprint and capacity. They provide fine-grained control over resource allocation, helping to optimize performance and prevent memory fragmentation within the virtual machine.

| Opcode | Arguments | Operands (stack) | Description |
| :--- | :--- | :--- | :--- |
| `shrink` | - | target, length | Reduces the capacity of the stack to fit its current length |
| `truncate` | - | target_size | Clear/reset the stack elements efficiently |