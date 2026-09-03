# Vector Operations
These instructions perform element-wise arithmetic on vectors (arrays) and compute vector products used in linear algebra.

| Opcode | Arguments | Operands (stack) | Description |
| :--- | :--- | :--- | :--- |
| `addv` / `subv` | type | arr1, arr2 | Element-wise Addition or Subtraction of two vectors |
| `mulv` / `divv` | type | arr1, arr2 | Element-wise Multiplication or Division of two vectors |
| `modv` | type | arr1, arr2 | Element-wise Modulo (Remainder) of two vectors |
| `negv` | type | arr | Element-wise Negation of a vector |
| `dot` | type | arr1, arr2 | Dot product of two vectors (returns a scalar) |
| `cross` | type | arr1, arr2 | Cross product of two 3-element vectors (returns a 3-element vector) |
