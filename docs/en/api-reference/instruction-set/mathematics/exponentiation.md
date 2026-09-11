# Exponentiation

## Purpose

Evaluate scalar powers and the natural exponential function.

## Supported instructions

`pow`, `powi`, `powf`, and `exp`.

## Stack operands and result

`pow`, `powi`, and `powf` consume a base below an exponent and replace both with one result. `exp` replaces the top value with e raised to that value.

## Supported numeric types

`pow` accepts all numeric types, `powi` and `powf` apply their implemented exponent conversions, and `exp` accepts `hlf`, `flt`, or `dbl`.

## Constraints and failure conditions

Missing operands produce `StackUnderflow`. Values or type arguments unsupported by an instruction produce `TypeMismatch`.

## Examples

With `2` below `3`, `pow int` leaves `8`. With `0` on top, `exp dbl` leaves `1`.

## Related instructions

See [Basic Arithmetic](./basic-arithmetic) and [Vector Exponentiation](../vector-mathematics/exponentiation-vector).
