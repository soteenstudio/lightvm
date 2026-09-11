# Batas Waktu Eksekusi

`TimeBudget` membatasi waktu yang dapat digunakan Gazle untuk mengoptimalkan bytecode. Nilai ini tidak membatasi waktu eksekusi VM.

| Nilai | Nilai numerik | Batas optimizer |
| :--- | :--- | :--- |
| `Cheap` (bawaan) | `0` | 200 ms |
| `Normal` | `1` | 1.000 ms |
| `Expensive` | `2` | 5.000 ms |

Atur nilai dengan `setTimeBudget(TimeBudget.Cheap)` (atau `set_time_budget(TimeBudget::Cheap)` pada Rust) sebelum memanggil `tools().optimizeBytecode(...)` (atau `tools().optimize_bytecode(...)` pada Rust). Batas yang lebih besar memberi waktu lebih lama kepada tahap optimasi, tetapi tidak menjamin hasil optimasi tertentu.

::: info
Gunakan `SecurityConfig.maxTicks` untuk membatasi pekerjaan eksekusi. `TimeBudget` dan `maxTicks` berlaku pada tahap yang berbeda.
:::
