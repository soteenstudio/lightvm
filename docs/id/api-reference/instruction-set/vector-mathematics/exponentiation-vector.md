# Eksponensiasi Vektor

## Tujuan

Menghitung perpangkatan dan eksponensial natural secara element-wise.

## Instruksi yang didukung

`powv`, `powiv`, `powfv`, dan `expv`.

## Operan stack dan hasil

Instruksi perpangkatan menggunakan array basis dan eksponen berukuran sama lalu menyisakan satu array hasil. `expv` mengganti array teratas.

## Tipe numerik yang didukung

`powv`, `powiv`, dan `powfv` menerima kombinasi tipe numerik yang diimplementasikan. `expv` menerima `hlf`, `flt`, atau `dbl`.

## Batasan dan kondisi kegagalan

Array perpangkatan harus memiliki panjang sama. Operan yang tidak tersedia menghasilkan `StackUnderflow`; array tidak valid, elemen nonnumerik, panjang berbeda, atau tipe yang tidak didukung menghasilkan `TypeMismatch`.

## Contoh

Menerapkan `powv int` pada `[2, 3]` di bawah `[3, 2]` menyisakan `[8, 9]`.

## Instruksi terkait

Lihat [Eksponensiasi](../mathematics/exponentiation) dan [Logaritma Vektor](./logarithm-vector).
