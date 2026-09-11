# LVM016 (Batas Tick Terlampaui)
Tipe eror runtime: `TickLimitExceeded`.

## Penyebab

Error ini terjadi ketika eksekusi mencapai jumlah tick maksimum, yang mewakili unit kompleksitas atau waktu yang diizinkan oleh `SecurityConfig`.

## Pesan runtime

Runtime melaporkan instruction pointer `0`.

## Penyelesaian

Optimalkan program untuk mengurangi kompleksitas komputasinya. Jika beban kerja memang memerlukan lebih banyak pemrosesan, tingkatkan `max_ticks` dalam `SecurityConfig`.
