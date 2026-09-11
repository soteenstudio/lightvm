# LVM001 (Stack Overflow)
Runtime error type: `StackOverflow`.

## Cause

The stack capacity can be configured before execution by using `setMaxStackSize(128)` or `set_max_stack_size(128)`, or by passing `0` to keep the default capacity.

The `val` instruction also raises `StackOverflow` when its variable index reaches the fixed variable limit. The `make_obj` instruction raises the same error when its requested object capacity exceeds the implementation limit.

## Runtime message

The runtime message reports `limit`, and the runtime records the instruction pointer where `StackOverflow` occurred.

## Resolution

Reduce unintended stack growth, including unbounded recursion. If the workload requires a larger stack, configure `SecurityConfig.max_stack_size`. Keep variable indexes and object-capacity requests within their supported limits.
