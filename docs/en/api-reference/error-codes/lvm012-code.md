# LVM012 (Call Limit Exceeded)

Runtime error type: `CallLimitExceeded`.

This error occurs when bytecode exceeds the permitted number of function-call instructions. The runtime reports the instruction pointer where the limit was exceeded.

Reduce recursion depth or function-call frequency. If the call pattern is legitimate, increase `max_call` in `SecurityConfig`.
