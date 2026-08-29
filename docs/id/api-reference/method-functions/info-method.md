# Metode Info
Setelah menginisialisasi VM, Anda dapat mengambil metadata, versi sistem, dan status pembaruan tanpa perlu menjalankan siklus eksekusi apapun.

## Menggunakan TypeScript
Untuk **TypeScript**, Anda dapat memanggil metode info langsung pada instance VM untuk mengambil detail versi saat ini dan memeriksa pembaruan yang tertunda secara asinkron.

::: code-group

<<< @/examples/methodFunctions/infoKode.ts{ts:line-numbers}[Kode]

:::

## Menggunakan Rust
Dalam **Rust**, Anda dapat menquery instance VM yang telah diinisialisasi untuk memeriksa versi runtime saat ini dan ketersediaan pembaruan dengan memanggil metode `.info()`, yang mengembalikan sebuah struct `InfoVM`.

::: code-group

<<< @/examples/methodFunctions/info_Kode.rs{rust:line-numbers}[Kode]

:::
