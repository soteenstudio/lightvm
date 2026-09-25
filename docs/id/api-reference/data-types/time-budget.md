# Anggaran Waktu

`TimeBudget` menentukan waktu yang dapat digunakan Gazle untuk mengoptimalkan bytecode. Nilai ini tidak membatasi waktu eksekusi VM.

| Nilai | Nilai numerik | Batas optimizer |
| :--- | :--- | :--- |
| `Cheap` (bawaan) | `0` | 200 ms |
| `Normal` | `1` | 1.000 ms |
| `Expensive` | `2` | 5.000 ms |
