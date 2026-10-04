# Krates (Validasi & Keamanan)
**Krates** adalah lapisan validasi dan keamanan **LightVM**. Krates memeriksa struktur bytecode tertentu dan pembatasan sumber daya yang dikonfigurasi sebelum dan selama eksekusi.

## Cara Kerja Krates
Krates menggabungkan validasi struktural, pemeriksaan keamanan, dan pemantauan tick eksekusi. Pemeriksaan ini tidak membuktikan keamanan bytecode secara menyeluruh, determinisme, atau keterjangkauan fungsi.

  * **Verifikasi Batas**: `validate_bytecode` memeriksa bahwa target `Jump`, `IfFalse`, dan `Break` berada di bawah panjang bytecode.
  * **Validasi Variabel**: `validate_vars` memeriksa indeks pada `ValIdx`, `GetIdx`, `SetIdx`, `IncIdx`, dan `DecIdx` terhadap `var_count`.
  * **Batas Alamat Awal Fungsi**: Memeriksa bahwa alamat awal setiap fungsi berada di bawah panjang bytecode.
  * **Pembatasan Fitur**: `has_nightly_opcodes` mendeteksi `instantiate`, `import`, dan `export` dalam sumber terserialisasi; antarmuka pemuatan menolaknya saat dukungan nightly dinonaktifkan.
  * **Kuota Sumber Daya**: `validate_security` menghitung I/O, impor, pembuatan objek/array, panggilan, dan lompatan dalam bytecode terhadap batas yang dikonfigurasi. Loop eksekusi juga menghitung operasi tersebut saat runtime.
  * **Daftar Putih Modul**: Memeriksa nama modul impor terhadap `SecurityConfig.allowed_imports` saat validasi keamanan.
  * **Pengisian Nop**: Menolak bytecode yang lebih panjang dari sepuluh instruksi ketika jumlah `Nop` melebihi jumlah total instruksi dibagi sepuluh menggunakan pembagian integer.
  * **Kemampuan Bypass**: `unsafe_mode` melewati `validate_security` (kuota statis, daftar putih impor, dan pemeriksaan pengisian `Nop`) serta kuota runtime untuk I/O, impor, alokasi, panggilan, dan lompatan. Validasi struktural, pembatasan fitur, batas stack, dan pemeriksaan gas tetap aktif.
  * **Pemantauan Gas (Kontrol Tick)**: `GasMonitor` memeriksa batas tick pada setiap iterasi loop eksekusi. Tick menghitung iterasi loop, bukan waktu yang berlalu atau siklus pemrosesan perangkat keras.
  * **Validasi Tick**: Menolak `max_ticks` bernilai nol saat inisialisasi dan melaporkan error ketika jumlah tick mencapai batas.
