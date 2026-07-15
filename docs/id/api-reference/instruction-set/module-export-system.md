# Sistem Modul & Ekspor
Instruksi-instruksi ini mengontrol sistem modul dan ekspor di dalam virtual machine. Instruksi ini menyediakan mekanisme untuk memodularisasi kode, memungkinkan impor pustaka eksternal, serta mengekspos komponen internal agar dapat diakses dari luar.

| Operasi Kode | Argumen | Operan (stack) | Deskripsi |
| :--- | :--- | :--- | :--- |
| `import` | module_name, alias_idx | target, length | Mengimpor pustaka/modul eksternal ke dalam indeks variabel tertentu |
| `export` | name | - | Menandai fungsi atau variabel agar dapat diakses dari luar VM |

::: warning
**Operasi Kode Nightly**: Instruksi `export` dan `import` masih bersifat eksperimental. API dapat berubah sewaktu-waktu tanpa pemberitahuan pada versi `@next` atau `@nightly`.
:::