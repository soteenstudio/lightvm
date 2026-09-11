# Vector Roots
These instructions evaluate root functions element-wise on a vector (array).

They accept the float type directives `hlf`, `flt`, and `dbl`. Each instruction consumes one array operand and replaces it with an array containing the element-wise results in the selected type.

| Opcode | Arguments | Operands (stack) | Description |
| :--- | :--- | :--- | :--- |
| `sqrtv` | type | arr | Evaluates the square root for every array element |
| `cbrtv` | type | arr | Evaluates the cube root for every array element |
