# Itme (Alat Benchmarking)
**Itme** adalah utilitas benchmarking untuk **LightVM** yang mengukur performa fungsi menggunakan jumlah iterasi adaptif dan analisis statistik.

## Cara Kerja Itme
Itme mengkalibrasi iterasi per sampel, melakukan pemanasan, dan mengukur jumlah sampel yang dikonfigurasi. Itme menyaring pencilan dan melaporkan variasi waktu; pengukuran tetap dapat dipengaruhi lingkungan host.

  * **Iterasi Adaptif**: Menyesuaikan iterasi per sampel hingga kalibrasi mencapai `target_time` atau batas satu miliar iterasi. Kalibrasi mengubah jumlah iterasi, bukan jumlah sampel; `samples()` mengatur jumlah sampel, yang secara bawaan bernilai 15.
  * **Fase Pemanasan (Warm-up Cycles)**: Melakukan 25 kali eksekusi tanpa pengukuran menggunakan jumlah iterasi hasil kalibrasi untuk mengurangi efek cold start.
  * **Penyaringan Pencilan (Metode IQR)**: Menyaring sampel di luar batas kuartil yang diperluas sebesar 1,5 kali IQR. Jika tersisa kurang dari tiga sampel, semua sampel asli digunakan.
  * **Analisis Statistik**: Menghitung rata-rata dan simpangan baku dari sampel yang digunakan. Metrik stabilitas adalah simpangan baku / rata-rata × 100; nilai yang lebih rendah menunjukkan variasi yang lebih kecil.
  * **Perhitungan Throughput**: Ketika ukuran byte disediakan, Itme secara otomatis menghitung throughput dalam MiB/s, membantu Anda mengukur efisiensi pemrosesan data pada algoritma Anda.
  * **Deteksi Noise**: Secara otomatis menandai benchmark sebagai `[NOISY]` jika terdeteksi varians yang tinggi (stabilitas > 15%), memperingatkan Anda akan potensi ketidakstabilan performa atau interferensi eksternal.
  * **Pelaporan Terformat**: Menampilkan median waktu per operasi, waktu minimum dan maksimum per operasi, stabilitas, serta throughput opsional dalam keluaran CLI berwarna.
