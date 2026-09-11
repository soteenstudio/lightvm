# LVM009 (Batas Impor Tercapai)
Tipe eror runtime: `ImportLimitReached`.

## Penyebab

Error ini terjadi ketika bytecode berisi lebih banyak impor modul daripada yang diizinkan oleh `SecurityConfig`.

## Pesan runtime

Runtime melaporkan instruction pointer tempat batas terlampaui.

## Penyelesaian

Hapus impor yang tidak digunakan atau gabungkan modul. Jika diperlukan lebih banyak impor, tingkatkan `max_import` dalam `SecurityConfig`.
