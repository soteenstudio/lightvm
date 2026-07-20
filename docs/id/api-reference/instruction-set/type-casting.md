# Konversi Tipe (Konversi)
Instruksi-instruksi ini memfasilitasi konversi tipe data secara eksplisit di dalam lingkungan LightVM. Opcode-opcode ini penting untuk mengelola presisi variabel dan memastikan format data selaras dengan persyaratan operasional yang diharapkan selama eksekusi runtime.

| Kode Operasi | Operan (stack) | Deskripsi |
| :--- | :--- | :--- |
| `to_short` | val | Mengubah nilai menjadi Short (16-bit) |
| `to_integer` | val | Mengubah nilai menjadi Integer (32-bit) |
| `to_long` | val | Mengubah nilai menjadi Long (64-bit) |
| `to_octa` | val | Mengubah nilai menjadi Octa (128-bit Integer) |
| `to_half` | val | Mengubah nilai menjadi Half-precision (16-bit Float) |
| `to_float` | val | Mengubah nilai menjadi Float (32-bit) |
| `to_double` | val | Mengubah nilai menjadi Double (64-bit) |
| `to_string` | val | Mengubah nilai menjadi String |