# Primitive Types
LightVM requires explicit type definitions for certain instructions to maintain deterministic execution and peak performance.

| Type | Aliases | Reference | Target Value Type |
|------|---------|-----------|-------------------|
| sht | i16 | Short | 16-bit Integer (Int16) |
| int | i32 | Integer | 32-bit Integer (Int32) |
| lng | i64 | Long | 64-bit Integer (Int64) |
| oct | i128 | Octa | 128-bit Integer (Int128) |
| hlf | f16 | Half | 16-bit Floating Point (Float16) |
| flt | f32 | Float | 32-bit Floating Point (Float32) |
| dbl | f64 | Double | 64-bit Floating Point (Float64) |
| str | - | String | String / Text data |