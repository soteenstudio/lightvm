# LVM001 (Stack Overflow)

Runtime error type: `StackOverflow`.

This error occurs when the stack reaches the maximum limit specified by `InitStack` or by the default stack limit. The runtime message reports the configured `limit` and the instruction pointer where the limit was reached.

Check for an infinitely recursive function call. If the required stack depth is valid, configure `InitStack` to reserve sufficient stack space.
