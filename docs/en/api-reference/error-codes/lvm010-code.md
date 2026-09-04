# LVM010 (Unauthorized Module)

Runtime error type: `UnauthorizedModule`.

This error occurs when bytecode attempts to import a module that is not whitelisted. The runtime message reports the rejected `module` and the instruction pointer of the import.

Add the module name to `allowed_imports` in `SecurityConfig` if it is trusted and required, or remove the import.
