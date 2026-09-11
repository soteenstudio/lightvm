# Trigonometry

## Purpose

Evaluate scalar trigonometric, inverse, and hyperbolic functions.

## Supported instructions

`sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, and `atanh`.

## Stack operands and result

Unary instructions replace the top value. `atan2` consumes the next-to-top value as its first operand and the top value as its second operand, then leaves one result.

## Supported numeric types

The type argument accepts `hlf`, `flt`, or `dbl`.

## Constraints and failure conditions

Missing operands produce `StackUnderflow`. Nonnumeric values or unsupported type arguments produce `TypeMismatch`. Domain errors follow floating-point behavior.

## Examples

With `0` on top, `sin dbl` leaves `0`.

## Related instructions

See [Roots](./root) and [Vector Trigonometry](../vector-mathematics/vector-trigonometry).
