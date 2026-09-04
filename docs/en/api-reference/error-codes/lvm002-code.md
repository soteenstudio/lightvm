# LVM002 (Stack Underflow)
Runtime error type: `StackUnderflow`.

This error occurs when an instruction attempts to pop a value from an empty stack. The runtime message identifies the instruction with `opcode` and reports the instruction pointer where it failed.

Check the bytecode's push and pop balance. Ensure every value consumed by the reported opcode is pushed first and that earlier control-flow paths leave the stack in a consistent state.
