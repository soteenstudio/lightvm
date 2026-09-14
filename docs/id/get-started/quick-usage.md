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

## Tautan diagnostik

Error VM yang diformat menyertakan baris metadata `documentation:` secara default. URL-nya berasal dari kode `LVM` error dan mengarah ke halaman yang sesuai dalam [referensi kode error](/id/api-reference/error-codes/lvm001-code).

Nonaktifkan baris tersebut melalui konfigurasi konstruktor atau API fluent:

::: code-group

```rust [Native configuration]
let vm = LightVM::new(VmConfig {
  error_options: Some(ErrorOptions {
    diagnostic_links: false,
    ..Default::default()
  }),
  ..Default::default()
});

let vm = LightVM::new(VmConfig::default())
  .with_diagnostic_links(false);
```

```ts [Node.js]
const vm = new LightVM({
  errorOptions: {
    backtrace: false,
    explain: false,
    hint: true,
    diagnosticLinks: false,
  },
});

vm.withDiagnosticLinks(false);
```

```ts [WASM]
const vm = new LightVM({
  caps: [],
  errorOptions: { diagnosticLinks: false },
});

vm.withDiagnosticLinks(false);
```

:::

Pengaturan yang sama dipertahankan oleh `tools()` dan berlaku untuk error dari operasi alat, termasuk optimasi bytecode.

## Hasil yang diharapkan

Anda memiliki instance VM terkonfigurasi yang siap memuat bytecode. Lanjutkan ke [Metode Run](/id/api-reference/method-functions/run-method), atau tinjau [Kapabilitas](/id/api-reference/capabilities) sebelum memberikan akses.

::: info Referensi API Terkait
`TimeBudget` membatasi waktu optimasi bytecode. Gunakan `SecurityConfig.maxTicks` untuk membatasi pekerjaan eksekusi.
:::
