# Stack & Variable Management
The following table details the core operations for stack manipulation and variable lifecycle management. These instructions are fundamental to LightVM's stack-based architecture, governing how data is stored, retrieved, and organized during execution.

| Opcode | Arguments | Operands (stack) | Description |
|--------|-----------|------------------|-------------|
| `push`   | value     | - |Inserting data into the stack |
| `val`    | name      | - | Declaring a new variable |
| `set`    | name      | val | Take the top stack and then save it to the variable ``name`` |
| `get`    | name      | - | Take the contents of the ``name`` variable and push it onto the stack |
| `dup`    | -         | val | Duplicate the top value in the stack |
| `swap` | - | val1, val2 | Swap the top stack with the bottom stack |