# Torja (Penyelesai Simbol)
Torja adalah Symbol Resolver inti dari LightVM. Torja bertindak sebagai jembatan antara bytecode tingkat tinggi—yang menggunakan nama-nama variabel dan fungsi yang mudah dibaca manusia—dan mesin eksekusi berkinerja tinggi yang mengandalkan indeks numerik yang efisien secara memori.

## Cara Kerja Torja
Sebelum bytecode Anda mencapai fase eksekusi, Torja melakukan tahap krusial untuk menyelesaikan semua referensi simbolik menjadi indeks posisi tetap.

  * **Pemetaan Simbol & Impor**: Torja memuat tabel simbol di awal dengan semua impor yang disediakan. Saat melintasi bytecode, Torja memetakan setiap nama variabel unik yang ditemukan dalam instruksi simbolik ke indeks integer yang stabil.
  * **Resolusi Dinamis**: Torja menggunakan logika `get_or_insert_idx`; jika nama variabel ditemui untuk pertama kalinya, Torja secara dinamis memberikan indeks baru menggunakan penghitung bertambah (`next_idx`), memastikan ID unik untuk setiap simbol di sepanjang siklus hidup program.
  * **Spesialisasi Instruksi**: Torja mengubah instruksi umum berbasis nama menjadi pasangan berbasis indeks yang terspesialisasi. Ini termasuk mengonversi `get` menjadi `get_idx`, `set` menjadi `set_idx`, `inc` menjadi `inc_idx`, dan `dec` menjadi `dec_idx`. Hal ini meminimalkan pencarian saat runtime dan secara signifikan mengurangi overhead CPU selama eksekusi.
  * **Pelacakan Cakupan Fungsional**: Torja mengidentifikasi nama parameter fungsi dalam instruksi `Func`. Torja mendaftarkan nama-nama ini ke dalam tabel simbol, memastikan semua pengenal dalam cakupan lokal dilacak dengan benar dan disiapkan untuk arsitektur berbasis tumpukan (stack) VM.
