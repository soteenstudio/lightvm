# Anggaran Waktu

`TimeBudget` menentukan waktu yang dapat digunakan Gazle untuk mengoptimalkan bytecode. Nilai ini tidak membatasi waktu eksekusi VM.

| Nilai | Nilai numerik | Batas optimizer |
| :--- | :--- | :--- |
| `Cheap` (bawaan) | `0` | 200 ms |
| `Normal` | `1` | 1.000 ms |
| `Expensive` | `2` | 5.000 ms |

Atur anggaran sebelum memulai optimasi. Gunakan `setTimeBudget(TimeBudget.Cheap)` sebelum `tools().optimizeBytecode(...)`, atau `set_time_budget(TimeBudget::Cheap)` sebelum `tools().optimize_bytecode(...)` untuk Rust.

Anggaran yang lebih besar memungkinkan optimasi berlangsung lebih lama. Nilai ini tidak menjamin hasil tertentu.

::: info
`SecurityConfig.maxTicks` membatasi pekerjaan eksekusi VM secara terpisah dari `TimeBudget`.
:::
