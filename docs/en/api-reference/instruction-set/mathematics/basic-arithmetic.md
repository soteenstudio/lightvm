# Basic Arithmetic

These instructions perform scalar arithmetic and update numeric variables.

| Opcode | Arguments | Operands (stack) | Description |
| --- | --- | --- | --- |
| `add` | type | left, right | Replaces two numeric operands with `left + right` in the selected type. |
| `sub` | type | left, right | Replaces two numeric operands with `left - right` in the selected type. |
| `mul` | type | left, right | Replaces two numeric operands with `left * right` in the selected type. |
| `div` | type | left, right | Replaces two numeric operands with `left / right` in the selected type. |
| `mod` | type | left, right | Replaces two numeric operands with the remainder of `left / right` in the selected type. |
| `neg` | type | value | Replaces one numeric operand with its negation in the selected type. |
| `inc` | name, type | - | Increments the named or indexed numeric variable and pushes the updated value. |
| `dec` | name, type | - | Decrements the named or indexed numeric variable without changing the stack. |
