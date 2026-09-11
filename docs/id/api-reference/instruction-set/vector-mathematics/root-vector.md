# Akar Vektor

## Tujuan

Menghitung akar kuadrat dan akar pangkat tiga secara element-wise.

## Instruksi yang didukung

`sqrtv` dan `cbrtv`.

## Operan stack dan hasil

Setiap instruksi mengganti array teratas dengan satu array hasil.

## Tipe numerik yang didukung

Argumen tipe menerima `hlf`, `flt`, atau `dbl`.

## Batasan dan kondisi kegagalan

Operan yang tidak tersedia menghasilkan `StackUnderflow`. Array tidak valid, elemen nonnumerik, atau tipe yang tidak didukung menghasilkan `TypeMismatch`. Akar kuadrat negatif mengikuti perilaku floating-point.

## Contoh

Menerapkan `sqrtv dbl` pada `[4, 9]` menyisakan `[2, 3]`.

## Instruksi terkait

Lihat [Akar](../mathematics/root) dan [Eksponensiasi Vektor](./exponentiation-vector).
