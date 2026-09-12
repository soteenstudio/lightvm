# LVM010 (Unauthorized Module)
Runtime error type: `UnauthorizedModule`.

## Cause

This error occurs when bytecode attempts to import a module that is not whitelisted.

## Runtime message

The runtime message reports the rejected `module` and the instruction pointer of the import.

## Resolution

Add the module name to `allowed_imports` in `SecurityConfig` if it is trusted and required, or remove the import.
