# LVM014 (Excessive Nop Padding)
Runtime error type: `ExcessiveNopPadding`.

## Cause

This error occurs when `Nop` instructions exceed 10% of the total instructions. Excessive padding can indicate obfuscation, an attempt to bypass analysis, or artificial bytecode inflation.

## Runtime message

The runtime reports instruction pointer `0`.

## Resolution

Review or regenerate the bytecode and remove unnecessary `Nop` instructions so their proportion stays within the permitted threshold.
