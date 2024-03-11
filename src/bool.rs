use crate::{
    num::{IBit, Unsigned},
    seal, Invalid,
};

pub trait Bool: seal::Sealed {
    const BOOL: bool;
    type Not: Bool;
    type And<B: Bool>: Bool;
    type Or<B: Bool>: Bool;
    type Xor<B: Bool>: Bool;

    type If<A, B>;
    type Ifbool<A: Bool, B: Bool>: Bool;
    type Ifbit<A: IBit, B: IBit>: IBit;
    type Ifuint<A: Unsigned, B: Unsigned>: Unsigned;

    /// Branches need to be closures because they need to be lazily evaluated.
    fn cond<A, B>(a: impl Fn() -> A, b: impl Fn() -> B) -> Self::If<A, B>;
}

impl Bool for Invalid {
    const BOOL: bool = { panic!("Invalid Bool type value!") };

    type Not = Invalid;
    type And<B: Bool> = Invalid;
    type Or<B: Bool> = Invalid;
    type Xor<B: Bool> = Invalid;

    type If<A, B> = Invalid;
    type Ifbool<A: Bool, B: Bool> = Invalid;
    type Ifbit<A: IBit, B: IBit> = Invalid;
    type Ifuint<A: Unsigned, B: Unsigned> = Invalid;

    fn cond<A, B>(_: impl Fn() -> A, _: impl Fn() -> B) -> Self::If<A, B> {
        panic!("Attempted to resolve an Invalid condition!")
    }
}

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
    type Ifbit<A: IBit, B: IBit> = A;
    type Ifuint<A: Unsigned, B: Unsigned> = A;

    fn cond<A, B>(a: impl Fn() -> A, _: impl Fn() -> B) -> Self::If<A, B> {
        a()
    }
}

pub struct False;
impl seal::Sealed for False {}
impl Bool for False {
    const BOOL: bool = false;
    type Not = True;
    type And<B: Bool> = False;
    type Or<B: Bool> = B;
    type Xor<B: Bool> = B;

    type If<A, B> = B;
    type Ifbit<A: IBit, B: IBit> = B;
    type Ifuint<A: Unsigned, B: Unsigned> = B;
    type Ifbool<A: Bool, B: Bool> = B;

    fn cond<A, B>(_: impl Fn() -> A, b: impl Fn() -> B) -> Self::If<A, B> {
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
