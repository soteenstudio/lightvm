# LVM014 (Padding Nop Berlebihan)
Tipe eror runtime: `ExcessiveNopPadding`.

Error ini terjadi ketika instruksi `Nop` melebihi 10% dari total instruksi. Padding berlebihan dapat menunjukkan obfuscation, upaya melewati analisis, atau pembesaran bytecode secara artifisial. Error ini melaporkan instruction pointer `0`.

Tinjau atau buat ulang bytecode dan hapus instruksi `Nop` yang tidak diperlukan agar proporsinya tetap dalam ambang batas yang diizinkan.
