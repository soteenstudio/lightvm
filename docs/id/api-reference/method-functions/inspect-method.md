# Metode Inspect <Badge type="warning" text="Preview" />
Metode `inspect` memungkinkan Anda untuk memeriksa keadaan VM saat ini, memberikan visibilitas ke dalam tumpukan, register, dan bytecode yang dimuat pada titik mana pun selama siklus eksekusi.

## Menggunakan TypeScript
Untuk **TypeScript**, Anda dapat menggunakan metode `inspect` untuk mengambil cuplikan status internal VM. Ini sangat berguna untuk debugging atau memantau perilaku VM selama runtime untuk memastikan

::: code-group

<<< @/examples/methodFunctions/inspectCode.ts{ts:line-numbers}[Kode]

:::

## Menggunakan Rust
Dalam **Rust**, metode `inspect` menyediakan tampilan terstruktur dari internal VM. Metode ini dirancang untuk mengekspos status lingkungan saat ini, memungkinkan pelacakan nilai variabel dan stack secara tepat.

::: code-group

<<< @/examples/methodFunctions/inspect_code.rs{rust:line-numbers}[Kode]

:::

::: info
**Kapabilitas yang Dibutuhkan**: `Observe`
:::