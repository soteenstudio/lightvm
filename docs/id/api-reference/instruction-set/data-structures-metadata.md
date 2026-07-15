# Struktur Data & Metadata
Instruksi-instruksi ini menangani alokasi memori dinamis untuk tipe data kompleks. Objek dan array di LightVM dialokasikan pada heap; saat menggunakan opcode ini, pastikan program kamu mengelola referensi dengan benar untuk menghindari overhead memori. Operasi metadata (`length`, `typeof`) memungkinkan introspeksi tipe data saat runtime dan validasi data.

| Kode Operasi | Argumen | Operan (stack) | Deskripsi |
| :--- | :--- | :--- | :--- |
| `make_obj` | count | k1, v1, ... kn, vn | Membuat Objek dari n pasangan key-value di stack |
| `make_array` | count | v1, v2, ... vn | Membuat Array dari n elemen di stack |
| `access` | prop_name | target_obj | Mengakses properti Objek |
| `access_index` | - | target_arr, index | Mengakses elemen Array berdasarkan indeks di stack |
| `length` | - | val | Memeriksa panjang string atau jumlah item dalam array/objek |
| `typeof` | - | val | Mendapatkan tipe data dari nilai teratas stack |
| `concat` | - | val1, val2 | Menggabungkan dua nilai (biasanya string) |