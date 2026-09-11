# Trigonometry

These instructions evaluate scalar trigonometric, inverse, and hyperbolic functions.

| Opcode | Arguments | Operands (stack) | Description |
| --- | --- | --- | --- |
| `sin` / `cos` / `tan` | type | value | Replaces one operand with its sine, cosine, or tangent; type must be `hlf`, `flt`, or `dbl`. |
| `asin` / `acos` / `atan` | type | value | Replaces one operand with its inverse sine, cosine, or tangent; type must be `hlf`, `flt`, or `dbl`. |
| `atan2` | type | y, x | Replaces two operands with `atan2(y, x)`; `x` is on top, and type must be `hlf`, `flt`, or `dbl`. |
| `sinh` / `cosh` / `tanh` | type | value | Replaces one operand with its hyperbolic sine, cosine, or tangent; type must be `hlf`, `flt`, or `dbl`. |
| `asinh` / `acosh` / `atanh` | type | value | Replaces one operand with its inverse hyperbolic sine, cosine, or tangent; type must be `hlf`, `flt`, or `dbl`. |
