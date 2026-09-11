# Vector Arithmetic

## Purpose

Perform element-wise arithmetic on arrays.

## Supported instructions

`addv`, `subv`, `mulv`, `divv`, `modv`, and `negv`.

## Stack operands and result

Binary instructions consume two arrays, using the next-to-top array as the left operand, and replace them with one result array. `negv` replaces the top array.

## Supported numeric types

The type argument accepts `sht`, `int`, `lng`, `oct`, `hlf`, `flt`, or `dbl`.

## Constraints and failure conditions

Binary operands must be arrays of equal length. Missing operands produce `StackUnderflow`; nonnumeric elements, invalid arrays, unequal lengths, or unsupported type arguments produce `TypeMismatch`.

## Examples

Applying `addv int` to `[1, 2]` below `[3, 4]` leaves `[4, 6]`.

## Related instructions

See [Basic Arithmetic](../mathematics/basic-arithmetic) and [Vector Products](./vector-products).
