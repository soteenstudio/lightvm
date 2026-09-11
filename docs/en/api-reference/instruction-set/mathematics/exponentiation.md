# Exponentiation

These instructions evaluate scalar powers and the natural exponential function.

| Opcode | Arguments | Operands (stack) | Description |
| --- | --- | --- | --- |
| `pow` | type | base, exponent | Replaces the operands with an integer power; type must be `sht`, `int`, `lng`, or `oct`, and the top value is the exponent. |
| `powi` | type | base, exponent | Replaces the operands with a floating-point base raised to an integer exponent; type must be `hlf`, `flt`, or `dbl`, and the top value is the exponent. |
| `powf` | type | base, exponent | Replaces the operands with a floating-point power; type must be `hlf`, `flt`, or `dbl`, and the top value is the exponent. |
| `exp` | type | value | Replaces one operand with e raised to `value`; type must be `hlf`, `flt`, or `dbl`. |
