# LVM010 (Modul Tidak Diizinkan)
Runtime error type: `UnauthorizedModule`.

Error ini terjadi ketika bytecode mencoba mengimpor modul yang tidak ada dalam daftar yang diizinkan. Pesan runtime melaporkan `module` yang ditolak dan instruction pointer dari impor tersebut.

Tambahkan nama modul ke `allowed_imports` dalam `SecurityConfig` jika modul tersebut tepercaya dan diperlukan, atau hapus impornya.
