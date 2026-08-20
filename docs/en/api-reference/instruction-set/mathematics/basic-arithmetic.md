# Basic Arithmetic
These instructions handle standard mathematical calculations, basic scaling, and direct variable modifications.

| Opcode | Arguments | Operands (stack) | Description |
| :--- | :--- | :--- | :--- |
| `add` / `sub` | type | val1, val2 | Addition or Subtraction |
| `mul` / `div` | type | val1, val2 | Multiplication or Division |
| `mod` | type | val1, val2 | Modulo (Remainder) |
| `neg` | type | val | Negation (changes sign: 5 to -5, or -5 to 5) |
| `inc` / `dec` | name, type | - | Directly add/remove variable contents |
| `pow` | type | val1, val2 | General power operation (x^y) |
| `powi` | type | val1, val2 | Power with integer exponent |
| `powf` | type | val1, val2 | Power with floating-point exponent |