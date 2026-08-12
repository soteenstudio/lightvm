# Apa itu LightVM?
**LightVM** adalah mesin virtual (*virtual machine*) berperforma tinggi dan deterministik yang dirancang untuk menjembatani kesenjangan antara logika yang mudah dibaca manusia dan eksekusi yang efisien bagi mesin. Dibangun dengan Rust, LightVM memprioritaskan transparansi sumber daya dan keamanan, menjadikannya *runtime* yang ideal untuk sistem tertanam (*embedded systems*), *engine* simulasi, dan aplikasi yang mengutamakan performa.

## Filosofi
Pada intinya, LightVM dibangun di atas tiga pilar fundamental yang mendefinisikan cara kerjanya dalam memproses kode Anda:

 * __Zero Magic (Deterministik)__: Eksekusi bersifat linear dan sepenuhnya dapat diprediksi. VM beroperasi secara eksplisit, artinya setiap instruksi dijalankan persis seperti yang didefinisikan, tanpa transisi status tersembunyi atau perilaku *runtime* yang tidak terduga.
 * __Resource Conscious (Sadar Sumber Daya)__: LightVM dirancang dengan jejak memori (*memory footprint*) yang minimal. Dengan memanfaatkan struktur data yang dioptimalkan seperti `SmolStr` dan `Ahash` untuk manajemen metadata, LightVM mempertahankan performa tinggi bahkan di bawah batasan sumber daya yang ketat.
 * __Explicit Security (Keamanan Eksplisit)__: Keamanan ditegakkan melalui sistem *Capability* yang ketat. VM tidak berasumsi mengenai izin akses; sebaliknya, setiap akses dan operasi harus memiliki hak yang didefinisikan secara eksplisit oleh *host* sejak awal, guna mencegah efek samping yang tidak diinginkan.

## Arsitektur: Pipeline Eksekusi
LightVM mencapai kecepatannya melalui *pipeline* pra-eksekusi yang canggih. Sebelum satu instruksi pun diproses oleh *loop* utama, *bytecode* Anda melewati tiga tahap khusus yang dirancang untuk memaksimalkan efisiensi:

### 1. Torja: Penyelesai Simbol
**Torja** bertindak sebagai gerbang masuk VM. Ia mengubah *bytecode* tingkat tinggi yang mudah dibaca manusia menjadi format berperforma tinggi. Dengan memetakan nama variabel dan pengenal fungsi ke indeks integer posisi tetap, Torja menghilangkan pencarian *hash-map* yang memakan waktu saat *runtime*. Torja juga melakukan "Value Promotion", yaitu mengonversi instruksi generik menjadi *opcode* khusus (contoh: `push_int16` vs `push_string`), yang memberikan informasi awal kepada *execution engine* mengenai tipe dan ukuran data.

### 2. Gazle: Pengoptimal Bytecode
**Gazle** bertindak sebagai mesin pengoptimal begitu simbol-simbol diselesaikan untuk menyempurnakan bytecode. Sistem ini menjalankan alur kerja pengoptimalan multi-tahap—termasuk constant folding, dead store elimination, dan jump threading—untuk memangkas operasi yang tidak perlu dan menyederhanakan alur kontrol. Pada saat bytecode mencapai fase eksekusi, langkah-langkah redundan telah dibersihkan, memastikan bahwa VM hanya melakukan pekerjaan yang berkontribusi langsung pada status program akhir.

### 3. Krates: Lapisan Validasi & Keamanan
**Krates** bertindak sebagai penjaga gerbang terakhir yang memeriksa bytecode sebelum eksekusi, memastikan runtime tetap terlindungi dari instruksi yang cacat, fitur yang tidak sah, dan pelanggaran akses memori. Dengan menerapkan protokol keamanan yang ketat melalui alur verifikasi yang komprehensif, Krates menjamin bahwa hanya bytecode yang aman dan deterministik yang mencapai mesin eksekusi. Krates menangani tugas-tugas keamanan penting, termasuk verifikasi batas untuk mencegah luapan memori, pemeriksaan keamanan variabel, dan validasi integritas fungsi. Selain itu, Krates memantau fitur-fitur yang dibatasi, menegakkan kuota sumber daya melalui pemantauan gas (tick) untuk mencegah loop tak terbatas, serta memelihara daftar putih modul yang ketat. Krates juga melakukan analisis pola instruksi untuk mendeteksi bytecode yang berpotensi berbahaya, sekaligus menawarkan konfigurasi `unsafe_mode` untuk melewati pemeriksaan ini pada lingkungan tepercaya yang mengutamakan performa tinggi.

### 4. Itme: Utilitas Benchmarking
**Itme** bertindak sebagai utilitas benchmarking presisi tinggi yang dirancang untuk mengukur dan menganalisis performa kode. Dengan memanfaatkan siklus iterasi adaptif, fase pemanasan (warm-up), dan analisis statistik yang ketat menggunakan metode Interquartile Range (IQR), Itme menyaring gangguan (noise) dan mengevaluasi konsistensi eksekusi. Alat ini secara otomatis menghitung waktu per operasi yang presisi, simpangan baku, throughput dalam MiB/s, serta persentase stabilitas, guna memastikan metrik performa kode Anda dapat diandalkan dan direproduksi dengan akurat.

::: tip
LightVM dirancang agar ramping, transparan, dan cepat. Dengan memisahkan **Resolusi** (Torja), **Optimasi** (Gazle), **Keamanan** (Krates) dan **Benchmarking** (Itme) menjadi modul terpisah, LightVM memastikan bahwa *loop* eksekusi inti VM tetap sesingkat mungkin.
:::