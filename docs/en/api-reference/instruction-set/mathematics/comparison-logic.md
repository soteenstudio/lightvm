# Comparison and Logic

## Purpose

Compare scalar values and evaluate truthiness.

## Supported instructions

`gt`, `lt`, `ge`, `le`, `eq`, `neq`, `and`, `or`, `xor`, and `not`.

## Stack operands and result

Binary instructions consume the next-to-top value as the left operand and the top value as the right operand, then push one boolean. `not` replaces the top value with its boolean negation.

## Supported numeric types

Comparisons use a type argument. Ordered comparisons support numeric types; `eq` and `neq` also support `str`. Logic instructions have no type argument and use value truthiness.

## Constraints and failure conditions

Missing operands produce `StackUnderflow`. These implementations coerce values through the selected comparison type and do not report `TypeMismatch`.

## Examples

With `2` below `3`, `lt int` leaves `true`. Applying `not` to `true` leaves `false`.

## Related instructions

See [Basic Arithmetic](./basic-arithmetic) and [Bitwise Operations](./bitwise-operations).
