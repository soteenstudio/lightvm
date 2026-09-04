# LVM015 (Invalid Max Ticks Config)

Runtime error type: `InvalidMaxTicksConfig`.

This error occurs when `SecurityConfig` is initialized with `max_ticks` set to `0`. A zero limit is invalid because it would allow unbounded execution. This error reports instruction pointer `0`.

Set `max_ticks` in `SecurityConfig` to a positive integer before initializing the VM.
