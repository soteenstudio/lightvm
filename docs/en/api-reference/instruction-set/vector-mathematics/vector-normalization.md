# Vector Normalization

The `normalize` instruction scales a floating-point vector to unit length.

| Opcode | Arguments | Operands (stack) | Description |
| --- | --- | --- | --- |
| `normalize` | type | values | Replaces one floating-point array with an equal-length unit vector in the selected type; type must be `hlf`, `flt`, or `dbl`, and a zero vector produces an equal-length zero vector. |
