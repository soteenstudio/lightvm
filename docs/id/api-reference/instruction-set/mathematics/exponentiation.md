# Eksponensiasi

## Tujuan

Menghitung perpangkatan skalar dan fungsi eksponensial natural.

## Instruksi yang didukung

`pow`, `powi`, `powf`, dan `exp`.

## Operan stack dan hasil

`pow`, `powi`, dan `powf` menggunakan basis di bawah eksponen lalu mengganti keduanya dengan satu hasil. `exp` mengganti nilai teratas dengan e pangkat nilai tersebut.

## Tipe numerik yang didukung

`pow` menerima semua tipe numerik, `powi` dan `powf` menerapkan konversi eksponen yang diimplementasikan, dan `exp` menerima `hlf`, `flt`, atau `dbl`.

## Batasan dan kondisi kegagalan

Operan yang tidak tersedia menghasilkan `StackUnderflow`. Nilai atau argumen tipe yang tidak didukung instruksi menghasilkan `TypeMismatch`.

## Contoh

Dengan `2` di bawah `3`, `pow int` menyisakan `8`. Dengan `0` di atas, `exp dbl` menyisakan `1`.

## Instruksi terkait

Lihat [Aritmatika Dasar](./basic-arithmetic) dan [Eksponensiasi Vektor](../vector-mathematics/exponentiation-vector).
