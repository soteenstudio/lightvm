# Kapabilitas

`Capability` memberi izin kepada operasi host untuk mengakses fungsi `LightVM` yang dilindungi. Teruskan nilai yang diperlukan melalui `VMConfig.caps`. TypeScript menggunakan `Observe` secara bawaan.

| Kapabilitas | Nilai numerik | Operasi yang dilindungi |
| --- | ---: | --- |
| `Control` | `0` | `run`, `compile`, `embedded`, ekspor fungsi, `provide`, menghapus output, dan optimasi bytecode |
| `Observe` | `1` | `inspect`, membaca output, dan ekspor variabel |
| `Debug` | `2` | Benchmark |
| `Unsafe` | `3` | `halt` |

## Konfigurasi

```ts
const vm = new LightVM({
  caps: [Capability.Control, Capability.Observe],
})
```

Kapabilitas yang tidak diberikan menyebabkan operasi terlindungi gagal. Kapabilitas tidak menggantikan batas sumber daya dalam `SecurityConfig`; konfigurasikan keduanya untuk bytecode yang tidak tepercaya.

::: warning
Berikan hanya kapabilitas yang diperlukan aplikasi host. `Unsafe` mengizinkan penghentian eksternal, tetapi tidak mengaktifkan `SecurityConfig.unsafeMode`.
:::
