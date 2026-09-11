# Comparison and Logic

These instructions compare scalar values and evaluate value truthiness.

| Opcode | Arguments | Operands (stack) | Description |
| --- | --- | --- | --- |
| `gt` | type | left, right | Replaces two operands with whether `left > right` after conversion to the selected type. |
| `lt` | type | left, right | Replaces two operands with whether `left < right` after conversion to the selected type. |
| `ge` | type | left, right | Replaces two operands with whether `left >= right` after conversion to the selected type. |
| `le` | type | left, right | Replaces two operands with whether `left <= right` after conversion to the selected type. |
| `eq` | type | left, right | Replaces two operands with whether they are equal after conversion to the selected type. |
| `neq` | type | left, right | Replaces two operands with whether they differ after conversion to the selected type. |
| `and` | - | left, right | Replaces two operands with the Boolean AND of their truthiness. |
| `or` | - | left, right | Replaces two operands with the Boolean OR of their truthiness. |
| `xor` | - | left, right | Replaces two operands with the Boolean exclusive OR of their truthiness. |
| `not` | - | value | Replaces one operand with the Boolean negation of its truthiness. |
