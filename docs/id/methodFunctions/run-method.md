# Metode Run
Setelah menginisialisasi VM dan mengkonfigurasi kapabilitas yang Anda inginkan, Anda dapat memuat bytecode dan memulai proses eksekusi.

## Menggunakan TypeScript
Untuk **TypeScript**, Anda dapat meneruskan array bytecode mentah langsung ke loader. Disarankan untuk menggunakan `optimizeBytecode` terlebih dahulu untuk memastikan VM menjalankan versi instruksi Anda yang paling efisien.

::: code-group

<<< @/examples/methodFunctions/runWithArray.ts{ts:line-numbers}[Dengan Array]

:::

## Menggunakan Rust
Dalam **Rust**, Anda biasanya bekerja dengan string bytecode yang diserialisasi (atau `serde_json`). Mirip dengan implementasi TypeScript, selalu optimalkan bytecode Anda sebelum memuatnya ke dalam VM untuk mempertahankan kinerja optimal.

::: code-group

<<< @/examples/methodFunctions/run_with_raw_string.rs{rust:line-numbers}[Dengan String Mentah]

<<< @/examples/methodFunctions/run_with_serde.rs{rust:line-numbers}[Dengan Serde]

:::

::: tip
Metode ``.run()`` adalah langkah terakhir dalam alur eksekusi. Pastikan semua `kapabilitas` yang diperlukan telah diberikan selama inisialisasi untuk menghindari pengecualian keamanan saat runtime.
:::
