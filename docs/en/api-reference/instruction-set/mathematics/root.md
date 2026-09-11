# Roots

## Purpose

Evaluate scalar square and cube roots.

## Supported instructions

`sqrt` and `cbrt`.

## Stack operands and result

Each instruction replaces the top stack value with its root.

## Supported numeric types

The type argument accepts `hlf`, `flt`, or `dbl`.

## Constraints and failure conditions

An empty stack produces `StackUnderflow`. A nonnumeric value or unsupported type argument produces `TypeMismatch`. Negative square roots follow floating-point behavior.

## Examples

With `9` on top, `sqrt dbl` leaves `3`.

## Related instructions

See [Exponentiation](./exponentiation) and [Vector Roots](../vector-mathematics/root-vector).
