# Vector Roots

## Purpose

Evaluate square and cube roots element by element.

## Supported instructions

`sqrtv` and `cbrtv`.

## Stack operands and result

Each instruction replaces the top array with one result array.

## Supported numeric types

The type argument accepts `hlf`, `flt`, or `dbl`.

## Constraints and failure conditions

Missing operands produce `StackUnderflow`. Invalid arrays, nonnumeric elements, or unsupported types produce `TypeMismatch`. Negative square roots follow floating-point behavior.

## Examples

Applying `sqrtv dbl` to `[4, 9]` leaves `[2, 3]`.

## Related instructions

See [Roots](../mathematics/root) and [Vector Exponentiation](./exponentiation-vector).
