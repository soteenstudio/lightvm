# Penggunaan Cepat
Inisialisasi LightVM **sangat fleksibel** dan memungkinkan Anda untuk mengkonfigurasi **kapabilitas** dan fitur **debugging sesuai** dengan kebutuhan aplikasi Anda.

## Menggunakan TypeScript
Untuk proyek berbasis **TypeScript**, Anda dapat mengkonfigurasi instance VM dengan pola builder yang intuitif sebelum mengakses antarmuka `tools` utama.

::: code-group

<<< @/examples/getStarted/builderPattern.ts{ts:line-numbers}[Builder Pattern]

<<< @/examples/getStarted/objectPattern.ts{ts:line-numbers}[Object Pattern]

:::

## Menggunakan Rust
Bagi pengguna **Rust**, konfigurasi dilakukan melalui `VmConfig`. Anda dapat mengatur kemampuan VM secara deklaratif sebelum mengeksekusi bytecode.

::: code-group

<<< @/examples/getStarted/builder_pattern.rs{rust:line-numbers}[Builder Pattern]

<<< @/examples/getStarted/object_pattern.rs{rust:line-numbers}[Object Pattern]

:::

::: tip
Dapatkan antarmuka alat. Simpan ini sebagai konstanta agar dapat digunakan kembali untuk semua tugas yang akan datang.
:::

::: info Referensi API Terkait
Mau ngatur izin akses atau batas eksekusi? Cek panduan lengkapnya di halaman [Kapabilitas](/id/api-reference/capabilities) dan [Batas Waktu Eksekusi](/id/api-reference/time-budget).
:::
