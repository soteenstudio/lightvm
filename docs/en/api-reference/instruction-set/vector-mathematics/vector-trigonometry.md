# Vector Trigonometry

These instructions evaluate trigonometric, inverse, and hyperbolic functions element by element.

| Opcode | Arguments | Operands (stack) | Description |
| --- | --- | --- | --- |
| `sinv` / `cosv` / `tanv` | type | values | Replaces one numeric array with an equal-length array of element-wise sine, cosine, or tangent results; type must be `hlf`, `flt`, or `dbl`. |
| `asinv` / `acosv` / `atanv` | type | values | Replaces one numeric array with an equal-length array of element-wise inverse sine, cosine, or tangent results; type must be `hlf`, `flt`, or `dbl`. |
| `atan2v` | type | y, x | Replaces two equal-length numeric arrays with an equal-length array of `atan2(y[i], x[i])`; the `x` array is on top, and type must be `hlf`, `flt`, or `dbl`. |
| `sinhv` / `coshv` / `tanhv` | type | values | Replaces one numeric array with an equal-length array of element-wise hyperbolic sine, cosine, or tangent results; type must be `hlf`, `flt`, or `dbl`. |
| `asinhv` / `acoshv` / `atanhv` | type | values | Replaces one numeric array with an equal-length array of element-wise inverse hyperbolic sine, cosine, or tangent results; type must be `hlf`, `flt`, or `dbl`. |
