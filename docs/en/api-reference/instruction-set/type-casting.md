# Type Casting (Conversion)
These instructions facilitate explicit data type conversion within the LightVM environment. These opcodes are essential for managing variable precision and ensuring that data formats align with expected operational requirements during runtime execution.

| Opcode | Operands (stack) | Description |
| :--- | :--- | :--- |
| `to_short` | val | Change value to Short (16-bit) |
| `to_integer` | val | Change value to Integer (32-bit) |
| `to_long` | val | Change the value to Long (64-bit) |
| `to_octa` | val | Change value to Octa (128-bit Integer) |
| `to_half` | val | Change value to Half-precision (16-bit Float) |
| `to_float` | val | Change value to Float (32-bit) |
| `to_double` | val | Change the value to Double (64-bit) |
| `to_string` | val | Change the value to String |