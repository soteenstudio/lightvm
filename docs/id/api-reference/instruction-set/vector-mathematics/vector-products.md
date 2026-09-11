# Hasil Kali Vektor

## Tujuan

Menghitung dot product dan cross product.

## Instruksi yang didukung

`dot` dan `cross`.

## Operan stack dan hasil

Keduanya menggunakan dua array dengan array kedua dari atas sebagai operan kiri. `dot` menyisakan skalar; `cross` menyisakan array.

## Tipe numerik yang didukung

Argumen tipe menerima `sht`, `int`, `lng`, `oct`, `hlf`, `flt`, atau `dbl`.

## Batasan dan kondisi kegagalan

`dot` memerlukan array dengan panjang sama. `cross` memerlukan dua array yang masing-masing tepat tiga elemen. Operan yang tidak tersedia menghasilkan `StackUnderflow`; array tidak valid, elemen nonnumerik, pelanggaran panjang, atau tipe yang tidak didukung menghasilkan `TypeMismatch`.

## Contoh

Menerapkan `dot int` pada `[1, 2]` di bawah `[3, 4]` menyisakan `11`.

## Instruksi terkait

Lihat [Aritmatika Vektor](./vector-arithmetic) dan [Trigonometri Vektor](./vector-trigonometry).
