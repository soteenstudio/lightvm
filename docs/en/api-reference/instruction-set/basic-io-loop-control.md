# Basic I/O & Loop Control
These instructions manage standard input/output operations and flow control within the virtual machine. They handle data output to the console, user input reading, and loop execution management.

| Opcode | Arguments | Operands (stack) | Description |
| :--- | :--- | :--- | :--- |
| `print` | - | val | Prints the top value of the stack to the console without a newline |
| `println` | - | val | Prints the top value of the stack to the console with a newline |
| `stdin` | - | val | Reads a line from standard input, trims the trailing newline characters, and pushes the resulting string onto the stack |
| `stdout` | - | val | Pops the top value from the stack (must be a String) and prints it to the console without a newline |
| `stdoutln` | - | val | Pops the top value from the stack (must be a String) and prints it to the console followed by a newline |
| `clear_screen` | - | - | Clears the terminal screen and resets the cursor position to the top-left corner using ANSI escape codes (\x1B[2J\x1B[1H) |
| `break` | - | target_ip | Stops the loop and jumps to the specified target_ip |
| `nop` | - | - | Empty instructions (usually for placeholders or alignment) |
