# Perbandingan dan Logika

## Tujuan

Membandingkan nilai skalar dan mengevaluasi truthiness.

## Instruksi yang didukung

`gt`, `lt`, `ge`, `le`, `eq`, `neq`, `and`, `or`, `xor`, dan `not`.

## Operan stack dan hasil

Instruksi biner menggunakan nilai kedua dari atas sebagai operan kiri dan nilai teratas sebagai operan kanan, lalu mendorong satu boolean. `not` mengganti nilai teratas dengan negasi boolean-nya.

## Tipe numerik yang didukung

Perbandingan menggunakan argumen tipe. Perbandingan berurutan mendukung tipe numerik; `eq` dan `neq` juga mendukung `str`. Instruksi logika tidak memiliki argumen tipe dan menggunakan truthiness nilai.

## Batasan dan kondisi kegagalan

Operan yang tidak tersedia menghasilkan `StackUnderflow`. Implementasi ini mengonversi nilai melalui tipe perbandingan yang dipilih dan tidak melaporkan `TypeMismatch`.

## Contoh

Dengan `2` di bawah `3`, `lt int` menyisakan `true`. Menerapkan `not` pada `true` menyisakan `false`.

## Instruksi terkait

Lihat [Aritmatika Dasar](./basic-arithmetic) dan [Operasi Bitwise](./bitwise-operations).
