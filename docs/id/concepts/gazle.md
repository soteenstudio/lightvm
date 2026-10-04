# Gazle (Pengoptimal)
**Gazle** adalah pengoptimal bawaan **LightVM** yang dipanggil secara eksplisit sebelum memuat atau mengeksekusi bytecode. Metode multi-pass adaptifnya menggunakan anggaran yang dapat dikonfigurasi: Cheap = 200 ms, Normal = 1.000 ms, dan Expensive = 5.000 ms.

## Cara Kerja Gazle
Gazle menggunakan mekanisme **Time-Budgeted Optimization** dan mengurutkan pass berdasarkan bobot keberhasilan (`pass_weights`). Anggaran diperiksa di antara pass; pass yang sedang berjalan dapat melampaui anggaran yang dipilih. Optimasi bukan tahap wajib dalam eksekusi normal.

  * **Instruksi Terspesialisasi**: Mengonversi nilai `push` generik yang didukung menjadi instruksi spesifik tipe seperti `push_int16`, `push_string`, dan `push_bool`.
  * **Pelipatan Konstanta**: Melakukan pra-hitung operasi matematika dan logika (misalnya, `add`, `sub`, `xor`, `concat`) jika nilainya diketahui pada saat kompilasi.
  * **Konversi & Pelipatan Metadata**: Melakukan pra-evaluasi konversi tipe (misalnya, `to_integer`, `to_string`) dan pemeriksaan metadata seperti `type_of` untuk menghilangkan pekerjaan runtime yang berlebihan.
  * **Pengurangan Kekuatan**: Menggantikan operasi "berat" dengan operasi yang lebih ringan, seperti mengubah perkalian dengan pangkat dua menjadi `shl` (Shift Left) bitwise.
  * **Penghapusan Penyimpanan yang Tidak Berguna**: Menggunakan informasi pembacaan variabel dan pemindaian kebutuhan stack dari belakang untuk mengganti penyimpanan, penambahan/pengurangan, dan instruksi penghasil nilai tertentu yang tidak digunakan dengan `Nop`.
  * **Penghapusan Loop Mati**: Pass saat ini memilih instruksi `Jump` dan `IfFalse` yang mengarah ke belakang sebagai kandidat loop. Pemeriksaan kemurniannya menyertakan dan menolak instruksi alur kontrol penutup tersebut, sehingga loop yang dipilih tidak dihapus.
  * **Penghapusan Pemuatan Berlebihan**: Mengganti pemuatan identik berurutan yang didukung dengan `dup`, termasuk `get`/`get_idx` berulang dan nilai `push` generik.
  * **Optimasi Lompatan**: Mendeteksi dan menghapus instruksi Lompatan yang berlebihan yang mengarah ke baris kode berikutnya.
  * **Lompatan Threading**: Mengoptimalkan alur kontrol dengan meruntuhkan rantai pengalihan, di mana sebuah lompatan mengarah langsung ke lompatan lain, memastikan penunjuk instruksi melewati lompatan perantara untuk mencapai tujuan.
  * **Perambatan Konstan**: Mengoptimalkan *bytecode* dengan melacak penugasan variabel dan mengganti operasi `get` dengan instruksi `push` langsung ketika nilainya berupa konstanta yang diketahui. Sistem ini memeriksa frekuensi penggunaan dan menghindari penyisipan (*inlining*) objek berat seperti *array* atau objek, serta menghapus konstanta yang dilacak pada instruksi `Jump` dan `IfFalse`.

::: info
Anda dapat menemukan cara menggunakan Gazle di halaman [Metode Optimize Bytecode](../api-reference/method-functions/tools-method/optimize-bytecode-method).
:::
