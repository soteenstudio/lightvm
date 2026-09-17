# Metode Clear Paniclog

Metode `LightVM.clearPaniclog()` menghapus semua catatan paniclog yang tersimpan. API ini membutuhkan `Capability.Debug` dan tidak tersedia dalam build WebAssembly.

## Menggunakan TypeScript

Wrapper TypeScript memanggil metode `clearPaniclog()` Node N-API. Metode ini kembali secara normal setelah catatan dihapus dan mengikuti perilaku N-API dengan melempar error saat gagal.

::: code-group

<<< @/examples/methodFunctions/clearPaniclogCode.ts{ts:line-numbers}[Kode]

:::

## Menggunakan Rust

Metode native Rust yang setara adalah `LightVM::clear_paniclog()`, yang mengembalikan `Result<(), VMError>`. Saat gagal, antarmuka native mencetak `VMError` yang telah diformat dan mengembalikan error yang sama.

::: code-group

<<< @/examples/methodFunctions/clear_paniclog_code.rs{rust:line-numbers}[Kode]

:::

## Menggunakan Node N-API

Metode `clearPaniclog()` Node N-API menghapus catatan yang tersimpan saat berhasil dan melempar error N-API saat otorisasi atau penyimpanan gagal.

::: info
**Kapabilitas yang Dibutuhkan**: `Capability.Debug`
:::
