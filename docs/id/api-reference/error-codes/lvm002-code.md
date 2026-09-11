# LVM002 (Stack Underflow)
Tipe eror runtime: `StackUnderflow`.

## Penyebab

Error ini terjadi ketika sebuah instruksi mencoba mengambil nilai dari tumpukan kosong.

## Pesan runtime

Pesan runtime mengidentifikasi instruksi melalui `opcode` dan melaporkan instruction pointer tempat kegagalan terjadi.

## Penyelesaian

Periksa keseimbangan operasi push dan pop pada bytecode. Pastikan setiap nilai yang digunakan oleh opcode tersebut telah didorong terlebih dahulu dan jalur alur kontrol sebelumnya meninggalkan tumpukan dalam keadaan yang konsisten.
