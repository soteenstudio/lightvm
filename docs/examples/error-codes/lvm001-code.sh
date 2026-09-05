Error[LVM001]: Stack limit reached (limit: 1024).
 │   at instruction pointer: 128
 │   error type: StackOverflow
 │
 └─ hint (explained): The execution encountered a stack overflow, likely triggered by either an infinitely recursive function call that never terminates or an InitStack instruction that failed to reserve enough space for the required stack depth, resulting in the runtime exceeding the allocated memory boundaries for the current call frame.