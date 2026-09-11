# Vector Bitwise Operations
These instructions perform bitwise shifts and rotations element-wise on two vectors (arrays).

They accept the integer type directives `sht`, `int`, `lng`, and `oct`. Each instruction consumes the equal-length arrays `arr1` and `arr2`, applies the operation to corresponding elements, and replaces both operands with one result array in the selected type.

| Opcode | Arguments | Operands (stack) | Description |
| :--- | :--- | :--- | :--- |
| `shlv` | type | arr1, arr2 | Shifts each element of arr1 left by the corresponding element of arr2 |
| `shrv` | type | arr1, arr2 | Shifts each element of arr1 right by the corresponding element of arr2 |
| `rolv` | type | arr1, arr2 | Rotates each element of arr1 left by the corresponding element of arr2 |
| `rorv` | type | arr1, arr2 | Rotates each element of arr1 right by the corresponding element of arr2 |
