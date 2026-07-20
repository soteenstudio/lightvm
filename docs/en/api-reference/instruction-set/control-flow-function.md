# Control Flow & Function
These instructions manage the execution flow of the virtual machine, including branching, function scoping, and process termination. Note: Functions in LightVM operate within an isolated scope; local variables defined within a `func` block are destroyed upon the `return` instruction to ensure memory efficiency.

| Opcode   | Arguments | Operands (stack) | Description |
|----------|-----------|------------------|-------------|
| `jump`     | target_ip | - | Jump to a specific instruction line (Instruction Pointer) |
| `if_false` | target_ip | cond | Jump if the value on the stack is false |
| `func`     | name, argc, start, end, [params] | - | Function block definition (scope) |
| `call`     | name, argc | - | Call a function with a specified number of arguments |
| `return`   | -          | val | Exit the function and return to the caller |
| `stop`     | -          | - | Kill all VM processes (Halt) |