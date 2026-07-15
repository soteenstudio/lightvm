# Objects & OOP

| Opcode | Arguments | Operands (stack) | Description |
|--------|-----------|------------------|-------------|
| `set_prop` | prop_name | val, obj | Set the value of an object property (retrieve ``value`` and ``target_obj`` from the stack) |
| `instantiate` | class_name, argc | arg1, ... argN | Creates a new instance of a class with a specified number of constructor arguments |
| `inspect_obj` | - | obj | Prints the internal structure of an Object to the console |
| `inspect_array` | - | arr | Print the internal contents of an Array to the console |

::: warning
__Nightly Opcode__: The `instantiate` instruction is still experimental. The API may change without notice in the `@next` or `@nightly` version.
:::
