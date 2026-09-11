# LVM006 (Target Lompatan Tidak Valid)
Tipe eror runtime: `InvalidJumpTarget`.

## Penyebab

Error ini terjadi ketika alur kontrol mencoba melompat ke luar bytecode.

## Pesan runtime

Pesan runtime melaporkan `target` yang diminta, `len` bytecode, dan instruction pointer dari lompatan tersebut.

## Penyelesaian

Perbaiki offset atau target lompatan agar mengarah ke dalam bytecode. Buat ulang bytecode jika target dihasilkan oleh instruksi yang rusak atau pemetaan alur kontrol yang keliru.
