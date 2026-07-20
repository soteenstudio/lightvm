# I/O Dasar & Kontrol Perulangan
Instruksi-instruksi ini mengelola operasi input/output standar dan kontrol alur di dalam virtual machine. Instruksi ini menangani pencetakan data ke konsol, pembacaan input pengguna, dan pengelolaan siklus eksekusi perulangan.

| Operasi Kode | Argumen | Operan (stack) | Deskripsi |
| :--- | :--- | :--- | :--- |
| `print` | - | val | Mencetak nilai teratas stack ke konsol tanpa baris baru |
| `println` | - | val | Mencetak nilai teratas stack ke konsol dengan baris baru |
| `stdin` | - | val | Membaca satu baris dari input standar, menghapus karakter baris baru di akhir, dan mendorong string hasil ke stack |
| `stdout` | - | val | Mengambil (pop) nilai teratas dari stack (harus String) dan mencetaknya ke konsol tanpa baris baru |
| `stdoutln` | - | val | Mengambil (pop) nilai teratas dari stack (harus String) dan mencetaknya ke konsol diikuti baris baru |
| `clear_screen` | - | - | Membersihkan layar terminal dan mengatur ulang posisi kursor ke sudut kiri atas menggunakan kode ANSI escape (\x1B[2J\x1B[1H) |
| `break` | - | target_ip | Menghentikan perulangan dan melompat ke target_ip yang ditentukan |
| `nop` | - | - | Instruksi kosong (biasanya untuk placeholder atau perataan) |
