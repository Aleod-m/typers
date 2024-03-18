# Typers

This crate provides some primitives to perform type level programming.
At the moment it only provides booleans, usigned numbers and list at the type level.

## Quick intro to type level programming.

How to construct a type from another?

To construct a type from another we use GATs (Generic Associated Types). An associated type can be seen as a function taking the type `Self` as its first argument and returns a type. If the associated type has generics they can be considered as additional arguments. To perform operation with those types you might add trait bounds get access to other type functions. Trait bounds are the types of types. Sometimes you may need to use the `<T as Trait>` syntax to use the assciated types of the `Trait`

The principal way to compute complex types is to use recursion. The `list` and `num` modules make use of it extensively.
