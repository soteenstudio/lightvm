# Perbandingan dan Logika

Instruksi ini membandingkan nilai skalar dan mengevaluasi truthiness nilai.

| Kode Operasi | Argumen | Operan (stack) | Deskripsi |
| --- | --- | --- | --- |
| `gt` | tipe | kiri, kanan | Mengganti dua operan dengan hasil apakah `left > right` setelah konversi ke tipe yang dipilih. |
| `lt` | tipe | kiri, kanan | Mengganti dua operan dengan hasil apakah `left < right` setelah konversi ke tipe yang dipilih. |
| `ge` | tipe | kiri, kanan | Mengganti dua operan dengan hasil apakah `left >= right` setelah konversi ke tipe yang dipilih. |
| `le` | tipe | kiri, kanan | Mengganti dua operan dengan hasil apakah `left <= right` setelah konversi ke tipe yang dipilih. |
| `eq` | tipe | kiri, kanan | Mengganti dua operan dengan hasil apakah keduanya sama setelah konversi ke tipe yang dipilih. |
| `neq` | tipe | kiri, kanan | Mengganti dua operan dengan hasil apakah keduanya berbeda setelah konversi ke tipe yang dipilih. |
| `and` | - | kiri, kanan | Mengganti dua operan dengan AND Boolean dari truthiness keduanya. |
| `or` | - | kiri, kanan | Mengganti dua operan dengan OR Boolean dari truthiness keduanya. |
| `xor` | - | kiri, kanan | Mengganti dua operan dengan OR eksklusif Boolean dari truthiness keduanya. |
| `not` | - | nilai | Mengganti satu operan dengan negasi Boolean dari truthiness-nya. |
