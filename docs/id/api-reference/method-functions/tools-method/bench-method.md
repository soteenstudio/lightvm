# Metode Bench
Setelah menyiapkan lingkungan VM dan mempersiapkan rangkaian pengujian Anda, Anda dapat memanfaatkan metode `bench` untuk mengukur dan menganalisis performa eksekusi bytecode Anda dengan presisi tinggi.

## Menggunakan TypeScript
Untuk **TypeScript**, Anda dapat mendefinisikan logika benchmarking Anda dengan membungkus target eksekusi menggunakan rutinitas penyiapan (setup). Disarankan untuk menggunakan `optimizeBytecode` terlebih dahulu sebelum melakukan benchmarking guna memastikan Anda mengukur instruksi yang paling optimal.

::: code-group

<<< @/examples/methodFunctions/benchCode.ts{ts:line-numbers}[Code]

:::

## Menggunakan Rust
Dalam **Rust**, Anda dapat memanfaatkan alat `bench` melalui modul tools VM untuk mengonfigurasi ukuran byte, menginisialisasi state, dan menjalankan iterasi sampel adaptif. Selalu pastikan bytecode Anda dioptimalkan terlebih dahulu untuk mendapatkan metrik performa yang akurat dan dapat diandalkan.

::: code-group

<<< @/examples/methodFunctions/bench_code.rs{rust:line-numbers}[Code]

:::

::: info
**Kapabilitas yang Dibutuhkan**: tidak ada kapabilitas khusus
:::
