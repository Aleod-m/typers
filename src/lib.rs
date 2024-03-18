#![doc = include_str!("../README.md")]
pub mod bool;
pub mod list;
pub mod num;

mod seal {
    pub trait Sealed {}
    // A struct to disallow the calling of a function.
    pub struct Key {}
    impl Sealed for Key {}
}
/// Invalid represent the result of any operation that can't be performed on a type.(e.g.
/// Decrementing the Representation of 0 in usigned numbers)
pub struct Invalid;
impl seal::Sealed for Invalid {}
