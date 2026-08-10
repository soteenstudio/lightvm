# Metode Compile
Setelah menginisialisasi VM dan menyiapkan lingkungan Anda, Anda dapat memproses dan mengompilasi konfigurasi bytecode Anda menjadi biner target.

## Menggunakan TypeScript
Untuk **TypeScript**, Anda dapat meneruskan array instruksi mentah langsung ke loader, menerapkan alat optimasi, dan menyiapkan konfigurasi kompilasi Anda dengan mulus.

::: code-group

<<< @/examples/methodFunctions/compileWithArray.ts{ts:line-numbers}[Dengan Array]

:::

## Menggunakan Rust
Dalam **Rust**, Anda biasanya bekerja dengan string instruksi mentah, mengoptimalkannya menggunakan alat bantu, dan meneruskan struktur `CompileConfig` yang berisi `TargetArch` dan `FileType` langsung ke metode `.compile()`.

::: code-group

<<< @/examples/methodFunctions/compile_with_raw_string.rs{rust:line-numbers}[Dengan String Mentah]

<<< @/examples/methodFunctions/compile_with_serde.rs{rust:line-numbers}[Dengan Serde]

:::

## Arsitektur Target
Berikut adalah status kompilasi untuk arsitektur perangkat keras yang didukung:

| Arsitektur | Status | Kompilasi |
| :--- | :--- | :--- |
| `AArch64` | nightly | ✓ |

## Jenis File
Ringkasan format file yang didukung dan digunakan di seluruh *pipeline*:

| Jenis | Deskripsi |
| :--- | :--- |
| `Assembly` | Representasi teks yang mudah dibaca manusia dari instruksi mesin tingkat rendah, berfungsi sebagai langkah perantara sebelum pembuatan *bytecode* akhir atau kode mesin. |
| `Binary` | Format eksekusi siap-mesin yang dikompilasi, terdiri dari *byte* mentah dan *opcode* yang dirancang untuk dieksekusi langsung oleh *runtime* atau perangkat keras. |

::: info
**Kapabilitas yang Dibutuhkan**: `Control`
:::
