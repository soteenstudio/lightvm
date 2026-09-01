# Metode Embedded

Metode `.embedded()` mengeksekusi program yang sedang dimuat di VM dan mengembalikan `value` yang terdefinisi, `outputs` eksekusi saat ini, serta status `halted` VM. Hasil yang tidak terdefinisi atau VM yang dihentikan memberikan `null` untuk `value`.

## Menggunakan TypeScript
Untuk **TypeScript**, buat VM dengan `Control` dan `Observe`, muat array bytecode mentah, lalu panggil `.embedded()`.

::: code-group

<<< @/examples/methodFunctions/embeddedCode.ts{ts:line-numbers}[Dengan Array]

:::

## Menggunakan Rust
Dalam **Rust**, Anda dapat memuat string bytecode yang diserialisasi atau nilai `serde_json` sebelum memanggil `.embedded()`.

::: code-group

<<< @/examples/methodFunctions/embedded_with_raw_string.rs{rust:line-numbers}[Dengan String Mentah]

<<< @/examples/methodFunctions/embedded_with_serde.rs{rust:line-numbers}[Dengan Serde]

:::

::: info
**Kapabilitas yang Dibutuhkan**: `Control` mengeksekusi program, dan `Observe` mengambil output-nya.
:::

::: tip
Sebelum setiap eksekusi embedded, `.embedded()` menghapus output yang ditinggalkan oleh eksekusi embedded sebelumnya. `outputs` yang dikembalikan berasal dari eksekusi saat ini, `value` berisi hasil yang terdefinisi atau `null` untuk hasil yang tidak terdefinisi atau VM yang dihentikan, dan `halted` melaporkan status penghentian VM. Kegagalan eksekusi native mengembalikan `{ status: "error", message }`, sedangkan N-API dan WebAssembly meneruskan kegagalan melalui mekanisme kesalahan binding masing-masing.
:::
