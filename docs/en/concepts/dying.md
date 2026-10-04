# Dying
**Dying** is a built-in module that provides shared ANSI color and style constants for **LightVM** diagnostics.

## How Dying Works
Dying defines reusable string constants. [VMError](./vmerror) uses these constants to format diagnostics and internal backtraces. Callers determine how errors are handled and whether the process terminates.

  * **Shared Color Constants**: Provides `RED`, `CYAN`, `GREEN`, `YELLOW`, `BLUE`, `MAGENTA`, `BRIGHT_GREEN`, and `DARK_GRAY` for callers to use in terminal output.
  * **Text Styling**: Provides `BOLD` for bold text and `RESET` to reset styling. Callers insert these sequences where needed.
