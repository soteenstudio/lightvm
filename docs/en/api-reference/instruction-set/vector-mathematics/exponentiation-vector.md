# Vector Exponentiation
This instruction evaluates the natural exponential function element-wise on a vector (array).

It accepts the float type directives `hlf`, `flt`, and `dbl`. The instruction consumes one array operand and replaces it with an array containing the element-wise results in the selected type.

| Opcode | Arguments | Operands (stack) | Description |
| :--- | :--- | :--- | :--- |
| `expv` | type | arr | Raises e to the power of every array element |
