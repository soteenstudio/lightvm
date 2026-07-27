# Metode Parse Array
Metode `parseLTCArray` dirancang untuk mengubah string assembly LightVM yang dapat dibaca manusia menjadi array JSON terstruktur yang dapat diuraikan oleh mesin. Meskipun metode `parse` standar menangani penerjemahan umum, `parseLTCArray` secara khusus disesuaikan untuk skenario di mana Anda memerlukan akses langsung ke struktur bytecode yang diserialisasi, menjadikannya ideal untuk debugging, analisis statis, atau membuat alat bantu eksternal.

## Menggunakan TypeScript
Untuk **TypeScript**, metode `parseLTCArray` mengonversi logika assembly Anda ke dalam format array JSON standar. Ini sangat berguna jika Anda membangun alat bantu frontend, dashboard, atau sekadar perlu memeriksa urutan opcode Anda dalam format objek JavaScript yang familier sebelum runtime.

::: code-group

<<< @/examples/methodFunctions/toolsMethod/parseLTCArrayCode.ts{ts:line-numbers}[Kode]

:::

## Menggunakan Rust
Dalam **Rust**, metode ini menyediakan cara yang mulus untuk mengubah string instruksi mentah menjadi format bytecode yang diserialisasi. Ini sempurna untuk membuat utilitas CLI atau pipeline pembangunan kustom di mana Anda perlu membuat, memvalidasi, atau memproses array instruksi secara terprogram.

::: code-group

<<< @/examples/methodFunctions/toolsMethod/parse_array_code.rs{rust:line-numbers}[Kode]

:::

::: info
**Kapabilitas yang Dibutuhkan**: tidak ada kapabilitas khusus
:::
