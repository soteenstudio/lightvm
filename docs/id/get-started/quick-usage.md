# Penggunaan Cepat

Buat dan konfigurasikan instance `LightVM`. Selesaikan [Instalasi](/id/get-started/installation) terlebih dahulu.

## Menggunakan TypeScript
Untuk proyek TypeScript, gunakan konfigurasi konstruktor atau metode konfigurasi berantai.

::: code-group

<<< @/examples/getStarted/builderPattern.ts{ts:line-numbers}[Builder Pattern]

<<< @/examples/getStarted/objectPattern.ts{ts:line-numbers}[Object Pattern]

:::

## Menggunakan Rust
Untuk proyek Rust, konfigurasikan `VmConfig` sebelum membuat VM.

::: code-group

<<< @/examples/getStarted/builder_pattern.rs{rust:line-numbers}[Builder Pattern]

<<< @/examples/getStarted/object_pattern.rs{rust:line-numbers}[Object Pattern]

:::

::: tip Detail Referensi Konfigurasi
Untuk rincian komprehensif mengenai setiap opsi konfigurasi, parameter, dan detail tipe dalam TypeScript, cek [Referensi Konfigurasi TypeScript](/id/api-reference/configuration-reference/typescript-configuration) atau [Referensi Konfigurasi Rust](/id/api-reference/configuration-reference/rust-configuration).
:::
