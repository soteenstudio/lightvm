# Aritmatika & Logika
Instruksi-instruksi ini menyediakan operasi yang diperlukan untuk perhitungan numerik dan logika. Catatan: untuk optimalisasi, instruksi-instruksi ini memerlukan [Tipe Primitif](/id/api-references/primitive-types) (`sht`, `hlf`, `int`, `flt`, `lng`, `dbl`, `oct`) untuk mencegah VM menebak tipe data selama eksekusi.

| Kode Operasi | Argumen | Operan (stack) | Deskripsi |
| :--- | :--- | :--- | :--- |
| `add` / `sub` | type | val1, val2 | Penjumlahan atau Pengurangan |
| `mul` / `div` | type | val1, val2 | Perkalian atau Pembagian |
| `mod` | type | val1, val2 | Modulo (Sisa bagi) |
| `neg` | type | val | Negasi (-5 ke 5, atau sebaliknya) |
| `inc` / `dec` | name, type | - | Menambah/mengurangi isi variabel secara langsung (tanpa melalui stack) |
| `gt` / `lt` | type | val1, val2 | Lebih Besar Dari atau Lebih Kecil Dari |
| `ge` / `le` | type | val1, val2 | Lebih Besar/Kecil Dari atau Sama Dengan |
| `eq` / `neq` | type | val1, val2 | Sama Dengan atau Tidak Sama Dengan |
| `shl` / `shr` | type | val1, val2 | Operasi bitwise Shift Left atau Shift Right berdasarkan tipe data |
| `rol` / `ror` | type | val1, val2 | Operasi bitwise __Circular__ Shift Left atau Right (Rotate) berdasarkan tipe data |
| `and` / `or` | - | val1, val2 | Operasi logika Boolean (``&&`` / ``\ | \ | ``) |
| `xor` | - | val1, val2 | Operasi bitwise __Exclusive OR__ antara dua nilai |
| `not` | - | val1, val2 | Operasi bitwise __NOT__ (Inversi) pada satu nilai |
| `pow` | type | val1, val2 | Operasi pangkat umum (x^y) |
| `powi` | type | val1, val2 | Pangkat dengan eksponen integer |
| `powf` | type | val1, val2 | Pangkat dengan eksponen floating point |
| `sin` / `cos` | type | val1, val2 | Operasi trigonometri Sine atau Cosine |
| `tan` | type | val1, val2 | Operasi trigonometri Tangent |
