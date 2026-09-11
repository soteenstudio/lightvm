# Bitwise Operations

## Purpose

Shift or rotate scalar integer bits.

## Supported instructions

`shl`, `shr`, `rol`, and `ror`.

## Stack operands and result

Each instruction consumes the next-to-top value as the value to transform and the top value as the shift count, then replaces both with one result.

## Supported numeric types

The type argument accepts `sht`, `int`, `lng`, or `oct`.

## Constraints and failure conditions

Fewer than two operands produce `StackUnderflow`. Non-integer values and unsupported type arguments produce `TypeMismatch`.

## Examples

With `1` below `3`, `shl int` leaves `8`.

## Related instructions

See [Comparison and Logic](./comparison-logic) and [Vector Bitwise Operations](../vector-mathematics/bitwise-vector).
