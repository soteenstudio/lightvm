# Torja (Symbol Resolver)
Torja is the core Symbol Resolver of LightVM. It acts as a bridge between high-level bytecode—which uses human-readable names for variables and functions—and the high-performance execution engine that relies on memory-efficient numerical indices.

## How Torja Works
Before your bytecode reaches the execution phase, Torja performs a crucial pass to resolve all symbolic references into fixed-position indices.

  * **Symbol Mapping & Imports**: Torja pre-loads the symbol table with all provided imports. As it traverses the bytecode, it maps every unique variable name found in symbolic instructions to a stable integer index.
  * **Dynamic Resolution**: It utilizes a `get_or_insert_idx` logic; if a variable name is encountered for the first time, Torja dynamically assigns a new index using an incrementing counter (`next_idx`), ensuring a unique ID for every symbol throughout the program lifecycle.
  * **Instruction Specialization**: It transforms generic, name-based instructions into their specialized index-based counterparts. This includes converting `get` to `get_idx`, `set` to `set_idx`, `inc` to `inc_idx`, and `dec` to `dec_idx`. This minimizes runtime lookups and significantly reduces CPU overhead during execution.
  * **Functional Scope Tracking**: Torja identifies function parameter names within `Func` instructions. It registers these names into the symbol table, ensuring all local-scoped identifiers are correctly tracked and prepared for the VM's stack-based architecture.
