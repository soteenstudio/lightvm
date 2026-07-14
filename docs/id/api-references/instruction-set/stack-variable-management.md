# Manajemen Stack & Variabel
Instruksi-instruksi ini merinci operasi inti untuk manipulasi stack dan pengelolaan siklus hidup variabel. Instruksi ini bersifat fundamental bagi arsitektur berbasis stack pada LightVM, yang mengatur bagaimana data disimpan, diambil, dan disusun selama eksekusi.

| Kode Operasi | Argumen | Operand (stack) | Deskripsi |
|--------|-----------|------------------|-------------|
| `push`   | value     | - | Memasukkan data ke dalam stack |
| `val`    | name      | - | Mendeklarasikan variabel baru |
| `set`    | name      | val | Mengambil nilai teratas stack lalu menyimpannya ke variabel `name` |
| `get`    | name      | - | Take the contents of the ``name`` variable and push it onto the stack |
| `dup`    | -         | val | Duplicate the top value in the stack |
| `swap` | - | val1, val2 | Swap the top stack with the bottom stack |