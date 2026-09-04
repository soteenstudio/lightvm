# LVM005 (Out of Bounds)
Runtime error type: `OutOfBounds`.

This error occurs when an array or object access uses an index outside the collection. The runtime message reports the attempted `index`, the collection `len`, and the instruction pointer where the access occurred.

Constrain the index to the range `0` through `len - 1`. If `len` is `0`, the collection is empty and no index is valid. Check index calculations for off-by-one errors.
