# Apa itu LightVM?
**LightVM** adalah mesin virtual (*virtual machine*) berperforma tinggi dan deterministik yang dirancang untuk menjembatani kesenjangan antara logika yang mudah dibaca manusia dan eksekusi yang efisien bagi mesin. Dibangun dengan Rust, LightVM memprioritaskan transparansi sumber daya dan keamanan, menjadikannya *runtime* yang ideal untuk sistem tertanam (*embedded systems*), *engine* simulasi, dan aplikasi yang mengutamakan performa.

## Filosofi
Pada intinya, LightVM dibangun di atas tiga pilar fundamental yang mendefinisikan cara kerjanya dalam memproses kode Anda:

 * __Zero Magic (Deterministik)__: Eksekusi bersifat linear dan sepenuhnya dapat diprediksi. VM beroperasi secara eksplisit, artinya setiap instruksi dijalankan persis seperti yang didefinisikan, tanpa transisi status tersembunyi atau perilaku *runtime* yang tidak terduga.
 * __Resource Conscious (Sadar Sumber Daya)__: LightVM dirancang dengan jejak memori (*memory footprint*) yang minimal. Dengan memanfaatkan struktur data yang dioptimalkan seperti `SmolStr` dan `Ahash` untuk manajemen metadata, LightVM mempertahankan performa tinggi bahkan di bawah batasan sumber daya yang ketat.
 * __Explicit Security (Keamanan Eksplisit)__: Keamanan ditegakkan melalui sistem *Capability* yang ketat. VM tidak berasumsi mengenai izin akses; sebaliknya, setiap akses dan operasi harus memiliki hak yang didefinisikan secara eksplisit oleh *host* sejak awal, guna mencegah efek samping yang tidak diinginkan.

## Arsitektur: Pipeline Eksekusi
LightVM mencapai kecepatannya melalui *pipeline* pra-eksekusi yang canggih. Sebelum satu instruksi pun diproses oleh *loop* utama, *bytecode* Anda melewati dua tahap khusus yang dirancang untuk memaksimalkan efisiensi:

### 1. Torja: The Symbol Resolver
**Torja** bertindak sebagai gerbang masuk VM. Ia mengubah *bytecode* tingkat tinggi yang mudah dibaca manusia menjadi format berperforma tinggi. Dengan memetakan nama variabel dan pengenal fungsi ke indeks integer posisi tetap, Torja menghilangkan pencarian *hash-map* yang memakan waktu saat *runtime*. Torja juga melakukan "Value Promotion", yaitu mengonversi instruksi generik menjadi *opcode* khusus (contoh: `push_int16` vs `push_string`), yang memberikan informasi awal kepada *execution engine* mengenai tipe dan ukuran data.

### 2. Gazle: The Bytecode Optimizer
Setelah simbol diselesaikan, **Gazle** mengambil alih untuk menyempurnakan *bytecode*. Ia menjalankan *pipeline* optimasi multi-pass—termasuk *constant folding*, *dead store elimination*, dan *jump threading*—untuk memangkas operasi yang tidak perlu dan menyederhanakan alur kontrol. Saat *bytecode* mencapai fase eksekusi, kode tersebut telah dibersihkan dari langkah-langkah redundan, memastikan bahwa VM hanya melakukan pekerjaan yang berkontribusi langsung pada status program akhir.

::: tip
LightVM dirancang agar ramping, transparan, dan cepat. Dengan memisahkan **Resolusi** (Torja) dari **Optimasi** (Gazle), LightVM memastikan bahwa *loop* eksekusi inti VM tetap sesingkat mungkin.
:::
