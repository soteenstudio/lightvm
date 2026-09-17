# Metode Paniclog

Metode `LightVM.paniclog()` mengembalikan catatan paniclog yang tersimpan sebagai array terstruktur. Setiap catatan berisi konteks diagnostik dan status VM yang dibatasi; hindari memasukkan nilai sensitif ke dalam konteks diagnostik.

API ini membutuhkan `Capability.Debug` dan tidak tersedia dalam build WebAssembly.

## Menggunakan TypeScript

Wrapper TypeScript memanggil metode `paniclog()` Node N-API. Metode ini mengembalikan array catatan saat berhasil dan mengikuti perilaku N-API dengan melempar error saat gagal.

::: code-group

<<< @/examples/methodFunctions/paniclogCode.ts{ts:line-numbers}[Kode]

:::

## Menggunakan Rust

Metode native Rust `LightVM::paniclog()` mengembalikan `Result<serde_json::Value, VMError>`. Nilai JSON tersebut berupa array saat berhasil. Saat gagal, antarmuka native mencetak `VMError` yang telah diformat dan mengembalikan error yang sama.

::: code-group

<<< @/examples/methodFunctions/paniclog_code.rs{rust:line-numbers}[Kode]

:::

## Menggunakan Node N-API

Metode `paniclog()` Node N-API mengembalikan array catatan terstruktur saat berhasil dan melempar error N-API saat otorisasi, penyimpanan, validasi, atau penguraian gagal.

::: info
**Kapabilitas yang Dibutuhkan**: `Capability.Debug`
:::
