# Data Structures & Metadata
These instructions handle dynamic memory allocation for complex types. Objects and arrays in LightVM are allocated on the heap; when using these opcodes, ensure that your program manages references correctly to avoid memory overhead. Metadata operations (`length`, `typeof`) allow for runtime type introspection and data validation.

| Opcode     | Arguments | Operands (stack) | Description |
|------------|-----------|------------------|-------------|
| `make_obj`   | count     | k1, v1, ... kn, vn | Create Object from n key-value pairs in stack |
| `make_array` | count     | v1, v2, ... vn | Create an Array of n elements in a stack |
| `access`     | prop_name | target_obj | Access Object's properties |
| `access_index` | -       | target_arr, index | Access Array elements by index on the stack |
| `length`     | -         | val | Check the length of a string or the number of items in an array/object |
| `typeof`     | -         | val | Get the data type from the top value of the stack |
| `concat`     | -         | val1, val2 | Combine two values (usually strings) |