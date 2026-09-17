# Metode Paniclog
Metode `paniclog` mengembalikan catatan panic yang tersimpan sebagai array terstruktur. Setiap catatan berisi konteks diagnostik dan status VM yang dibatasi. Hindari memasukkan nilai sensitif ke dalam konteks diagnostik.

## Menggunakan TypeScript
Untuk **TypeScript**, panggil `.paniclog()` pada VM yang dikonfigurasi dengan kapabilitas `Debug`. Metode ini mengembalikan array catatan dan melempar error jika catatan tidak dapat diambil atau diuraikan.

::: code-group

<<< @/examples/methodFunctions/paniclogCode.ts{ts:line-numbers}[Kode]

:::

## Menggunakan Rust
Dalam **Rust**, panggil `.paniclog()` pada VM yang dikonfigurasi dengan kapabilitas `Debug`. Metode ini mengembalikan `Result<serde_json::Value, VMError>`, dengan nilai JSON berupa array catatan panic.

::: code-group

<<< @/examples/methodFunctions/paniclog_code.rs{rust:line-numbers}[Kode]

:::

::: info
**Kapabilitas yang Dibutuhkan**: `Debug`
:::

::: warning
Metode `.paniclog()` tidak tersedia dalam build WebAssembly.
:::
