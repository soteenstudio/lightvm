# Metode Halt
Metode `halt` adalah fungsi kontrol penting yang digunakan untuk segera menghentikan semua eksekusi VM. Metode ini mengakhiri proses saat ini, mencegah instruksi bytecode lebih lanjut diproses.

## Menggunakan TypeScript
Untuk **TypeScript**, memanggil metode `halt` memaksa VM untuk berhenti seketika. Ini berguna untuk penghentian darurat atau ketika logika Anda menentukan bahwa konteks eksekusi saat ini tidak lagi valid.

::: code-group

<<< @/examples/methodFunctions/haltCode.ts{ts:line-numbers}[Kode]

:::

## Menggunakan Rust
In **Rust**, the halt method provides a direct way to terminate the VM instance. Once called, the VM will stop executing, ensuring that no subsequent instructions or queued tasks are performed.

::: code-group

<<< @/examples/methodFunctions/halt_code.rs{rust:line-numbers}[Kode]

:::

::: info
**Kapabilitas yang Dibutuhkan**: `Unsafe`
:::