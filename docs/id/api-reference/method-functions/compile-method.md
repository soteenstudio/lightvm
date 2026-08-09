# Metode Compile
Setelah menginisialisasi VM dan menyiapkan lingkungan Anda, Anda dapat memproses dan mengompilasi konfigurasi bytecode Anda menjadi biner target.

## Menggunakan TypeScript
Untuk **TypeScript**, Anda dapat meneruskan array instruksi mentah langsung ke loader, menerapkan alat optimasi, dan menyiapkan konfigurasi kompilasi Anda dengan mulus.

::: code-group

<<< @/examples/methodFunctions/compileWithArray.ts{ts:line-numbers}[With Array]

:::

## Menggunakan Rust
Dalam **Rust**, Anda biasanya bekerja dengan string instruksi mentah, mengoptimalkannya menggunakan alat bantu, dan meneruskan struktur `CompileConfig` yang berisi `TargetArch` dan `FileType` langsung ke metode `.compile()`.

::: code-group

<<< @/examples/methodFunctions/compile_with_raw_string.rs{rust:line-numbers}[With Raw String]

<<< @/examples/methodFunctions/compile_with_serde.rs{rust:line-numbers}[With Serde]

:::

::: info
**Kapabilitas yang Dibutuhkan**: `Control`
:::
