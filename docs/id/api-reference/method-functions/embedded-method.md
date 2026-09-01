# Metode Embedded

Metode `.embedded()` mengeksekusi program yang sedang dimuat di VM dan mengembalikan `VMResult`.

Sebelum eksekusi, metode ini menghapus output dari eksekusi embedded sebelumnya. Karena itu, `VMResult.outputs` yang dikembalikan hanya berisi output yang dihasilkan oleh eksekusi saat ini.

## Menggunakan TypeScript

Buat VM dengan `Control` dan `Observe`, muat program, lalu panggil `.embedded()`:

::: code-group

<<< @/examples/methodFunctions/embeddedCode.ts{ts:line-numbers}[Eksekusi Embedded]

:::

## Hasil

`VMResult` berisi bidang berikut:

- `value`: Nilai hasil program yang terdefinisi. Nilainya `null` ketika program tidak mengembalikan nilai atau ketika VM dihentikan.
- `outputs`: Output yang hanya dihasilkan oleh eksekusi embedded saat ini.
- `halted`: Status penghentian VM saat ini.

::: info
**Kapabilitas yang Dibutuhkan**: `Control` mengeksekusi program, dan `Observe` mengambil output-nya.
:::

## Kesalahan

Metode `.embedded()` native mengembalikan `{ status: "error", message }` ketika eksekusi gagal. Binding N-API dan WebAssembly meneruskan kegagalan melalui mekanisme kesalahan binding masing-masing.
