# Aritmatika Dasar

## Tujuan

Melakukan aritmatika skalar dan memperbarui variabel numerik.

## Instruksi yang didukung

`add`, `sub`, `mul`, `div`, `mod`, `neg`, `inc`, dan `dec`.

## Operan stack dan hasil

Instruksi biner menggunakan nilai kedua dari atas sebagai operan kiri dan nilai teratas sebagai operan kanan, lalu menggantinya dengan satu hasil. `neg` mengganti nilai teratas. `inc` dan `dec` memperbarui variabel bernama atau berindeks; `inc` juga mendorong nilai terbaru.

## Tipe numerik yang didukung

Argumen tipe menerima `sht`, `int`, `lng`, `oct`, `hlf`, `flt`, atau `dbl`.

## Batasan dan kondisi kegagalan

Operan yang tidak tersedia menghasilkan `StackUnderflow`. Nilai atau argumen tipe yang tidak sesuai dengan operasi menghasilkan `TypeMismatch`.

## Contoh

Dengan `2` di bawah `3`, `add int` menyisakan `5`. Dengan `7` di bawah `2`, `sub int` menyisakan `5`.

## Instruksi terkait

Lihat [Eksponensiasi](./exponentiation) dan [Aritmatika Vektor](../vector-mathematics/vector-arithmetic).
