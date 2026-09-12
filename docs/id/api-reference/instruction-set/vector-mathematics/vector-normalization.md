# Normalisasi Vektor

Instruksi `normalize` menskalakan vektor floating-point menjadi panjang satu unit.

| Kode Operasi | Argumen | Operan (stack) | Deskripsi |
| --- | --- | --- | --- |
| `normalize` | tipe | nilai | Mengganti satu array floating-point dengan array berpanjang sama dalam tipe yang dipilih. Tipe harus `hlf`, `flt`, atau `dbl`. |

Instruksi ini mengambil array teratas dari stack dan mendorong vektor hasil normalisasinya. Vektor bukan nol menghasilkan vektor dengan panjang satu unit. Vektor nol menghasilkan vektor nol dengan panjang yang sama sehingga tidak terjadi pembagian dengan nol.

Operan harus berupa array numerik dan argumen tipe harus `hlf`, `flt`, atau `dbl`. Operan atau tipe lain menghasilkan error [`TypeMismatch`](/id/api-reference/error-codes/lvm004-code). Operan yang tidak tersedia menghasilkan error [`StackUnderflow`](/id/api-reference/error-codes/lvm002-code). Stack tidak berubah jika operan tidak valid.

## Contoh

Setiap contoh menormalisasi `[3, 4]` menjadi sekitar `[0.6, 0.8]`, dengan elemen disimpan dalam tipe numerik yang dipilih.

```json
[["push", [3.0, 4.0]], ["normalize", "hlf"]]
```

```json
[["push", [3.0, 4.0]], ["normalize", "flt"]]
```

```json
[["push", [3.0, 4.0]], ["normalize", "dbl"]]
```
