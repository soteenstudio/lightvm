# Vector Exponentiation

## Purpose

Evaluate powers and natural exponentials element by element.

## Supported instructions

`powv`, `powiv`, `powfv`, and `expv`.

## Stack operands and result

Power instructions consume equally sized base and exponent arrays and leave one result array. `expv` replaces the top array.

## Supported numeric types

`powv`, `powiv`, and `powfv` accept implemented numeric type combinations. `expv` accepts `hlf`, `flt`, or `dbl`.

## Constraints and failure conditions

Power arrays must have equal length. Missing operands produce `StackUnderflow`; invalid arrays, nonnumeric elements, unequal lengths, or unsupported types produce `TypeMismatch`.

## Examples

Applying `powv int` to `[2, 3]` below `[3, 2]` leaves `[8, 9]`.

## Related instructions

See [Exponentiation](../mathematics/exponentiation) and [Vector Logarithms](./logarithm-vector).
