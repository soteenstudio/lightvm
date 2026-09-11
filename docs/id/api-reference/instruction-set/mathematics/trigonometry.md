# Trigonometri

## Tujuan

Menghitung fungsi trigonometri skalar, invers, dan hiperbolik.

## Instruksi yang didukung

`sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, dan `atanh`.

## Operan stack dan hasil

Instruksi uner mengganti nilai teratas. `atan2` menggunakan nilai kedua dari atas sebagai operan pertama dan nilai teratas sebagai operan kedua, lalu menyisakan satu hasil.

## Tipe numerik yang didukung

Argumen tipe menerima `hlf`, `flt`, atau `dbl`.

## Batasan dan kondisi kegagalan

Operan yang tidak tersedia menghasilkan `StackUnderflow`. Nilai nonnumerik atau argumen tipe yang tidak didukung menghasilkan `TypeMismatch`. Error domain mengikuti perilaku floating-point.

## Contoh

Dengan `0` di atas, `sin dbl` menyisakan `0`.

## Instruksi terkait

Lihat [Akar](./root) dan [Trigonometri Vektor](../vector-mathematics/vector-trigonometry).
