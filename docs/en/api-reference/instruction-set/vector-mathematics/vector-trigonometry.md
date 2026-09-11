# Vector Trigonometry

## Purpose

Evaluate trigonometric, inverse, and hyperbolic functions element by element.

## Supported instructions

`sinv`, `cosv`, `tanv`, `asinv`, `acosv`, `atanv`, `atan2v`, `sinhv`, `coshv`, `tanhv`, `asinhv`, `acoshv`, and `atanhv`.

## Stack operands and result

Unary instructions replace the top array. `atan2v` consumes two equal-length arrays and leaves one result array.

## Supported numeric types

The type argument accepts `hlf`, `flt`, or `dbl`.

## Constraints and failure conditions

`atan2v` requires equal-length arrays. Missing operands produce `StackUnderflow`; invalid arrays, nonnumeric elements, unequal lengths, or unsupported types produce `TypeMismatch`. Domain errors follow floating-point behavior.

## Examples

Applying `sinv dbl` to `[0, 0]` leaves `[0, 0]`.

## Related instructions

See [Trigonometry](../mathematics/trigonometry) and [Vector Products](./vector-products).
