# Torja (Penyelesai Simbol)
Torja adalah Penyelesai Simbol inti dari LightVM. Ia bertindak sebagai jembatan antara bytecode tingkat tinggi—yang menggunakan nama-nama yang mudah dibaca manusia untuk variabel dan fungsi—dan mesin eksekusi berperforma tinggi yang mengandalkan indeks numerik yang efisien memori.

## Cara Kerja Torja
Sebelum bytecode Anda mencapai fase eksekusi, Torja melakukan proses krusial untuk menyelesaikan semua referensi simbolik menjadi indeks posisi tetap.

 * **Pemetaan Simbol**: Memindai semua impor dan instruksi dinamis untuk memetakan setiap nama variabel atau fungsi yang unik ke indeks integer yang stabil.
 * **Resolusi Dinamis**: Jika sebuah variabel atau fungsi ditemui untuk pertama kalinya selama proses resolusi, Torja secara dinamis memberikan indeks baru, memastikan ID unik untuk setiap simbol selama siklus hidup program.
 * **Spesialisasi Instruksi**: Ia mengubah instruksi berbasis nama yang generik (contoh: `get`, `set`, `inc`) menjadi padanannya yang berbasis indeks (contoh: `get_idx`, `set_idx`, `inc_idx`). Ini meminimalkan pencarian saat runtime dan secara signifikan mengurangi beban kerja CPU selama eksekusi.
 * **Promosi Nilai**: Selama proses simbol, Torja juga mengoptimalkan instruksi `push` dengan mempromosikan tipe Nilai generik menjadi opcode khusus tipe data (contoh: `push_int16`, `push_float64`, `push_string`), memastikan VM mengetahui ukuran dan tipe data yang tepat secara langsung.
 * **Pelacakan Cakupan Fungsional**: Torja mengidentifikasi parameter fungsi dan pengenal cakupan, memastikan bahwa semua simbol lokal dilacak dengan benar di dalam tabel simbol dan disiapkan untuk arsitektur berbasis stack milik VM.
