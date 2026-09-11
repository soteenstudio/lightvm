# LVM011 (Memory Limit Exceeded)
Runtime error type: `MemoryLimitExceeded`.

## Cause

This error occurs when bytecode exceeds the permitted number of memory allocations, including `MakeObj` and `MakeArray` operations.

## Runtime message

The runtime reports the instruction pointer where the limit was exceeded.

## Resolution

Reduce object and array allocations or reuse existing values. If the allocations are justified, increase `max_alloc` in `SecurityConfig`.
