# LVM004 (Type Mismatch)
Runtime error type: `TypeMismatch`.

## Cause

This error occurs when an instruction receives a value of the wrong type.

## Runtime message

The runtime message reports the required type in `expected`, the actual type in `found`, and the instruction pointer where the mismatch occurred.

## Resolution

Update the value passed to the instruction so it matches the instruction's required type and parameter signature.
