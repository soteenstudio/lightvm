# Dying
**Dying** is a built-in termination and panic handling system that comes with **LightVM** to manage fatal errors, system crashes, and critical runtime exceptions with clean, color-coded stack traces and diagnostics to ensure maximum debugging efficiency, minimal overhead, and reliable error reporting. Each error report is formatted with precise ANSI color codes to highlight critical failure points and prevent unhandled exception ambiguity.

## How Dying Works
Dying employs a specialized error-formatting pipeline and a **Color-Coded Diagnostics** mechanism to report runtime faults safely. It processes the diagnostic stream through multiple visual passes dynamically sorted by severity weights (`color_weights`), systematically identifying, highlighting effective failure contexts, and formatting diagnostic messages before the VM runtime shuts down.

  * **Specialized Color Constants**: Dying optimizes terminal readability and log distinction by defining distinct ANSI color constants (e.g., `RED`, `CYAN`, `GREEN`, `YELLOW`, `BLUE`, `MAGENTA`, `BRIGHT_GREEN`, `DARK_GRAY`). This allows the VM to output diagnostics with predefined visual hierarchies, significantly reducing the overhead of manual log parsing and ANSI code duplication.
 * **Critical Error Highlighting**: Pre-formats fatal crash messages using bold red indicators (`RED` and `BOLD`) if the failure state is recognized at runtime.
 * **Metadata & Status Coloring**: Pre-evaluates status indicators and metadata labels using cyan and green styling (`CYAN`, `GREEN`, `BRIGHT_GREEN`) to distinguish between normal execution boundaries and error states.
 * **Warning & Notice Styling**: Replaces generic warning logs with high-visibility yellow and magenta tones (`YELLOW`, `MAGENTA`) to draw immediate attention to non-fatal runtime anomalies.
 * **Trace Dimming**: Analyzes stack trace depth and automatically applies muted styling (`DARK_GRAY`) to secondary call frames that don't contribute directly to the primary failure cause.
 * **Scope Reset Enforcement**: Identifies and appends the standard reset sequence (`RESET`) to every formatted log block, preventing color bleed into the outer host terminal session.
