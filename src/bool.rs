#![doc = include_str!("./bool.md")]
use crate::{
    list::TList,
    num::{bit::Bit, unsigned::Unsigned},
    seal, Invalid,
};

/// A trait implemented for all boolean types values.
pub trait Bool: seal::Sealed {
    /// Value equivalent to the type.
    const BOOL: bool;
    #[doc(hidden)]
    type Not: Bool;
    #[doc(hidden)]
    type And<B: Bool>: Bool;
    #[doc(hidden)]
    type Or<B: Bool>: Bool;
    #[doc(hidden)]
    type Xor<B: Bool>: Bool;

    #[doc(hidden)]
    type If<A, B>;
    #[doc(hidden)]
    type Ifbool<A: Bool, B: Bool>: Bool;
    #[doc(hidden)]
    type Ifbit<A: Bit, B: Bit>: Bit;
    #[doc(hidden)]
    type Ifuint<A: Unsigned, B: Unsigned>: Unsigned;
    #[doc(hidden)]
    type Iflist<A: TList, B: TList>: TList;

    /// Branches need to be closures because they need to be lazily evaluated.
    fn cond<A, B>(a: impl FnOnce() -> A, b: impl FnOnce() -> B) -> Self::If<A, B>;
}

/// Result of the boolean negation.
pub type Not<B> = <B as Bool>::Not;
/// Result of the boolean and between `Self` and `B`.
pub type And<A, B> = <A as Bool>::And<B>;
/// Result of the boolean or between `Self` and `B`.
pub type Or<A, B> = <A as Bool>::Or<B>;
/// Result of the boolean xor between `Self` and `B`.
pub type Xor<A, B> = <A as Bool>::Xor<B>;
/// If `Self` is [True] construct type `A` otherwise `B`.
pub type If<C, A, B> = <C as Bool>::If<A, B>;

impl Bool for Invalid {
    #[doc(hidden)]
    const BOOL: bool = { panic!("Invalid Bool type value!") };

    type Not = Invalid;
    type And<B: Bool> = Invalid;
    type Or<B: Bool> = Invalid;
    type Xor<B: Bool> = Invalid;

    type If<A, B> = Invalid;
    type Ifbool<A: Bool, B: Bool> = Invalid;
    type Ifbit<A: Bit, B: Bit> = Invalid;
    type Ifuint<A: Unsigned, B: Unsigned> = Invalid;
    type Iflist<A: TList, B: TList> = Invalid;

    fn cond<A, B>(_: impl FnOnce() -> A, _: impl FnOnce() -> B) -> Self::If<A, B> {
        panic!("Attempted to resolve an Invalid condition!")
    }
}

/// The boolean value `true` in type land.
pub struct True;
impl seal::Sealed for True {}
impl Bool for True {
    const BOOL: bool = true;
    type Not = False;
    type And<B: Bool> = B;
    type Or<B: Bool> = Self;
    type Xor<B: Bool> = B::Not;

    type If<A, B> = A;
    type Ifbool<A: Bool, B: Bool> = A;
    type Ifbit<A: Bit, B: Bit> = A;
    type Ifuint<A: Unsigned, B: Unsigned> = A;
    type Iflist<A: TList, B: TList> = A;

    fn cond<A, B>(a: impl FnOnce() -> A, _: impl FnOnce() -> B) -> Self::If<A, B> {
        a()
    }
}

/// The boolean value `false` in type land.
pub struct False;
impl seal::Sealed for False {}
impl Bool for False {
    const BOOL: bool = false;
    type Not = True;
    type And<B: Bool> = False;
    type Or<B: Bool> = B;
    type Xor<B: Bool> = B;

    type If<A, B> = B;
    type Ifbit<A: Bit, B: Bit> = B;
    type Ifuint<A: Unsigned, B: Unsigned> = B;
    type Ifbool<A: Bool, B: Bool> = B;
    type Iflist<A: TList, B: TList> = B;

    fn cond<A, B>(_: impl FnOnce() -> A, b: impl FnOnce() -> B) -> Self::If<A, B> {
        b()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert!(<<True as Bool>::If<True, False> as Bool>::BOOL);
        assert!(<<False as Bool>::If<False, True> as Bool>::BOOL);
    }

    #[test]
    fn test_cond() {
        assert!(True::cond(|| true, || false));
        assert!(False::cond(|| false, || true));
    }
}
