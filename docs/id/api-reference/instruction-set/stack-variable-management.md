# Manajemen Stack & Variabel
Instruksi-instruksi ini merinci operasi inti untuk manipulasi stack dan pengelolaan siklus hidup variabel. Instruksi ini bersifat fundamental bagi arsitektur berbasis stack pada LightVM, yang mengatur bagaimana data disimpan, diambil, dan disusun selama eksekusi.

| Kode Operasi | Argumen | Operan (stack) | Deskripsi |
|--------|-----------|------------------|-------------|
| `push`   | value     | - | Memasukkan data ke dalam stack |
| `val`    | name      | - | Mendeklarasikan variabel baru |
| `set`    | name      | val | Mengambil nilai teratas stack lalu menyimpannya ke variabel `name` |
| `get`    | name      | - | Mengambil isi variabel `name` lalu memasukkannya ke stack |
| `dup`    | -         | val | Menduplikasi nilai teratas di stack |
| `swap` | - | val1, val2 | Menukar posisi nilai teratas dan nilai di bawahnya di stack |