# Metode Export <Badge type="warning" text="Preview" />
Metode `export` memungkinkan Anda untuk mengekstrak fungsi atau logika spesifik yang terdaftar di dalam lingkungan VM, dan mengeksposnya ke aplikasi host Anda. Ini secara efektif menjembatani kesenjangan antara VM dan aplikasi host Anda. 

## Menggunakan TypeScript
Untuk **TypeScript**, metode `export` menyediakan cara mudah untuk mengambil dan memanggil fungsi yang didefinisikan di dalam VM. Dengan mengekspor fungsi, Anda dapat memperlakukan logika sisi VM sebagai kode asli, sehingga menyederhanakan proses.

::: code-group

<<< @/examples/methodFunctions/exportFunction.ts{ts:line-numbers}[Export Function]

<<< @/examples/methodFunctions/exportVariable.ts{ts:line-numbers}[Export Variable]

:::

## Menggunakan Rust
Dalam **Rust**, metode `export` digunakan untuk memetakan fungsi sisi VM ke antarmuka yang dapat dipanggil oleh Rust. Mekanisme ini memungkinkan Anda untuk memanfaatkan hasil eksekusi VM secara langsung di dalam aplikasi Rust utama Anda.

::: code-group

<<< @/examples/methodFunctions/export_function.rs{rust:line-numbers}[Export Function]

<<< @/examples/methodFunctions/export_variable.rs{rust:line-numbers}[Export Variable]

:::

::: info
**Kapabilitas yang Dibutuhkan**: `Control`
:::