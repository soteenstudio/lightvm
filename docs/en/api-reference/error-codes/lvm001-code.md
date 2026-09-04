# LVM001 (Stack Overflow)
This error occurs when the VM attempts to push a new value onto the stack after the stack has already reached its maximum permitted capacity. The maximum capacity is determined by the value specified through `InitStack` or, when no custom value is provided, by the VM's default stack limit.

A stack overflow usually indicates that the program is using more stack space than allowed, potentially because of excessive nested function calls, recursive execution, or too many temporary values remaining on the stack.

To resolve this error, reduce unnecessary stack usage or increase the configured stack limit if the program legitimately requires additional stack capacity.

## Example
The following message is displayed when the VM reaches the configured stack limit. The value shown in `limit` represents the maximum number of stack entries permitted by the current configuration.

::: code-group

<<< @/examples/error-codes/lvm001-code.sh{sh:line-numbers}[Example Message]

:::