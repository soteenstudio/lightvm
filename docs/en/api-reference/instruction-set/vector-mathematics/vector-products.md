# Vector Products

## Purpose

Compute dot and cross products.

## Supported instructions

`dot` and `cross`.

## Stack operands and result

Both consume two arrays, using the next-to-top array as the left operand. `dot` leaves a scalar; `cross` leaves an array.

## Supported numeric types

The type argument accepts `sht`, `int`, `lng`, `oct`, `hlf`, `flt`, or `dbl`.

## Constraints and failure conditions

`dot` requires equal-length arrays. `cross` requires two arrays of exactly three elements. Missing operands produce `StackUnderflow`; invalid arrays, nonnumeric elements, length violations, or unsupported types produce `TypeMismatch`.

## Examples

Applying `dot int` to `[1, 2]` below `[3, 4]` leaves `11`.

## Related instructions

See [Vector Arithmetic](./vector-arithmetic) and [Vector Trigonometry](./vector-trigonometry).
