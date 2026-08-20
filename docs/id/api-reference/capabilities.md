# Kapabilitas
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