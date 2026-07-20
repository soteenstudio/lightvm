# Metode Parse
Metode `parse` adalah kebalikan dari `stringify`. Metode ini memungkinkan Anda untuk mengkonversi string instruksi LightVM yang dapat dibaca manusia kembali menjadi struktur bytecode yang dapat dieksekusi. Ini sangat penting ketika Anda ingin menulis

## Menggunakan TypeScript
Untuk **TypeScript**, Anda dapat langsung meneruskan string instruksi mentah Anda ke metode `parse`. Ini akan menerjemahkan kode Anda ke dalam format bytecode terstruktur yang dibutuhkan oleh VM.

::: code-group

<<< @/examples/methodFunctions/toolsMethod/parseCode.ts{ts:line-numbers}[Kode]

:::

## Menggunakan Rust
Dalam **Rust**, metode `parse` memungkinkan Anda untuk mengambil string instruksi mentah dan mengubahnya menjadi representasi VM internal, sehingga memudahkan untuk menjembatani skrip yang ditulis manusia dengan VM berbasis Rust Anda. 

::: code-group

<<< @/examples/methodFunctions/toolsMethod/parse_code.rs{rust:line-numbers}[Kode]

:::

::: info
**Kapabilitas yang Dibutuhkan**: tidak ada kapabilitas khusus
:::
