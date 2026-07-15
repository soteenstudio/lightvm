# Objek & OOP
Instruksi-instruksi ini mengelola konsep pemrograman berorientasi objek di dalam virtual machine. Opcode-opcode ini memungkinkan modifikasi status objek, pembuatan instance kelas, dan debugging struktur data kompleks untuk memfasilitasi pengembangan aplikasi yang tangguh.

| Operasi Kode | Argumen | Operan (stack) | Deskripsi |
| :--- | :--- | :--- | :--- |
| `set_prop` | prop_name | val, obj | Mengatur nilai properti objek (mengambil ``value`` dan ``target_obj`` dari stack) |
| `instantiate` | class_name, argc | arg1, ... argN | Membuat instance baru dari suatu kelas dengan jumlah argumen konstruktor tertentu |
| `inspect_obj` | - | obj | Mencetak struktur internal Objek ke konsol |
| `inspect_array` | - | arr | Mencetak isi internal Array ke konsol |

::: warning
**Nightly Opcode**: Instruksi `instantiate` masih bersifat eksperimental. API dapat berubah sewaktu-waktu tanpa pemberitahuan pada versi `@next` atau `@nightly`.
:::