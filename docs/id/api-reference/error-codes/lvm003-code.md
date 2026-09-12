# LVM003 (Opcode Tidak Valid)
Tipe eror runtime: `InvalidOpcode`.

## Penyebab

Error ini terjadi ketika parser atau eksekutor menemukan instruksi ilegal.

## Pesan runtime

Pesan runtime melaporkan instruksi yang tidak dikenali dalam `code` dan instruction pointer tempat instruksi tersebut ditemukan.

## Penyelesaian

Buat ulang atau perbaiki bytecode. Pastikan bytecode tidak rusak, ditujukan untuk versi VM saat ini, dan aliran instruksinya tersusun dengan benar.
