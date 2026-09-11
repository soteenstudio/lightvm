# Logaritma Vektor

## Tujuan

Menghitung logaritma secara element-wise.

## Instruksi yang didukung

`lnv`, `log2v`, dan `log10v`.

## Operan stack dan hasil

Setiap instruksi mengganti array teratas dengan satu array hasil.

## Tipe numerik yang didukung

Argumen tipe menerima `hlf`, `flt`, atau `dbl`.

## Batasan dan kondisi kegagalan

Operan yang tidak tersedia menghasilkan `StackUnderflow`. Array tidak valid, elemen nonnumerik, atau tipe yang tidak didukung menghasilkan `TypeMismatch`. Error domain mengikuti perilaku floating-point.

## Contoh

Menerapkan `lnv dbl` pada `[1, 1]` menyisakan `[0, 0]`.

## Instruksi terkait

Lihat [Logaritma](../mathematics/logarithm) dan [Eksponensiasi Vektor](./exponentiation-vector).
