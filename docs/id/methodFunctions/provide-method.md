# Metode Provide
Setelah mendefinisikan fungsi atau logika Anda, metode `provide` digunakan untuk mendaftarkannya ke dalam konteks VM, sehingga dapat diakses selama eksekusi.

## Menggunakan TypeScript
Untuk **TypeScript**, Anda dapat menyediakan fungsi atau struktur data langsung ke dalam konteks VM. Pastikan tipe yang disediakan sesuai dengan antarmuka yang diharapkan dari instance `LightVM`.

::: code-group

<<< @/examples/methodFunctions/provideWithArray.ts{ts:line-numbers}[Dengan Array]

:::

## Menggunakan Rust
Dalam **Rust**, Anda dapat menyediakan logika Anda menggunakan representasi string mentah atau dengan memanfaatkan `serde_json` untuk struktur data yang lebih kompleks. Hal ini memungkinkan VM untuk mengintegrasikan logika yang didefinisikan Rust secara mulus.

::: code-group

<<< @/examples/methodFunctions/provide_with_raw_string.rs{rust:line-numbers}[Dengan String Mentah]

<<< @/examples/methodFunctions/provide_with_serde.rs{rust:line-numbers}[Dengan Serde]

:::

::: info
**Kapabilitas yang Dibutuhkan**: tidak ada kapabilitas khusus
:::