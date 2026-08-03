# Krates (Validasi & Keamanan)
**Krates** adalah lapisan validasi dan keamanan khusus dari **LightVM**. Ia bertindak sebagai penjaga gerbang terakhir yang memeriksa bytecode sebelum eksekusi, memastikan lingkungan runtime tetap terlindungi dari instruksi yang cacat, fitur yang tidak sah, dan pelanggaran akses memori.

## Cara Kerja Krates
Krates menerapkan protokol keamanan yang ketat melalui alur verifikasi yang komprehensif. Dengan memvalidasi integritas dan kepatuhan setiap instruksi, Krates menjamin bahwa hanya bytecode yang aman dan deterministik yang dapat mencapai mesin eksekusi.

  * **Verifikasi Batas**: Memindai semua instruksi lompatan (jump), percabangan (branch), dan pengulangan (loop) untuk memastikan alamat tujuan berada dalam ruang memori yang valid, guna mencegah akses di luar batas.
  * **Keamanan Variabel**: Memvalidasi bahwa semua instruksi akses berbasis indeks (`get_idx`, `set_idx`, dll.) merujuk pada variabel dalam rentang `var_count` yang dialokasikan, guna menghentikan potensi kerusakan memori.
  * **Integritas Fungsi**: Mencocokkan semua metadata fungsi (alamat awal) terhadap total panjang bytecode untuk memastikan setiap titik pemanggilan dapat dijangkau dan aman.
  * **Pembatasan Fitur (Feature Gating)**: Bertindak sebagai pagar pengaman dengan memantau fitur-fitur yang dibatasi, seperti opcode nightly, dan mencegah eksekusi jika lingkungan VM tidak dikonfigurasi untuk mendukung kapabilitas eksperimental.
  * **Penegakan Kuota Sumber Daya**: Melakukan pemeriksaan jumlah instruksi bytecode secara statis sebelum eksekusi melalui `validate_security` untuk mencegah kehabisan sumber daya, termasuk pembatasan pada operasi I/O, impor, alokasi memori, dan lompatan alur kontrol.
  * **Daftar Putih Modul**: Memastikan bahwa hanya modul yang telah disetujui sebelumnya yang ditetapkan dalam `SecurityConfig` yang dapat diimpor, guna memitigasi risiko dari kode eksternal yang tidak sah.
  * **Analisis Pola Instruksi**: Mendeteksi pola bytecode berbahaya, seperti pengisian `Nop` yang berlebihan, yang dapat digunakan untuk melewati analisis atau memperlambat waktu eksekusi.
  * **Kemampuan Bypass**: Mendukung konfigurasi `unsafe_mode` yang memungkinkan penonaktifan pemeriksaan keamanan secara eksplisit, ditujukan untuk lingkungan tepercaya dengan performa tinggi di mana overhead harus diminimalkan.
  * **Pemantauan Gas (Kontrol Tick)**: Menggunakan sistem `GasMonitor` untuk melacak waktu eksekusi atau kompleksitas melalui "tick". Ini menerapkan batas atas yang ketat pada siklus pemrosesan untuk mencegah loop tak terbatas atau eksekusi yang tidak terkendali, memastikan VM tetap responsif dan deterministik.
  * **Validasi Tick**: Memvalidasi `SecurityConfig` selama inisialisasi untuk memastikan batas tick bukan nol, guna mencegah status konfigurasi yang tidak valid atau tidak aman sebelum runtime dimulai.
