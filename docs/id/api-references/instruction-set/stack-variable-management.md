# Manajemen Stack & Variabel
Tabel berikut merinci operasi inti untuk manipulasi stack dan manajemen siklus hidup variabel. Instruksi-instruksi ini merupakan dasar dari arsitektur berbasis stack pada LightVM, yang mengatur bagaimana data disimpan, diambil, dan disusun selama eksekusi berlangsung.

| Kode Operasi | Argumen | Operand (stack) | Deskripsi |
|--------|-----------|------------------|-------------|
| `push`   | value     | - |Inserting data into the stack |
| `val`    | name      | - | Declaring a new variable |
| `set`    | name      | val | Take the top stack and then save it to the variable ``name`` |
| `get`    | name      | - | Take the contents of the ``name`` variable and push it onto the stack |
| `dup`    | -         | val | Duplicate the top value in the stack |
| `swap` | - | val1, val2 | Swap the top stack with the bottom stack |