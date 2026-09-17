# Metode Clear Paniclog
Metode `clearPaniclog` menghapus semua catatan panic yang tersimpan.

## Menggunakan TypeScript
Untuk **TypeScript**, panggil `.clearPaniclog()` pada VM yang dikonfigurasi dengan kapabilitas `Debug`. Metode ini selesai setelah catatan dihapus dan melempar error jika operasi gagal.

::: code-group

<<< @/examples/methodFunctions/clearPaniclogCode.ts{ts:line-numbers}[Kode]

:::

## Menggunakan Rust
Dalam **Rust**, panggil `.clear_paniclog()` pada VM yang dikonfigurasi dengan kapabilitas `Debug`. Metode ini mengembalikan `Result<(), VMError>`.

::: code-group

<<< @/examples/methodFunctions/clear_paniclog_code.rs{rust:line-numbers}[Kode]

:::

::: info
**Kapabilitas yang Dibutuhkan**: `Debug`
:::

::: warning
Metode `.clearPaniclog()` dan `.clear_paniclog()` tidak tersedia dalam build WebAssembly.
:::
