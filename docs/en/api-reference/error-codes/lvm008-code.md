# LVM008 (I/O Flood)

Runtime error type: `IoFlood`.

This error occurs when bytecode exceeds the permitted number of I/O operations, including operations such as print, println, stdout, and stdin. The runtime reports the instruction pointer where the limit was exceeded.

Reduce or consolidate I/O operations. If the usage is legitimate, increase `max_io` in `SecurityConfig`.
