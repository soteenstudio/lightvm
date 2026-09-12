# LVM013 (Jump Limit Exceeded)
Runtime error type: `JumpLimitExceeded`.

## Cause

This error occurs when bytecode exceeds the permitted number of control-flow jumps, including `Jump`, `IfFalse`, and `Break` instructions.

## Runtime message

The runtime reports the instruction pointer where the limit was exceeded.

## Resolution

Simplify branching and nested loops. If the control-flow complexity is required, increase `max_jump` in `SecurityConfig`.
