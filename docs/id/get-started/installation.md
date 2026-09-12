# Instalasi

Instal LightVM untuk proyek Node.js atau Rust. Anda memerlukan pengelola paket Node.js yang didukung atau toolchain Rust stabil.

## Dengan NPM
Gunakan metode ini jika Anda sedang membangun proyek berbasis **TypeScript/Node.js**.

::: code-group

<<< @/examples/installation/npm-latest.sh{sh:line-numbers}[Latest]

<<< @/examples/installation/npm-next.sh{sh:line-numbers}[Next]

<<< @/examples/installation/npm-nightly.sh{sh:line-numbers}[Nightly]

:::

## Dengan Yarn
Alternatif bagi Anda yang lebih suka menggunakan **Yarn** sebagai pengelola paket.

::: code-group

<<< @/examples/installation/yarn-latest.sh{sh:line-numbers}[Latest]

<<< @/examples/installation/yarn-next.sh{sh:line-numbers}[Next]

<<< @/examples/installation/yarn-nightly.sh{sh:line-numbers}[Nightly]

:::

## Dengan Cargo
Jika Anda menggunakan **Rust**, gunakan **Cargo** untuk mengintegrasikan **LightVM** secara native.

::: code-group

<<< @/examples/installation/cargo-latest.sh{sh:line-numbers}[Latest]

<<< @/examples/installation/cargo-specific.sh{sh:line-numbers}[Versi Spesifik]

:::

## Hasil yang diharapkan

Paket ditambahkan ke manifest proyek dan siap diimpor. Lanjutkan ke [Penggunaan Cepat](/id/get-started/quick-usage) untuk membuat instance `LightVM`.
