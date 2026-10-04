# Torja (Symbol Resolver)
**Torja** is the Symbol Resolver of **LightVM**. It maps variable names to numerical indices used by the execution engine.

## How Torja Works
Each `resolve_symbols()` call creates one symbol table, loads import names, and rewrites supported symbolic instructions. Indices belong to that resolution call; they are not guaranteed to remain the same across calls.

  * **Symbol Mapping & Imports**: Preloads import names into the symbol table and assigns indices to additional names encountered in bytecode.
  * **Dynamic Resolution**: `get_or_insert_idx` reuses an existing index or assigns a new one using `next_idx`.
  * **Index-Based Instructions**: Converts `val`, `get`, `set`, `inc`, and `dec` into their index-based counterparts. Type-specific `push` specialization belongs to Gazle.
  * **Function Parameter Names**: Registers parameter names from `Func` instructions in the same symbol table as other names. It does not create separate lexical scope tables; identical names share an index within the call.
