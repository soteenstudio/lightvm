# Advanced Stack & Memory Management
These instructions manage the evaluation stack's memory footprint and capacity. They provide fine-grained control over resource allocation, helping to optimize performance and prevent memory fragmentation within the virtual machine.

| Opcode | Arguments | Operands (stack) | Description |
| :--- | :--- | :--- | :--- |
| `init_stack` | size | - | Initialize the evaluation stack memory capacity at the beginning of VM execution to prevent reallocation |
| `shrink` | - | target, length | Reduces the capacity of the stack to fit its current length |
| `truncate` | - | target_size | Clear/reset the stack elements efficiently |