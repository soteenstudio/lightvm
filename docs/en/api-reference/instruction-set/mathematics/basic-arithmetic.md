# Basic Arithmetic

## Purpose

Perform scalar arithmetic and update numeric variables.

## Supported instructions

`add`, `sub`, `mul`, `div`, `mod`, `neg`, `inc`, and `dec`.

## Stack operands and result

Binary instructions consume the next-to-top value as the left operand and the top value as the right operand, then replace them with one result. `neg` replaces the top value. `inc` and `dec` update a named or indexed variable; `inc` also pushes the updated value.

## Supported numeric types

The type argument accepts `sht`, `int`, `lng`, `oct`, `hlf`, `flt`, or `dbl`.

## Constraints and failure conditions

Missing operands produce `StackUnderflow`. Values or a type argument incompatible with the selected operation produce `TypeMismatch`.

## Examples

With `2` below `3`, `add int` leaves `5`. With `7` below `2`, `sub int` leaves `5`.

## Related instructions

See [Exponentiation](./exponentiation) and [Vector Arithmetic](../vector-mathematics/vector-arithmetic).
