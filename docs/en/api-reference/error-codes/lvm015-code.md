# LVM015 (Invalid Max Ticks Config)
Runtime error type: `InvalidMaxTicksConfig`.

## Cause

This error occurs when `SecurityConfig` is initialized with `max_ticks` set to `0`. A zero limit is invalid because it would allow unbounded execution.

## Runtime message

The runtime reports instruction pointer `0`.

## Resolution

Set `max_ticks` in `SecurityConfig` to a positive integer before initializing the VM.
