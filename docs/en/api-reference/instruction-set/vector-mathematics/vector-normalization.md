# Vector Normalization

The `normalize` instruction scales a floating-point vector to unit length.

| Opcode | Arguments | Operands (stack) | Description |
| --- | --- | --- | --- |
| `normalize` | type | values | Replaces one floating-point array with an equal-length array of the selected type. Type must be `hlf`, `flt`, or `dbl`. |

The instruction consumes the array at the top of the stack and pushes its normalized vector. A non-zero vector produces a unit-length vector. A zero vector produces an equal-length zero vector instead of dividing by zero.

The operand must be a numeric array, and the type argument must be `hlf`, `flt`, or `dbl`. Other operand or type values produce a [`TypeMismatch`](/api-reference/error-codes/lvm004-code) error. A missing operand produces a [`StackUnderflow`](/api-reference/error-codes/lvm002-code) error. The stack remains unchanged when the operand is invalid.

## Examples

Each example normalizes `[3, 4]` to approximately `[0.6, 0.8]`, with elements stored in the selected numeric type.

```json
[["push", [3.0, 4.0]], ["normalize", "hlf"]]
```

```json
[["push", [3.0, 4.0]], ["normalize", "flt"]]
```

```json
[["push", [3.0, 4.0]], ["normalize", "dbl"]]
```
