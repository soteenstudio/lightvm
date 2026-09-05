# LVM500 (Error Sistem)
Tipe eror runtime: `SystemError`.

Error ini terjadi ketika operasi yang melibatkan sistem host atau sistem operasi gagal. Pesannya berasal dari error sistem host dan dapat berbeda-beda. Runtime menggunakan instruction pointer `0` untuk error ini, tetapi tidak menyertakan instruction pointer atau tipe error dalam pesan yang ditampilkan.

Periksa izin lingkungan, memori yang tersedia, dan log sistem untuk menemukan kegagalan yang mendasarinya, lalu perbaiki masalah sistem host yang dilaporkan.
