# LVM003 (Invalid Opcode)
Runtime error type: `InvalidOpcode`.

This error occurs when the parser or executor encounters an illegal instruction. The runtime message reports the unrecognized instruction in `code` and the instruction pointer where it was encountered.

Regenerate or repair the bytecode. Confirm that it is not corrupted, that it targets the current VM version, and that the instruction stream is correctly aligned.
