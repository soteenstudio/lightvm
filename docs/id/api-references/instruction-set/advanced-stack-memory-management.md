# Manajemen Memori & Stack Lanjutan
Instruksi-instruksi ini mengelola jejak memori dan kapasitas stack evaluasi. Instruksi ini menyediakan kontrol yang mendalam terhadap alokasi sumber daya, membantu mengoptimalkan performa dan mencegah fragmentasi memori di dalam virtual machine.

| Operasi Kode | Argumen | Operan (stack) | Deskripsi |
| :--- | :--- | :--- | :--- |
| `init_stack` | size | - | Menginisialisasi kapasitas memori stack evaluasi di awal eksekusi VM untuk mencegah realokasi |
| `shrink` | - | target, length | Mengurangi kapasitas stack agar sesuai dengan panjangnya saat ini |
| `truncate` | - | target_size | Membersihkan/mengatur ulang elemen stack secara efisien |