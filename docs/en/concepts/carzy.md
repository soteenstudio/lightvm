# Carzy (Assembly Builder)
**Carzy** is a built-in low-level assembly and code generation module that comes with **LightVM**. It provides helpers for assembly text, symbols, memory sections (*text*, *data*, *rodata*), I/O constants, and architecture-specific instructions.

## How Carzy Works
Carzy uses a two-tier architecture: the **Core Assembly Engine (`AsmBuilder`)** manages text formatting and a string buffer, while the **Architecture-Specific Builder (`AArch64Builder`)** wraps it with instruction helpers. These builders emit assembly text without validating instructions or operands.

  * **Core Assembly Buffer (`AsmBuilder`)**: Writes directly to a `String` buffer using the `write!` and `writeln!` macros and the `std::fmt::Write` trait. `build()` returns the text, and `write_to_file()` writes it to a file.
  * **Allocation Name Sanitization**: `AsmBuilder::alloc()` replaces characters that are neither alphanumeric nor underscores with `_` in allocation names. `label()`, `global()`, and `symbol_type()` write names as supplied.
  * **Global I/O Constant Injection**: Emits constants for a newline, the text `16`, object/array markers, and ANSI screen-clearing sequences.
  * **Architecture Instruction Helpers**: `AArch64Builder` provides methods such as `mov`, `add`, `sub`, `ldr`, `str`, and `ret` to format instruction text.
  * **Memory Section Management**: Emits `.text`, `.data`, and `.section .rodata` directives.
  * **String Escaping**: `alloc()` escapes backslashes, quotes, newlines, carriage returns, and tabs for `PrimitiveTypes::Str` values.

::: info
You can find how to use Carzy on the [Compile Method](../api-reference/method-functions/compile-method) page.
:::
