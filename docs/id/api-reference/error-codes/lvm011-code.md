# LVM011 (Batas Memori Terlampaui)
Tipe eror runtime: `MemoryLimitExceeded`.

## Penyebab

Error ini terjadi ketika bytecode melampaui jumlah alokasi memori yang diizinkan, termasuk operasi `MakeObj` dan `MakeArray`.

## Pesan runtime

Runtime melaporkan instruction pointer tempat batas terlampaui.

## Penyelesaian

Kurangi alokasi objek dan array atau gunakan kembali nilai yang ada. Jika alokasi tersebut diperlukan, tingkatkan `max_alloc` dalam `SecurityConfig`.
