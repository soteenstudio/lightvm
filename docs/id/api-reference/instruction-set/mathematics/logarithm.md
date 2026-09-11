# Logaritma

## Tujuan

Menghitung fungsi logaritma skalar.

## Instruksi yang didukung

`ln`, `log2`, dan `log10`.

## Operan stack dan hasil

Setiap instruksi mengganti nilai teratas stack dengan logaritmanya.

## Tipe numerik yang didukung

Argumen tipe menerima `hlf`, `flt`, atau `dbl`.

## Batasan dan kondisi kegagalan

Stack kosong menghasilkan `StackUnderflow`. Nilai nonnumerik atau argumen tipe yang tidak didukung menghasilkan `TypeMismatch`. Error domain mengikuti perilaku floating-point.

## Contoh

Dengan `1` di atas, `ln dbl` menyisakan `0`.

## Instruksi terkait

Lihat [Eksponensiasi](./exponentiation) dan [Logaritma Vektor](../vector-mathematics/logarithm-vector).
