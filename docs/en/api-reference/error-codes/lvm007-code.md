# LVM007 (Feature Restricted)

Runtime error type: `FeatureRestricted`.

This error occurs when bytecode uses a nightly or experimental opcode while nightly mode is disabled. The runtime message reports the restricted `feature` and the instruction pointer where it was used.

Enable nightly mode in `VmConfig` when the restricted feature is intentionally required, or replace it with a stable instruction.
