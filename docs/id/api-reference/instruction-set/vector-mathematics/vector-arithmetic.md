# Aritmatika Vektor

## Tujuan

Melakukan aritmatika element-wise pada array.

## Instruksi yang didukung

`addv`, `subv`, `mulv`, `divv`, `modv`, dan `negv`.

## Operan stack dan hasil

Instruksi biner menggunakan dua array dengan array kedua dari atas sebagai operan kiri, lalu menggantinya dengan satu array hasil. `negv` mengganti array teratas.

## Tipe numerik yang didukung

Argumen tipe menerima `sht`, `int`, `lng`, `oct`, `hlf`, `flt`, atau `dbl`.

## Batasan dan kondisi kegagalan

Operan biner harus berupa array dengan panjang sama. Operan yang tidak tersedia menghasilkan `StackUnderflow`; elemen nonnumerik, array tidak valid, panjang berbeda, atau argumen tipe yang tidak didukung menghasilkan `TypeMismatch`.

## Contoh

Menerapkan `addv int` pada `[1, 2]` di bawah `[3, 4]` menyisakan `[4, 6]`.

## Instruksi terkait

Lihat [Aritmatika Dasar](../mathematics/basic-arithmetic) dan [Hasil Kali Vektor](./vector-products).
