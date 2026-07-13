# Gazle (Pengoptimal)
**Gazle** adalah pengoptimal bawaan yang disertakan dengan **LightVM** untuk melakukan pengoptimalan sebelum bytecode Anda dieksekusi melalui metode pengoptimalan multi-pass untuk memastikan kinerja maksimum dan meminimalkan

## Cara Kerja Gazle
Gazle menggunakan alur transformasi khusus untuk menyempurnakan bytecode Anda. Ia memproses aliran instruksi melalui beberapa tahapan, secara sistematis mengidentifikasi dan menghilangkan ketidakefisienan sebelum.

 * **Pelipatan Konstanta**: Melakukan pra-hitung operasi matematika dan logika (misalnya, `add`, `sub`, `xor`, `concat`) jika nilainya diketahui pada saat kompilasi.
 * **Konversi & Pelipatan Metadata**: Melakukan pra-evaluasi konversi tipe (misalnya, `to_integer`, `to_string`) dan pemeriksaan metadata seperti `type_of` untuk menghilangkan pekerjaan runtime yang berlebihan.
 * **Pengurangan Kekuatan**: Menggantikan operasi "berat" dengan operasi yang lebih ringan, seperti mengubah perkalian dengan pangkat dua menjadi `shl` (Shift Left) bitwise.
 * **Penghapusan Penyimpanan yang Tidak Berguna**: Menganalisis penggunaan variabel dan secara otomatis menghapus operasi `push`, `set`, atau `inc` yang tidak berkontribusi pada keadaan program akhir.
 * **Penghapusan Loop Mati**: Mengidentifikasi dan memangkas loop "murni" yang tidak memiliki efek samping (tidak ada I/O, panggilan, atau pengembalian), sehingga mencegah siklus CPU yang tidak perlu.
 * **Penghapusan Pemuatan Berlebihan**: Mendeteksi upaya berulang untuk memuat nilai atau variabel identik ke dalam tumpukan dan mengganti operasi yang berlebihan dengan instruksi `dup` berkinerja tinggi untuk meminimalkan 
 * **Optimasi Lompatan**: Mendeteksi dan menghapus instruksi Lompatan yang berlebihan yang mengarah ke baris kode berikutnya.
 * **Jump Threading**: Mengoptimalkan alur kontrol dengan meruntuhkan rantai pengalihan, di mana sebuah lompatan mengarah langsung ke lompatan lain, memastikan penunjuk instruksi melewati lompatan perantara untuk mencapai tujuan.

::: info
Anda dapat menemukan cara menggunakan Gazle di halaman [Metode Optimize Bytecode](../api-references/method-functions/tools-method/optimize-bytecode-method).
:::