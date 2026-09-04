# LVM014 (Excessive Nop Padding)
Runtime error type: `ExcessiveNopPadding`.

This error occurs when `Nop` instructions exceed 10% of the total instructions. Excessive padding can indicate obfuscation, an attempt to bypass analysis, or artificial bytecode inflation. This error reports instruction pointer `0`.

Review or regenerate the bytecode and remove unnecessary `Nop` instructions so their proportion stays within the permitted threshold.
