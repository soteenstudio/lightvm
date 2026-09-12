# Vector Products

These instructions compute dot and cross products from numeric arrays.

| Opcode | Arguments | Operands (stack) | Description |
| --- | --- | --- | --- |
| `dot` | type | left, right | Replaces two equal-length numeric arrays with one scalar: the sum of `left[i] * right[i]` in the selected type. |
| `cross` | type | left, right | Replaces two numeric arrays of exactly three elements each with their three-element cross-product array, preserving operand order. |
