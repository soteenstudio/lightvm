# LVM008 (Banjir I/O)
Tipe eror runtime: `IoFlood`.

## Penyebab

Error ini terjadi ketika bytecode melampaui jumlah operasi I/O yang diizinkan, termasuk operasi seperti print, println, stdout, dan stdin.

## Pesan runtime

Runtime melaporkan instruction pointer tempat batas terlampaui.

## Penyelesaian

Kurangi atau gabungkan operasi I/O. Jika penggunaannya sah, tingkatkan `max_io` dalam `SecurityConfig`.
