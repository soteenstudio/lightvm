# Operasi Bitwise Vektor

## Tujuan

Menggeser atau merotasi elemen integer yang bersesuaian dalam dua array.

## Instruksi yang didukung

`shlv`, `shrv`, `rolv`, dan `rorv`.

## Operan stack dan hasil

Setiap instruksi menggunakan dua array dengan array kedua dari atas sebagai nilai dan array teratas sebagai jumlah pergeseran, lalu menyisakan satu array hasil.

## Tipe numerik yang didukung

Argumen tipe menerima `sht`, `int`, `lng`, atau `oct`.

## Batasan dan kondisi kegagalan

Array harus memiliki panjang sama dan elemen numerik. Operan yang tidak tersedia menghasilkan `StackUnderflow`; array tidak valid, panjang berbeda, elemen nonnumerik, atau tipe yang tidak didukung menghasilkan `TypeMismatch`.

## Contoh

Menerapkan `shlv int` pada `[1, 2]` di bawah `[1, 2]` menyisakan `[2, 8]`.

## Instruksi terkait

Lihat [Operasi Bitwise](../mathematics/bitwise-operations) dan [Aritmatika Vektor](./vector-arithmetic).
