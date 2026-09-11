# LVM001 (Stack Overflow)
Tipe eror runtime: `StackOverflow`.

## Penyebab

Error ini terjadi ketika operasi stack mencoba menambahkan nilai setelah stack mencapai kapasitasnya. `SecurityConfig.max_stack_size` mengatur kapasitas awal stack untuk eksekusi; nilai `0` membuat kapasitas bawaan tetap berlaku.

Instruksi `val` juga memunculkan `StackOverflow` ketika indeks variabelnya mencapai batas variabel tetap. Instruksi `make_obj` memunculkan error yang sama ketika kapasitas objek yang diminta melebihi batas implementasi.

## Pesan runtime

Pesan runtime melaporkan `limit`, dan runtime mencatat instruction pointer tempat `StackOverflow` terjadi.

## Penyelesaian

Kurangi pertumbuhan stack yang tidak disengaja, termasuk rekursi tanpa batas. Jika beban kerja memerlukan stack yang lebih besar, konfigurasikan `SecurityConfig.max_stack_size`. Pastikan indeks variabel dan permintaan kapasitas objek tetap berada dalam batas yang didukung.
