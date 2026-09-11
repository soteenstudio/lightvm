# Eksponensiasi Vektor

Instruksi ini mengevaluasi perpangkatan dan eksponensial alami secara element-wise.

| Kode Operasi | Argumen | Operan (stack) | Deskripsi |
| --- | --- | --- | --- |
| `powv` | tipe | basis, eksponen | Mengganti dua array numerik dengan panjang sama dengan perpangkatan integer element-wise; tipe harus `sht`, `int`, `lng`, atau `oct`, dan array teratas menyediakan eksponen. |
| `powiv` | tipe | basis, eksponen | Mengganti dua array numerik dengan panjang sama dengan basis floating-point yang dipangkatkan dengan eksponen integer; tipe harus `hlf`, `flt`, atau `dbl`, dan array teratas menyediakan eksponen. |
| `powfv` | tipe | basis, eksponen | Mengganti dua array numerik dengan panjang sama dengan perpangkatan floating-point element-wise; tipe harus `hlf`, `flt`, atau `dbl`, dan array teratas menyediakan eksponen. |
| `expv` | tipe | nilai | Mengganti satu array numerik dengan array berpanjang sama yang berisi e pangkat setiap elemen; tipe harus `hlf`, `flt`, atau `dbl`. |
