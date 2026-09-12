# LVM009 (Import Limit Reached)
Runtime error type: `ImportLimitReached`.

## Cause

This error occurs when bytecode contains more module imports than `SecurityConfig` permits.

## Runtime message

The runtime reports the instruction pointer where the limit was exceeded.

## Resolution

Remove unused imports or consolidate modules. If more imports are required, increase `max_import` in `SecurityConfig`.
