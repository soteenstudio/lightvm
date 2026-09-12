# Vector Bitwise Operations

These instructions shift or rotate corresponding integer elements in two arrays.

| Opcode | Arguments | Operands (stack) | Description |
| --- | --- | --- | --- |
| `shlv` | type | values, counts | Replaces two equal-length arrays with element-wise left shifts; the top array supplies counts, and type must be `sht`, `int`, `lng`, or `oct`. |
| `shrv` | type | values, counts | Replaces two equal-length arrays with element-wise right shifts; the top array supplies counts, and type must be `sht`, `int`, `lng`, or `oct`. |
| `rolv` | type | values, counts | Replaces two equal-length arrays with element-wise left rotations; the top array supplies counts, and type must be `sht`, `int`, `lng`, or `oct`. |
| `rorv` | type | values, counts | Replaces two equal-length arrays with element-wise right rotations; the top array supplies counts, and type must be `sht`, `int`, `lng`, or `oct`. |
