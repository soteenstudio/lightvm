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

## Hasil yang diharapkan

Anda memiliki instance VM terkonfigurasi yang siap memuat bytecode. Lanjutkan ke [Metode Run](/id/api-reference/method-functions/run-method), atau tinjau [Kapabilitas](/id/api-reference/capabilities) sebelum memberikan akses.

::: info Referensi API Terkait
`TimeBudget` membatasi waktu optimasi bytecode. Gunakan `SecurityConfig.maxTicks` untuk membatasi pekerjaan eksekusi.
:::
