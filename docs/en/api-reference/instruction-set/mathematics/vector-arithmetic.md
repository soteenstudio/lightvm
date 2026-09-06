# Vector Arithmetic
These instructions perform element-wise arithmetic on vectors (arrays).

| Opcode | Arguments | Operands (stack) | Description |
| :--- | :--- | :--- | :--- |
| `addv` / `subv` | type | arr1, arr2 | Element-wise Addition or Subtraction of two vectors |
| `mulv` / `divv` | type | arr1, arr2 | Element-wise Multiplication or Division of two vectors |
| `modv` | type | arr1, arr2 | Element-wise Modulo (Remainder) of two vectors |
| `negv` | type | arr | Element-wise Negation of a vector |
| `sinv` | type | arr | Evaluates the sine function for every array element |
| `cosv` | type | arr | Evaluates the cosine function for every array element |
| `tanv` | type | arr | Evaluates the tangent function for every array element |
| `asinv` | type | arr | Evaluates the inverse sine function for every array element |
| `acosv` | type | arr | Evaluates the inverse cosine function for every array element |
| `atanv` | type | arr | Evaluates the inverse tangent function for every array element |
| `atan2v` | type | arrX, arrY | Evaluates the two-argument inverse tangent function for every pair of array elements |
| `sinhv` | type | arr | Evaluates the hyperbolic sine function for every array element |
| `coshv` | type | arr | Evaluates the hyperbolic cosine function for every array element |
| `tanhv` | type | arr | Evaluates the hyperbolic tangent function for every array element |
| `asinhv` | type | arr | Evaluates the inverse hyperbolic sine function for every array element |
| `acoshv` | type | arr | Evaluates the inverse hyperbolic cosine function for every array element |
| `atanhv` | type | arr | Evaluates the inverse hyperbolic tangent function for every array element |
