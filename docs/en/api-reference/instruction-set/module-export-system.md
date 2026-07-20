# Module & Export System
These instructions control the module and export system within the virtual machine. They provide the mechanism for modularizing code, enabling the import of external libraries and the public exposure of internal components.

| Opcode | Arguments | Operands (stack) | Description |
| :--- | :--- | :--- | :--- |
| `import` | module_name, alias_idx | target, length | Importing external libraries/modules into a specific variable index |
| `export` | name | - | Mark a function or variable to be accessible from outside the VM |

::: warning
**Nightly Opcode**: The `export` and `import` instructions are still experimental. The API may change without notice in the `@next` or `@nightly` version.
:::