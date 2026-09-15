# LVM017 (Invalid Value)
Runtime error type: `InvalidValue`.

## Cause

This error occurs when bytecode attempts to push or operate on a value that the VM does not recognize or support. The value may use an unsupported data type, be corrupted, or have an invalid literal format.

## Runtime message

The runtime reports `Invalid value '<value>' encountered`, where `<value>` is the invalid value's type name. The diagnostic metadata includes the instruction pointer where the value was encountered and the error type `InvalidValue`.

## Resolution

Review the value and ensure its data type, integrity, and literal format conform to the types and formats supported by the VM.
