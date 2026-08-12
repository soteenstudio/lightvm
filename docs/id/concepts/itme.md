# Itme (Alat Benchmarking)
**Itme** adalah utilitas benchmarking presisi tinggi yang dirancang untuk LightVM, memberikan wawasan yang dapat ditindaklanjuti kepada pengembang mengenai performa kode. Dengan memanfaatkan analisis statistik dan siklus iterasi adaptif, **Itme** memastikan pengukuran fungsi Anda dapat diandalkan dan direproduksi dengan akurat.

## Cara Kerja Itme
Itme menggunakan strategi eksekusi adaptif untuk menyeimbangkan durasi benchmarking dengan presisi. Alat ini secara otomatis mengkalibrasi jumlah iterasi yang diperlukan untuk mencapai jendela waktu eksekusi target, diikuti dengan analisis statistik sampel yang ketat untuk menyaring noise dan melaporkan metrik performa yang akurat.

  * **Iterasi Adaptif**: Alih-alih menggunakan jumlah perulangan yang tetap, Itme secara dinamis menyesuaikan jumlah iterasi hingga waktu yang diukur mencapai `target_time`. Hal ini memastikan fungsi yang berjalan singkat maupun lama mendapatkan ukuran sampel yang signifikan secara statistik.
  * **Fase Pemanasan (Warm-up Cycles)**: Mengeksekusi beberapa kali pengujian pra-pengukuran untuk memastikan cache CPU dan prediktor cabang sudah siap, sehingga mengurangi noise "cold start" pada data akhir.
  * **Penyaringan Pencilan (Metode IQR)**: Menggunakan metode Interquartile Range (IQR) untuk mengidentifikasi dan memangkas data pencilan (outlier) performa. Dengan mengecualikan titik data anomali, Itme memberikan representasi waktu eksekusi median yang lebih akurat.
  * **Analisis Statistik**: Menghitung rata-rata, simpangan baku, dan persentase stabilitas, yang memungkinkan Anda mengukur konsistensi performa kode di berbagai siklus eksekusi.
  * **Perhitungan Throughput**: Ketika ukuran byte disediakan, Itme secara otomatis menghitung throughput dalam MiB/s, membantu Anda mengukur efisiensi pemrosesan data pada algoritma Anda.
  * **Deteksi Noise**: Secara otomatis menandai benchmark sebagai `[NOISY]` jika terdeteksi varians yang tinggi (stabilitas > 15%), memperingatkan Anda akan potensi ketidakstabilan performa atau interferensi eksternal.
  * **Pelaporan Presisi**: Menghasilkan output CLI berformat dan berwarna yang menampilkan waktu per operasi, rentang performa, metrik stabilitas, serta throughput secara jelas.
