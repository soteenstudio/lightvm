# Trigonometri Vektor

Instruksi ini mengevaluasi fungsi trigonometri, invers, dan hiperbolik secara element-wise.

| Kode Operasi | Argumen | Operan (stack) | Deskripsi |
| --- | --- | --- | --- |
| `sinv` / `cosv` / `tanv` | tipe | nilai | Mengganti satu array numerik dengan array berpanjang sama berisi hasil sinus, kosinus, atau tangen element-wise; tipe harus `hlf`, `flt`, atau `dbl`. |
| `asinv` / `acosv` / `atanv` | tipe | nilai | Mengganti satu array numerik dengan array berpanjang sama berisi hasil invers sinus, kosinus, atau tangen element-wise; tipe harus `hlf`, `flt`, atau `dbl`. |
| `atan2v` | tipe | y, x | Mengganti dua array numerik dengan panjang sama dengan array berpanjang sama berisi `atan2(y[i], x[i])`; array `x` berada di atas, dan tipe harus `hlf`, `flt`, atau `dbl`. |
| `sinhv` / `coshv` / `tanhv` | tipe | nilai | Mengganti satu array numerik dengan array berpanjang sama berisi hasil sinus, kosinus, atau tangen hiperbolik element-wise; tipe harus `hlf`, `flt`, atau `dbl`. |
| `asinhv` / `acoshv` / `atanhv` | tipe | nilai | Mengganti satu array numerik dengan array berpanjang sama berisi hasil invers sinus, kosinus, atau tangen hiperbolik element-wise; tipe harus `hlf`, `flt`, atau `dbl`. |
