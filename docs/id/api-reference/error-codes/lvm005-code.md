# LVM005 (Di Luar Batas)
Runtime error type: `OutOfBounds`.

Error ini terjadi ketika akses array atau objek menggunakan indeks di luar koleksi. Pesan runtime melaporkan `index` yang dicoba, `len` koleksi, dan instruction pointer tempat akses terjadi.

Batasi indeks ke rentang `0` hingga `len - 1`. Jika `len` adalah `0`, koleksi kosong dan tidak ada indeks yang valid. Periksa perhitungan indeks untuk error off-by-one.
