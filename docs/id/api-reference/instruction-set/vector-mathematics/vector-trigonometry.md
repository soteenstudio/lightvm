# Trigonometri Vektor

## Tujuan

Menghitung fungsi trigonometri, invers, dan hiperbolik secara element-wise.

## Instruksi yang didukung

`sinv`, `cosv`, `tanv`, `asinv`, `acosv`, `atanv`, `atan2v`, `sinhv`, `coshv`, `tanhv`, `asinhv`, `acoshv`, dan `atanhv`.

## Operan stack dan hasil

Instruksi uner mengganti array teratas. `atan2v` menggunakan dua array dengan panjang sama dan menyisakan satu array hasil.

## Tipe numerik yang didukung

Argumen tipe menerima `hlf`, `flt`, atau `dbl`.

## Batasan dan kondisi kegagalan

`atan2v` memerlukan array dengan panjang sama. Operan yang tidak tersedia menghasilkan `StackUnderflow`; array tidak valid, elemen nonnumerik, panjang berbeda, atau tipe yang tidak didukung menghasilkan `TypeMismatch`. Error domain mengikuti perilaku floating-point.

## Contoh

Menerapkan `sinv dbl` pada `[0, 0]` menyisakan `[0, 0]`.

## Instruksi terkait

Lihat [Trigonometri](../mathematics/trigonometry) dan [Hasil Kali Vektor](./vector-products).
