# Bitwise Operations

These instructions shift or rotate scalar integer values.

| Opcode | Arguments | Operands (stack) | Description |
| --- | --- | --- | --- |
| `shl` | type | value, count | Replaces the operands with `value` shifted left by `count`; type must be `sht`, `int`, `lng`, or `oct`. |
| `shr` | type | value, count | Replaces the operands with `value` shifted right by `count`; type must be `sht`, `int`, `lng`, or `oct`. |
| `rol` | type | value, count | Replaces the operands with `value` rotated left by `count`; type must be `sht`, `int`, `lng`, or `oct`. |
| `ror` | type | value, count | Replaces the operands with `value` rotated right by `count`; type must be `sht`, `int`, `lng`, or `oct`. |
