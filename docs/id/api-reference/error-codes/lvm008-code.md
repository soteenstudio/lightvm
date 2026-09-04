# LVM008 (Banjir I/O)
Runtime error type: `IoFlood`.

Error ini terjadi ketika bytecode melampaui jumlah operasi I/O yang diizinkan, termasuk operasi seperti print, println, stdout, dan stdin. Runtime melaporkan instruction pointer tempat batas terlampaui.

Kurangi atau gabungkan operasi I/O. Jika penggunaannya sah, tingkatkan `max_io` dalam `SecurityConfig`.
