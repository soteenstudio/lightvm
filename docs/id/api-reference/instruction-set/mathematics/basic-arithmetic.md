# Aritmatika Dasar

Instruksi ini melakukan aritmatika skalar dan memperbarui variabel numerik.

| Kode Operasi | Argumen | Operan (stack) | Deskripsi |
| --- | --- | --- | --- |
| `add` | tipe | kiri, kanan | Mengganti dua operan numerik dengan `left + right` dalam tipe yang dipilih. |
| `sub` | tipe | kiri, kanan | Mengganti dua operan numerik dengan `left - right` dalam tipe yang dipilih. |
| `mul` | tipe | kiri, kanan | Mengganti dua operan numerik dengan `left * right` dalam tipe yang dipilih. |
| `div` | tipe | kiri, kanan | Mengganti dua operan numerik dengan `left / right` dalam tipe yang dipilih. |
| `mod` | tipe | kiri, kanan | Mengganti dua operan numerik dengan sisa dari `left / right` dalam tipe yang dipilih. |
| `neg` | tipe | nilai | Mengganti satu operan numerik dengan negasinya dalam tipe yang dipilih. |
| `inc` | nama, tipe | - | Menambah variabel numerik bernama atau berindeks dan mendorong nilai terbarunya. |
| `dec` | nama, tipe | - | Mengurangi variabel numerik bernama atau berindeks tanpa mengubah stack. |
