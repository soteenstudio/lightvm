# Vector Arithmetic

These instructions perform element-wise arithmetic on arrays.

| Opcode | Arguments | Operands (stack) | Description |
| --- | --- | --- | --- |
| `addv` | type | left, right | Replaces two equal-length numeric arrays with an array of element-wise sums in the selected type. |
| `subv` | type | left, right | Replaces two equal-length numeric arrays with an array of `left[i] - right[i]` in the selected type. |
| `mulv` | type | left, right | Replaces two equal-length numeric arrays with an array of element-wise products in the selected type. |
| `divv` | type | left, right | Replaces two equal-length numeric arrays with an array of `left[i] / right[i]` in the selected type. |
| `modv` | type | left, right | Replaces two equal-length numeric arrays with an array of element-wise remainders in the selected type. |
| `negv` | type | values | Replaces one numeric array with an equal-length array containing each element's negation in the selected type. |
