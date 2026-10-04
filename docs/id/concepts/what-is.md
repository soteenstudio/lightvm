# Apa itu LightVM?
**LightVM** adalah mesin virtual (*virtual machine*) berperforma tinggi dan deterministik yang dirancang untuk menjembatani kesenjangan antara logika yang mudah dibaca manusia dan eksekusi yang efisien bagi mesin. Dibangun dengan Rust, LightVM memprioritaskan transparansi sumber daya dan keamanan, menjadikannya *runtime* yang ideal untuk sistem tertanam (*embedded systems*), *engine* simulasi, dan aplikasi yang mengutamakan performa.

## Filosofi
Pada intinya, LightVM dibangun di atas tiga pilar fundamental yang mendefinisikan cara kerjanya dalam memproses kode Anda:

  * **Zero Magic (Eksekusi Eksplisit)**: Loop eksekusi menjalankan instruksi bytecode secara langsung. Impor host dan I/O dapat memengaruhi hasil, sehingga determinisme bergantung pada program dan lingkungannya.
  * __Resource Conscious (Sadar Sumber Daya)__: LightVM dirancang dengan jejak memori (*memory footprint*) yang minimal. Dengan memanfaatkan struktur data yang dioptimalkan seperti `SmolStr` dan `Ahash` untuk manajemen metadata, LightVM mempertahankan performa tinggi bahkan di bawah batasan sumber daya yang ketat.
  * **Explicit Security (Keamanan Eksplisit)**: Host mengatur kuota sumber daya, impor yang diizinkan, dan batas tick melalui `SecurityConfig`.

## Arsitektur: Pipeline Eksekusi
LightVM memisahkan resolusi simbol dan validasi dari optimasi eksplisit dan benchmarking. Eksekusi normal menyelesaikan simbol dan memvalidasi bytecode; optimasi Gazle adalah operasi opsional sebelum pemuatan atau eksekusi.

### 1. Torja: Penyelesai Simbol
**Torja** memetakan nama variabel dan parameter fungsi ke indeks numerik dalam satu tabel simbol per pemanggilan resolusi. Torja mengonversi instruksi berbasis nama yang didukung menjadi bentuk berbasis indeks tanpa membuat tabel cakupan leksikal terpisah.

### 2. Gazle: Pengoptimal Bytecode
**Gazle** dipanggil secara eksplisit untuk menerapkan pass seperti constant folding, dead store elimination, dan jump threading. Gazle juga mengubah nilai `push` generik yang didukung menjadi instruksi spesifik tipe seperti `push_int16` dan `push_string`. Optimasi menggunakan anggaran waktu yang dapat dikonfigurasi dan diperiksa di antara pass, bukan tenggat mutlak untuk pass yang sedang berjalan.

### 3. Krates: Lapisan Validasi & Keamanan
**Krates** memeriksa indeks variabel, target `Jump`/`IfFalse`/`Break`, dan batas alamat awal fungsi. Validasi keamanan memeriksa kuota instruksi yang dikonfigurasi, daftar putih impor, dan pengisian `Nop` berlebihan. Kuota runtime dan pemantauan gas menghitung operasi yang dieksekusi dan iterasi loop eksekusi. `unsafe_mode` melewati validasi keamanan dan kuota operasi runtime; validasi struktural, pembatasan fitur, batas stack, dan pemeriksaan gas tetap aktif. Pemeriksaan ini tidak menjamin keamanan bytecode secara menyeluruh atau keterjangkauan fungsi.

### 4. Itme: Utilitas Benchmarking
**Itme** adalah utilitas terpisah yang mengkalibrasi iterasi per sampel, melakukan pemanasan, dan menganalisis sampel waktu menggunakan penyaringan IQR. Itme melaporkan waktu per operasi, throughput ketika ukuran byte diberikan, dan metrik stabilitas berupa simpangan baku / rata-rata × 100. Nilai stabilitas yang lebih rendah menunjukkan variasi yang lebih kecil; hasil bergantung pada lingkungan pengukuran.

::: tip
Gunakan **Resolusi** (Torja) dan **Validasi** (Krates) selama eksekusi normal, panggil **Optimasi** (Gazle) secara eksplisit saat diperlukan, dan gunakan **Benchmarking** (Itme) untuk mengukur performa secara terpisah.
:::
