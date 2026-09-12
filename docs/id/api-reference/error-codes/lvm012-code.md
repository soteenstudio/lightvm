# LVM012 (Batas Panggilan Terlampaui)
Tipe eror runtime: `CallLimitExceeded`.

## Penyebab

Error ini terjadi ketika bytecode melampaui jumlah instruksi pemanggilan fungsi yang diizinkan.

## Pesan runtime

Runtime melaporkan instruction pointer tempat batas terlampaui.

## Penyelesaian

Kurangi jumlah instruksi `Call` dalam bytecode. Jika pola panggilan tersebut sah, tingkatkan `max_call` dalam `SecurityConfig`.
