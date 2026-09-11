# Operasi Bitwise

## Tujuan

Menggeser atau merotasi bit bilangan bulat skalar.

## Instruksi yang didukung

`shl`, `shr`, `rol`, dan `ror`.

## Operan stack dan hasil

Setiap instruksi menggunakan nilai kedua dari atas sebagai nilai yang diubah dan nilai teratas sebagai jumlah pergeseran, lalu mengganti keduanya dengan satu hasil.

## Tipe numerik yang didukung

Argumen tipe menerima `sht`, `int`, `lng`, atau `oct`.

## Batasan dan kondisi kegagalan

Kurang dari dua operan menghasilkan `StackUnderflow`. Nilai non-integer dan argumen tipe yang tidak didukung menghasilkan `TypeMismatch`.

## Contoh

Dengan `1` di bawah `3`, `shl int` menyisakan `8`.

## Instruksi terkait

Lihat [Perbandingan dan Logika](./comparison-logic) dan [Operasi Bitwise Vektor](../vector-mathematics/bitwise-vector).
