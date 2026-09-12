# LVM016 (Tick Limit Exceeded)
Runtime error type: `TickLimitExceeded`.

## Cause

This error occurs when execution reaches the maximum number of ticks, which represent complexity or time units permitted by `SecurityConfig`.

## Runtime message

The runtime reports instruction pointer `0`.

## Resolution

Optimize the program to reduce its computational complexity. If the workload legitimately needs more processing, increase `max_ticks` in `SecurityConfig`.
