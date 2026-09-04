# LVM015 (Konfigurasi Max Ticks Tidak Valid)
Tipe eror runtime: `InvalidMaxTicksConfig`.

Error ini terjadi ketika `SecurityConfig` diinisialisasi dengan `max_ticks` bernilai `0`. Batas nol tidak valid karena akan memungkinkan eksekusi tanpa batas. Error ini melaporkan instruction pointer `0`.

Atur `max_ticks` dalam `SecurityConfig` ke bilangan bulat positif sebelum menginisialisasi VM.
