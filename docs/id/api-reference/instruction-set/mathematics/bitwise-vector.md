# Operasi Bitwise Vektor
Instruksi-instruksi ini melakukan pergeseran dan rotasi bitwise secara element-wise pada dua vektor (array).

Instruksi ini menerima direktif tipe integer `sht`, `int`, `lng`, dan `oct`. Setiap instruksi menggunakan array `arr1` dan `arr2` yang memiliki panjang sama, menerapkan operasi pada elemen yang bersesuaian, lalu mengganti kedua operan dengan satu array hasil dalam tipe yang dipilih.

| Kode Operasi | Argumen | Operan (stack) | Deskripsi |
| :--- | :--- | :--- | :--- |
| `shlv` | type | arr1, arr2 | Menggeser setiap elemen arr1 ke kiri sebanyak elemen arr2 yang bersesuaian |
| `shrv` | type | arr1, arr2 | Menggeser setiap elemen arr1 ke kanan sebanyak elemen arr2 yang bersesuaian |
| `rolv` | type | arr1, arr2 | Merotasi setiap elemen arr1 ke kiri sebanyak elemen arr2 yang bersesuaian |
| `rorv` | type | arr1, arr2 | Merotasi setiap elemen arr1 ke kanan sebanyak elemen arr2 yang bersesuaian |
