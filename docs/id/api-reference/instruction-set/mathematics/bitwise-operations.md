# Operasi Bitwise
Instruksi-instruksi ini melakukan manipulasi biner level rendah, pergeseran bit, dan gerbang logika bitwise.

| Kode Operasi | Argumen | Operan (stack) | Deskripsi |
| :--- | :--- | :--- | :--- |
| `shl` / `shr` | tipe | val1, val2 | Operasi bitwise Geser Kiri atau Geser Kanan |
| `rol` / `ror` | tipe | val1, val2 | Geser Kiri atau Kanan secara __Siklik__ (Rotate) |
| `xor` | - | val1, val2 | Operasi bitwise __Exclusive OR__ antara dua nilai |
| `not` | - | val1, val2 | Operasi bitwise __NOT__ (Inversi) pada satu nilai |