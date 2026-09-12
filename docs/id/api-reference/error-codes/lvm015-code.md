# LVM015 (Konfigurasi Max Ticks Tidak Valid)
Tipe eror runtime: `InvalidMaxTicksConfig`.

## Penyebab

Error ini terjadi ketika `SecurityConfig` diinisialisasi dengan `max_ticks` bernilai `0`. Batas nol tidak valid karena akan memungkinkan eksekusi tanpa batas.

## Pesan runtime

Runtime melaporkan instruction pointer `0`.

## Penyelesaian

Atur `max_ticks` dalam `SecurityConfig` ke bilangan bulat positif sebelum menginisialisasi VM.
