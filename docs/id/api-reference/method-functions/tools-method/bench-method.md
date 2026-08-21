# Metode Bench
Setelah menyiapkan lingkungan VM dan mempersiapkan rangkaian pengujian Anda, Anda dapat memanfaatkan metode `bench` untuk mengukur dan menganalisis performa eksekusi bytecode Anda dengan presisi tinggi.

## Menggunakan TypeScript
Untuk **TypeScript**, Anda dapat mendefinisikan logika benchmarking Anda dengan membungkus target eksekusi menggunakan rutinitas penyiapan (setup). Disarankan untuk menggunakan `optimizeBytecode` terlebih dahulu sebelum melakukan benchmarking guna memastikan Anda mengukur instruksi yang paling optimal.

Konfigurasi benchmark menerima parameter berikut:
- `targetTime`: Durasi target untuk eksekusi benchmark dalam **milidetik** (harus lebih besar dari nol)
- `bytes`: Jumlah byte yang diproses **per iterasi** (opsional, digunakan untuk perhitungan throughput)
- `samples`: Jumlah iterasi sampel yang dikumpulkan (harus lebih besar dari nol)

::: code-group

<<< @/examples/methodFunctions/toolsMethod/benchCode.ts{ts:line-numbers}[Kode]

:::

## Menggunakan Rust
Dalam **Rust**, Anda dapat memanfaatkan alat `bench` melalui modul tools VM untuk mengonfigurasi ukuran byte, menginisialisasi state, dan menjalankan iterasi sampel adaptif. Selalu pastikan bytecode Anda dioptimalkan terlebih dahulu untuk mendapatkan metrik performa yang akurat dan dapat diandalkan.

Konfigurasi benchmark menerima parameter berikut:
- `target_time(Duration)`: Durasi target untuk eksekusi benchmark (harus lebih besar dari nol)
- `bytes`: Jumlah byte yang diproses **per iterasi** (opsional, digunakan untuk perhitungan throughput)
- `samples`: Jumlah iterasi sampel yang dikumpulkan (harus lebih besar dari nol)

::: code-group

<<< @/examples/methodFunctions/toolsMethod/bench_code.rs{rust:line-numbers}[Kode]

:::

::: info
**Kapabilitas yang Dibutuhkan**: `Debug`
:::
