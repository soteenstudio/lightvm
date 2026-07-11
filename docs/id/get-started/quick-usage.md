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

## Kapabilitas
Keamanan dan perilaku `LightVM` dikelola melalui sistem kapabilitas yang tangguh. Gunakan tabel berikut untuk memahami izin apa yang diperlukan untuk kasus penggunaan spesifik Anda:

| Kapabilitas | Level | Deskripsi |
|------------|-------|-------------|
| `Control` | Rendah | Memberikan izin untuk memulai/menghentikan eksekusi dan fungsi ekspor. |
| `Observe` | Sedang | Memungkinkan host untuk memeriksa status internal, tumpukan variabel, dan metrik. |
| `Debug` | Tinggi | Membuka akses ke log internal yang detail dan status tersembunyi untuk keperluan pemecahan masalah. |
| `Unsafe` | Kritis | Menghilangkan pengaman, memungkinkan penghentian manual dan akses langsung ke memori/proses. |

::: warning Pemberitahuan Keamanan
Selalu patuhi **Prinsip Hak Akses Minimal**. Aktifkan hanya kemampuan spesifik yang dibutuhkan aplikasi Anda untuk memastikan lingkungan eksekusi yang aman dan dapat diprediksi.
:::