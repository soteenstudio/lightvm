# Alur Kontrol & Fungsi
Instruksi-instruksi ini mengelola alur eksekusi virtual machine, termasuk percabangan, cakupan fungsi (scoping), dan penghentian proses. Catatan: Fungsi di LightVM beroperasi dalam cakupan yang terisolasi; variabel lokal yang didefinisikan di dalam blok `func` akan dihancurkan setelah instruksi `return` untuk memastikan efisiensi memori.

| Kode Operasi | Argumen | Operan (stack) | Deskripsi |
| :--- | :--- | :--- | :--- |
| `jump` | target_ip | - | Melompat ke baris instruksi tertentu (Instruction Pointer) |
| `if_false` | target_ip | cond | Melompat jika nilai pada stack adalah false |
| `func` | name, argc, start, end, [params] | - | Definisi blok fungsi (cakupan) |
| `call` | name, argc | - | Memanggil fungsi dengan sejumlah argumen tertentu |
| `return` | - | val | Keluar dari fungsi dan kembali ke pemanggil |
| `stop` | - | - | Menghentikan semua proses VM (Halt) |