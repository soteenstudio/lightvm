# Vector Logarithms

## Purpose

Evaluate logarithms element by element.

## Supported instructions

`lnv`, `log2v`, and `log10v`.

## Stack operands and result

Each instruction replaces the top array with one result array.

## Supported numeric types

The type argument accepts `hlf`, `flt`, or `dbl`.

## Constraints and failure conditions

Missing operands produce `StackUnderflow`. Invalid arrays, nonnumeric elements, or unsupported types produce `TypeMismatch`. Domain errors follow floating-point behavior.

## Examples

Applying `lnv dbl` to `[1, 1]` leaves `[0, 0]`.

## Related instructions

See [Logarithms](../mathematics/logarithm) and [Vector Exponentiation](./exponentiation-vector).
