# LVM012 (Call Limit Exceeded)
Runtime error type: `CallLimitExceeded`.

## Cause

This error occurs when bytecode exceeds the permitted number of function-call instructions.

## Runtime message

The runtime reports the instruction pointer where the limit was exceeded.

## Resolution

Reduce the number of `Call` instructions in the bytecode. If the call pattern is legitimate, increase `max_call` in `SecurityConfig`.
