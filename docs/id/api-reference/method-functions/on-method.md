# Metode On <Badge type="warning" text="Nightly" />
Metode `on` adalah inti berbasis peristiwa dari `LightVM`, yang memungkinkan Anda untuk terhubung ke peristiwa siklus hidup VM tertentu. Dengan mendaftarkan callback, Anda dapat bereaksi terhadap perubahan status internal dan tonggak eksekusi.

## Menggunakan TypeScript
Untuk **TypeScript**, metode `on` memungkinkan Anda untuk melampirkan pendengar peristiwa yang akan dieksekusi setiap kali peristiwa terdaftar dipicu. Ini ideal untuk pencatatan log, memantau perubahan status VM, atau memicu eksekusi.

::: code-group

<<< @/examples/methodFunctions/onCode.ts{ts:line-numbers}[Kode]

:::

## Menggunakan Rust
Dalam **Rust**, metode `on` memungkinkan Anda untuk mendefinisikan penanganan peristiwa yang terintegrasi langsung dengan logika sistem Anda. Ini memungkinkan pemberitahuan asinkron yang kuat ketika VM memasuki keadaan tertentu,

::: code-group

<<< @/examples/methodFunctions/on_code.rs{rust:line-numbers}[Kode]

:::

::: info
**Kapabilitas yang Dibutuhkan**: tidak ada kapabilitas khusus

**Event yang Dimiliki**: `tick`, `halt`, dan `panic`
:::