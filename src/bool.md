# Type level Boolean.

The type level boolean allows you to construct types depending on a condition by using the `<B as Bool>::If<A, B>` assocated type. If `Self` is `True` then `A` is constructed otherwise `B` is constructed. 

The `cond` method allows you to construct the value associated to the `If` type.

The `BOOL` associated constant to get the value associated with the type.

There is also the standard operations: 
- `Not`.
- `And<B: Bool>`.
- `Or<B: Bool>`.
- `Xor<B: Bool>`.


