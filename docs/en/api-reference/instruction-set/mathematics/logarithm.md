# Logarithms

## Purpose

Evaluate scalar logarithmic functions.

## Supported instructions

`ln`, `log2`, and `log10`.

## Stack operands and result

Each instruction replaces the top stack value with its logarithm.

## Supported numeric types

The type argument accepts `hlf`, `flt`, or `dbl`.

## Constraints and failure conditions

An empty stack produces `StackUnderflow`. A nonnumeric value or unsupported type argument produces `TypeMismatch`. Domain errors follow floating-point behavior.

## Examples

With `1` on top, `ln dbl` leaves `0`.

## Related instructions

See [Exponentiation](./exponentiation) and [Vector Logarithms](../vector-mathematics/logarithm-vector).
