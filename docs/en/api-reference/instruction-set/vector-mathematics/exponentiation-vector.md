# Vector Exponentiation

These instructions evaluate powers and natural exponentials element by element.

| Opcode | Arguments | Operands (stack) | Description |
| --- | --- | --- | --- |
| `powv` | type | bases, exponents | Replaces two equal-length numeric arrays with integer element-wise powers; type must be `sht`, `int`, `lng`, or `oct`, and the top array supplies exponents. |
| `powiv` | type | bases, exponents | Replaces two equal-length numeric arrays with floating-point bases raised to integer exponents; type must be `hlf`, `flt`, or `dbl`, and the top array supplies exponents. |
| `powfv` | type | bases, exponents | Replaces two equal-length numeric arrays with floating-point element-wise powers; type must be `hlf`, `flt`, or `dbl`, and the top array supplies exponents. |
| `expv` | type | values | Replaces one numeric array with an equal-length array containing e raised to each element; type must be `hlf`, `flt`, or `dbl`. |
