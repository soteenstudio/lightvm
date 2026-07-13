# Tipe Primitif
LightVM memerlukan definisi tipe eksplisit untuk instruksi tertentu guna mempertahankan eksekusi deterministik dan kinerja puncak.

| Tipe | Alias | Referensi | Jenis Nilai Target |
|------|---------|-----------|-------------------|
| `sht` | `i16` | Short | Bilangan bulat 16-bit (Int16) |
| `int` | `i32` | Integer | Bilangan bulat 32-bit (Int32) |
| `lng` | `i64` | Long | Bilangan bulat 64-bit (Int64) |
| `oct` | `i128` | Octa | Bilangan bulat 128-bit (Int128) |
| `hlf` | `f16` | Half | Bilangan Titik Mengambang 16-bit (Float16) |
| `flt` | `f32` | Float | Bilangan Titik Mengambang 32-bit (Float32) |
| `dbl` | `f64` | Double | Bilangan Titik Mengambang 64-bit (Float64) |
| `str` | - | String | Data string/teks |