# LVM002 (Stack Underflow)
Runtime error type: `StackUnderflow`.

Error ini terjadi ketika sebuah instruksi mencoba mengambil nilai dari tumpukan kosong. Pesan runtime mengidentifikasi instruksi melalui `opcode` dan melaporkan instruction pointer tempat kegagalan terjadi.

Periksa keseimbangan operasi push dan pop pada bytecode. Pastikan setiap nilai yang digunakan oleh opcode tersebut telah didorong terlebih dahulu dan jalur alur kontrol sebelumnya meninggalkan tumpukan dalam keadaan yang konsisten.
