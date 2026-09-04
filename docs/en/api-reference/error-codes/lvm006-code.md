# LVM006 (Invalid Jump Target)

Runtime error type: `InvalidJumpTarget`.

This error occurs when control flow attempts to jump outside the bytecode. The runtime message reports the requested `target`, the bytecode `len`, and the instruction pointer of the jump.

Correct the jump offset or target so it points within the bytecode. Regenerate the bytecode if the target was produced by corrupted instructions or incorrect control-flow mapping.
