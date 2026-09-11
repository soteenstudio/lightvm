# LVM004 (Ketidakcocokan Tipe)
Tipe eror runtime: `TypeMismatch`.

## Penyebab

Error ini terjadi ketika sebuah instruksi menerima nilai dengan tipe yang salah.

## Pesan runtime

Pesan runtime melaporkan tipe yang diperlukan dalam `expected`, tipe aktual dalam `found`, dan instruction pointer tempat ketidakcocokan terjadi.

## Penyelesaian

Perbarui nilai yang diteruskan ke instruksi agar sesuai dengan tipe dan signature parameter yang diperlukan instruksi tersebut.
