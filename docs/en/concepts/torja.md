# Torja
Torja is the core Symbol Resolver of LightVM. It acts as a bridge between high-level bytecode—which uses human-readable names for variables and functions—and the high-performance execution engine that relies on memory-efficient numerical indices.

## How Torja Works
Before your bytecode reaches the execution phase, Torja performs a crucial pass to resolve all symbolic references into fixed-position indices.

 * __Symbol Mapping__: Scans through all imports and dynamic instructions to map every unique variable or function name to a stable integer index.
 * __Dynamic Resolution__: If a variable or function is encountered for the first time during the resolution pass, Torja dynamically assigns a new index, ensuring a unique ID for every symbol throughout the lifecycle of the program.
 * __Instruction Specialization__: It transforms generic, name-based instructions (e.g., `get`, `set`, `inc`) into their specialized index-based counterparts (`get_idx`, `set_idx`, `inc_idx`). This minimizes runtime lookups and significantly reduces CPU overhead during execution.
 * __Value Promotion__: During the symbol pass, Torja also optimizes Push instructions by promoting generic Value types into specialized, type-specific opcodes (e.g., `push_int16`, `push_float64`, `push_string`), ensuring the VM knows the exact data size and type immediately.
 * __Functional Scope Tracking__: Torja identifies function parameters and scoped identifiers, ensuring that all local symbols are correctly tracked within the symbol table and prepared for the VM's stack-based architecture.


