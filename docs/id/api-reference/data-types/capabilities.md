# Kapabilitas

`Capability` memberi izin kepada operasi host untuk mengakses fungsi `LightVM` yang dilindungi. Teruskan nilai yang diperlukan melalui `VMConfig.caps`. TypeScript menggunakan `Observe` secara bawaan.

| Kapabilitas | Nilai numerik | Operasi yang dilindungi |
| --- | ---: | --- |
| `Control` | `0` | `run`, `compile`, `embedded`, ekspor fungsi, `provide`, menghapus output, dan optimasi bytecode |
| `Observe` | `1` | `inspect`, membaca output, dan ekspor variabel |
| `Debug` | `2` | Benchmark, membaca catatan panic, dan menghapus catatan panic |
| `Unsafe` | `3` | `halt`, yang menghentikan VM |

::: warning
Kegagalan memberikan kapabilitas yang sesuai akan menyebabkan operasi host yang dilindungi gagal dijalankan atau memicu error saat runtime. Pastikan untuk selalu memberikan izin kapabilitas minimum yang dibutuhkan oleh aplikasi Anda.
:::
