# LVM006 (Target Lompatan Tidak Valid)
Runtime error type: `InvalidJumpTarget`.

Error ini terjadi ketika alur kontrol mencoba melompat ke luar bytecode. Pesan runtime melaporkan `target` yang diminta, `len` bytecode, dan instruction pointer dari lompatan tersebut.

Perbaiki offset atau target lompatan agar mengarah ke dalam bytecode. Buat ulang bytecode jika target dihasilkan oleh instruksi yang rusak atau pemetaan alur kontrol yang keliru.
