# LVM016 (Batas Tick Terlampaui)
Runtime error type: `TickLimitExceeded`.

Error ini terjadi ketika eksekusi mencapai jumlah tick maksimum, yang mewakili unit kompleksitas atau waktu yang diizinkan oleh `SecurityConfig`. Error ini melaporkan instruction pointer `0`.

Optimalkan program untuk mengurangi kompleksitas komputasinya. Jika beban kerja memang memerlukan lebih banyak pemrosesan, tingkatkan `max_ticks` dalam `SecurityConfig`.
