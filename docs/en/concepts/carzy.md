# Carzy
**Carzy** is a built-in low-level assembly and code generation module that comes with **LightVM** to produce clean, structured, and secure assembly instructions. This module provides a flexible and modular architectural foundation to handle symbol generation, memory sections (*text*, *data*, *rodata*), I/O constant injection, and architecture-specific instruction manipulation without requiring drastic documentation overhauls when new architectures are added.

## How Carzy Works
Carzy employs a two-tier architecture mechanism: the **Core Assembly Engine (`AsmBuilder`)** as a general foundation handling text formatting, string encoding, and buffer writing, alongside an **Architecture-Specific Builder (e.g., `AArch64Builder`)** as a wrapper layer to translate hardware-specific commands. This separation ensures that core assembly-writing logic remains centralized, while expansion to other architectures can be achieved simply by extending wrapper layers without altering the main documentation structure.

  * **Core Assembly Buffer (`AsmBuilder`)**: Optimizes assembly text generation performance using direct `String` memory buffer management and the `writeln!` trait, significantly reducing memory reallocation overhead compared to temporary string formatting methods.
  * **Automatic Symbol Name Sanitization**: Automatically cleans and maps non-alphanumeric characters in label or variable names to underscores (`_`) to prevent syntax errors in low-level assemblers.
  * **Global I/O Constant Injection**: Provides a centralized function to instantly inject standard system constants (such as newline characters, object/array type markers, and ANSI screen-clearing codes).
  * **Architecture Instruction Abstraction (`AArch64Builder`, etc.)**: Wraps hardware-specific instructions (such as `mov`, `add`, `sub`, `ldr`, `str`, and `ret`) into clean helper methods, making assembly code writing resemble safe, high-level function calls.
  * **Modular Memory Section Management**: Declaratively defines memory segment separation through `.text` (instruction code), `.data` (modifiable variables), and `.section .rodata` (read-only/constant data).
  * **Secure String Escaping**: Automatically handles special characters like backslashes, quotes, tabs, and newlines on primitive string data types to ensure safe compilation by external assemblers.

::: info
You can find how to extend and use Carzy on the [Compile Method](../api-reference/method-functions/compile-method) page.
:::
