# Akar

## Tujuan

Menghitung akar kuadrat dan akar pangkat tiga skalar.

## Instruksi yang didukung

`sqrt` dan `cbrt`.

## Operan stack dan hasil

Setiap instruksi mengganti nilai teratas stack dengan hasil akarnya.

## Tipe numerik yang didukung

Argumen tipe menerima `hlf`, `flt`, atau `dbl`.

## Batasan dan kondisi kegagalan

Stack kosong menghasilkan `StackUnderflow`. Nilai nonnumerik atau argumen tipe yang tidak didukung menghasilkan `TypeMismatch`. Akar kuadrat negatif mengikuti perilaku floating-point.

## Contoh

Dengan `9` di atas, `sqrt dbl` menyisakan `3`.

## Instruksi terkait

Lihat [Eksponensiasi](./exponentiation) dan [Akar Vektor](../vector-mathematics/root-vector).
