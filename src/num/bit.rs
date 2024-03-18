use crate::{
    bool::{Bool, False, True},
    seal, Invalid,
};

/// Represent a bit set to `0`.
#[derive(Default)]
pub struct B0;
/// Represent a bit set to `1`.
#[derive(Default)]
pub struct B1;

impl seal::Sealed for B0 {}
impl seal::Sealed for B1 {}

/// A trait representing all valid operation for a  bit.
pub trait Bit: seal::Sealed {
    /// Bit value.
    const USIZE: usize;
    #[doc(hidden)]
    /// If bit is `B0` this is `True`.
    type IsZero: Bool;
    #[doc(hidden)]
    /// Bit Negation.
    type Not: Bit;
    #[doc(hidden)]
    /// Bit and.
    type And<Rhs: Bit>: Bit;
    #[doc(hidden)]
    /// Bit or.
    type Or<Rhs: Bit>: Bit;
    #[doc(hidden)]
    /// Bit xor.
    type Xor<Rhs: Bit>: Bit;

    #[doc(hidden)]
    /// Bit Addition.
    type Add<Rhs: Bit>: Bit;
    #[doc(hidden)]
    /// Bit Carry of addition.
    type Carry<Rhs: Bit>: Bit;
    #[doc(hidden)]
    /// Bit Addition with carry in input.
    type FullAdd<Rhs: Bit, C: Bit>: Bit;
    #[doc(hidden)]
    /// Bit Addition carry with carry in input.
    type FullCarry<Rhs: Bit, C: Bit>: Bit;

    #[doc(hidden)]
    /// Substraction.
    type Diff<Rhs: Bit>: Bit;
    #[doc(hidden)]
    /// Borrow of Substraction. (Carry analog for Substraction)
    type Borrow<Rhs: Bit>: Bit;
    #[doc(hidden)]
    /// Substraction with borrow in input.
    type FullDiff<Rhs: Bit, B: Bit>: Bit;
    #[doc(hidden)]
    /// Borrow of Substraction with borrow in input.
    type FullBorrow<Rhs: Bit, B: Bit>: Bit;
}

/// Short Hand type function.
pub type IsZero<B> = <B as Bit>::IsZero;
/// Short Hand type function.
pub type Not<B> = <B as Bit>::Not;
/// Short Hand type function.
pub type And<Lhs, Rhs> = <Lhs as Bit>::And<Rhs>;
/// Short Hand type function.
pub type Or<Lhs, Rhs> = <Lhs as Bit>::Or<Rhs>;
/// Short Hand type function.
pub type Xor<Lhs, Rhs> = <Lhs as Bit>::Xor<Rhs>;

/// Short Hand type function.
pub type Add<Lhs, Rhs> = <Lhs as Bit>::Add<Rhs>;
/// Short Hand type function.
pub type Carry<Lhs, Rhs> = <Lhs as Bit>::Carry<Rhs>;
/// Short Hand type function.
pub type FullAdd<Lhs, Rhs, C> = <Lhs as Bit>::FullAdd<Rhs, C>;
/// Short Hand type function.
pub type FullCarry<Lhs, Rhs, C> = <Lhs as Bit>::FullCarry<Rhs, C>;

/// Short Hand type function.
pub type Diff<Lhs, Rhs> = <Lhs as Bit>::Diff<Rhs>;
/// Short Hand type function.
pub type Borrow<Lhs, Rhs> = <Lhs as Bit>::Borrow<Rhs>;
/// Short Hand type function.
pub type FullDiff<Lhs, Rhs, C> = <Lhs as Bit>::FullDiff<Rhs, C>;
/// Short Hand type function.
pub type FullBorrow<Lhs, Rhs, C> = <Lhs as Bit>::FullBorrow<Rhs, C>;

impl Bit for Invalid {
    #[doc(hidden)]
    const USIZE: usize = { panic!("Invlid Bit type value!") };

    type IsZero = Invalid;

    type Not = Invalid;

    type And<Rhs: Bit> = Invalid;

    type Or<Rhs: Bit> = Invalid;

    type Xor<Rhs: Bit> = Invalid;

    type Add<Rhs: Bit> = Invalid;

    type Carry<Rhs: Bit> = Invalid;

    type FullAdd<Rhs: Bit, C: Bit> = Invalid;

    type FullCarry<Rhs: Bit, C: Bit> = Invalid;

    type Diff<Rhs: Bit> = Invalid;

    type Borrow<Rhs: Bit> = Invalid;

    type FullDiff<Rhs: Bit, B: Bit> = Invalid;

    type FullBorrow<Rhs: Bit, B: Bit> = Invalid;
}

impl Bit for B0 {
    const USIZE: usize = 0;
    type IsZero = True;
    type Not = B1;
    type And<Rhs: Bit> = B0;
    type Or<Rhs: Bit> = Rhs;
    type Xor<Rhs: Bit> = Rhs;

    type Add<Rhs: Bit> = Rhs;
    type Carry<Rhs: Bit> = B0;
    type FullAdd<Rhs: Bit, C: Bit> = Rhs::Add<C>;
    type FullCarry<Rhs: Bit, C: Bit> = Rhs::Carry<C>;

    type Diff<Rhs: Bit> = Rhs;
    type Borrow<Rhs: Bit> = Rhs;
    type FullDiff<Rhs: Bit, B: Bit> = Rhs::Xor<B>;
    type FullBorrow<Rhs: Bit, B: Bit> = Rhs::Or<B>;
}

impl Bit for B1 {
    const USIZE: usize = 1;
    type IsZero = False;
    type Not = B0;
    type And<Rhs: Bit> = B0;
    type Or<Rhs: Bit> = Rhs;
    type Xor<Rhs: Bit> = Rhs::Not;

    type Add<Rhs: Bit> = Self::Xor<Rhs>;
    type Carry<Rhs: Bit> = Self::And<Rhs>;
    type FullAdd<Rhs: Bit, C: Bit> = <Self::Add<Rhs> as Bit>::Add<C>;

    type FullCarry<Rhs: Bit, C: Bit> =
        <<Self::Add<Rhs> as Bit>::Carry<C> as Bit>::Xor<Self::Carry<Rhs>>;

    type Diff<Rhs: Bit> = Rhs::Not;
    type Borrow<Rhs: Bit> = B0;
    type FullDiff<Rhs: Bit, B: Bit> = <Self::Diff<Rhs> as Bit>::Diff<B>;
    type FullBorrow<Rhs: Bit, B: Bit> =
        <Self::Borrow<Rhs> as Bit>::Xor<<Self::Diff<Rhs> as Bit>::Borrow<B>>;
}
