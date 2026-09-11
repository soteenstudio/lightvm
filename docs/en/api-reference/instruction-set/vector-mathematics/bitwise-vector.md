# Vector Bitwise Operations

## Purpose

Shift or rotate corresponding integer elements in two arrays.

## Supported instructions

`shlv`, `shrv`, `rolv`, and `rorv`.

## Stack operands and result

Each instruction consumes two arrays, using the next-to-top array as values and the top array as shift counts, then leaves one result array.

## Supported numeric types

The type argument accepts `sht`, `int`, `lng`, or `oct`.

## Constraints and failure conditions

Arrays must have equal length and numeric elements. Missing operands produce `StackUnderflow`; invalid arrays, unequal lengths, nonnumeric elements, or unsupported types produce `TypeMismatch`.

## Examples

Applying `shlv int` to `[1, 2]` below `[1, 2]` leaves `[2, 8]`.

## Related instructions

See [Bitwise Operations](../mathematics/bitwise-operations) and [Vector Arithmetic](./vector-arithmetic).
