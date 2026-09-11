# Vector Logarithms
These instructions evaluate logarithmic functions element-wise on a vector (array).

They accept the float type directives `hlf`, `flt`, and `dbl`. Each instruction consumes one array operand and replaces it with an array containing the element-wise results in the selected type.

| Opcode | Arguments | Operands (stack) | Description |
| :--- | :--- | :--- | :--- |
| `lnv` | type | arr | Evaluates the natural logarithm for every array element |
| `log2v` | type | arr | Evaluates the base-2 logarithm for every array element |
| `log10v` | type | arr | Evaluates the base-10 logarithm for every array element |
