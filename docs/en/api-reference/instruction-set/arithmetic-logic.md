# Arithmetic & Logic
These instructions provide the necessary operations for numerical and logical calculations. Note that for optimization, these instructions require a [Primitive Types](/api-references/primitive-types) (`sht`, `hlf`, `int`, `flt`, `lng`, `dbl`, `oct`) to prevent the VM from guessing the data type during execution.

| Opcode    | Arguments  | Operands (stack) | Description |
|-----------|------------|------------------|-------------|
| `add` / `sub` | type       | val1, val2 | Addition or Subtraction |
| `mul` / `div` | type       | val1, val2 | Multiplication or Division |
| `mod`       | type       | val1, val2 | Modulo (Remainder) |
| `neg` | type | val | Negation (-5 to 5, or vice versa) |
| `inc` / `dec` | name, type | - | Directly add/remove variable contents (without going through the stack) |
| `gt` / `lt`   | type       | val1, val2 |Greater Than or Less Than |
| `ge` / `le`   | type       | val1, val2 | Greater/Less Than or Equal |
| `eq` / `neq`  | type       | val1, val2 | Equal or Not Equal |
| `shl` / `shr` | type       | val1, val2 | Shift Left or Shift Right bitwise operation based on data type |
| `rol` / `ror` | type       | val1, val2 | __Circular__ Shift Left or Right (Rotate) bitwise operation based on data type |
| `and` / `or`  | -          | val1, val2 | Boolean logic operations (``&&`` / ``\|\|``) |
| `xor`       | -          | val1, val2 | Bitwise __Exclusive OR__ operation between two values |
| `not`       | -          | val1, val2 | Bitwise __NOT__ (Inversion) operation on a single value |
| `pow` | type | val1, val2 | General power operation (x^y) |
| `powi` | type | val1, val2 | Power with integer exponent |
| `powf` | type | val1, val2 | Power with floating-point exponent |
| `sin` / `cos` | type | val1, val2 | Sine or Cosine trigonometric operation |
| `tan` | type | val1, val2 | Tangent trigonometric operation |
