use std::marker;
use crate::{
    num::bit::{Bit, B0, B1, self},
    bool::{Bool, self},
    seal, Invalid,
};

pub type If<C, A, B> = <C as Bool>::Ifuint<A, B>;

pub trait Unsigned: seal::Sealed {
    /// The value equivalent of `Self`
    const USIZE: usize;

    #[doc(hidden)]
    type Msb: Unsigned;
    #[doc(hidden)]
    type Lsb: Bit;

    #[doc(hidden)]
    type Bsr: Unsigned;
    #[doc(hidden)]
    type Bsl: Unsigned;

    #[doc(hidden)]
    type Inc: Unsigned;
    #[doc(hidden)]
    type Dec: Unsigned;
    #[doc(hidden)]
    type Add<Rhs: Unsigned>: Unsigned;
    #[doc(hidden)]
    type AddWithCarry<Rhs: Unsigned, C: Bit>: Unsigned;
    #[doc(hidden)]
    type Sub<Rhs: Unsigned>: Unsigned;
    #[doc(hidden)]
    type SubWithBorrow<Rhs: Unsigned, B: Bit>: Unsigned;
    #[doc(hidden)]
    type IsZero: Bool;
    #[doc(hidden)]
    type RmExtraBits: Unsigned;
    #[doc(hidden)]
    type Mul<Rhs: Unsigned>: Unsigned;
}


/// The most significant bits of `Self`.
pub type Msb<U> = <U as Unsigned>::Msb;
/// The least significant bit of `Self`.
pub type Lsb<U> = <U as Unsigned>::Lsb;
/// The result of bit shifting right on `Self`.
pub type Bsr<U> = <U as Unsigned>::Bsr;
/// The result of bit shifting left on `Self`.
pub type Bsl<U> = <U as Unsigned>::Bsl;
/// The result of incrementing `Self`.
pub type Inc<U> = <U as Unsigned>::Inc;
/// The result of decrementing `Self`.
pub type Dec<U> = <U as Unsigned>::Dec;
/// Result of adding `Self` with `Rhs`.
pub type Add<Lhs, Rhs> = <Lhs as Unsigned>::Add<Rhs>;
/// Result of adding `Self` with `Rhs` and a carry bit.
pub type AddWithCarry<Lhs, Rhs, C> = <Lhs as Unsigned>::AddWithCarry<Rhs, C>;
/// Result of substracting `Self` with `Rhs`.
pub type Sub<Lhs, Rhs> = <Lhs as Unsigned>::Sub<Rhs>;
/// Result of substracting`Self` with `Rhs` and a borrow bit.
pub type SubWithBorrow<Lhs, Rhs, B> = <Lhs as Unsigned>::SubWithBorrow<Rhs, B>;
/// Check if the Unsigned is zero.
pub type IsZero<U> = <U as Unsigned>::IsZero;
/// Remove the extra zero bits that can occur durring computation.
pub type RmExtraBits<U> = <U as Unsigned>::RmExtraBits;
/// Result of multiplying `Self` with `Rhs`.
pub type Mul<Lhs, Rhs> = <Lhs as Unsigned>::Mul<Rhs>;

/// `Uint` is represented as a list of bits.
#[derive(Default)]
pub struct UInt<Msbs: Unsigned, Lsb: Bit>(marker::PhantomData<(Msbs, Lsb)>);
impl<Msbs: Unsigned, Lsb: Bit> seal::Sealed for UInt<Msbs, Lsb> {}

/// `Msb` is the most significant bit of the Uint.
#[derive(Default)]
pub struct Last<B: Bit>(marker::PhantomData<B>);
impl<B: Bit> seal::Sealed for Last<B> {}

/// A macro to define simle Unsigned types with just the bits.
/// ```ignore
/// type U10 = uint!(B1, B0, B1, B0);
/// ```
#[macro_export]
macro_rules! uint {
    ($ty:ident $(, $rest:ident)*) => {
        uint!(@parse input: [$($rest),*], out: [Last<$ty>])
    };

    (@parse input: [$ty:ident $(, $rest:ident)*], out: [$($out:tt)*]) => {
        uint!(@parse input: [$($rest),*], out: [UInt<$($out)*, $ty>])
    };
    (@parse input: [], out: [$($out:tt)*]) => {
        $($out)*
    };
}

impl Unsigned for Invalid {
    #[doc(hidden)]
    const USIZE: usize = { panic!("Invlid Unsigned Value!") };
    type Msb = Invalid;
    type Lsb = Invalid;

    type Bsr = Invalid;
    type Bsl = Invalid;

    type Inc = Invalid;
    type Dec = Invalid;

    type Add<Rhs: Unsigned> = Invalid;
    type AddWithCarry<Rhs: Unsigned, C: Bit> = Invalid;

    type Sub<Rhs: Unsigned> = Invalid;
    type SubWithBorrow<Rhs: Unsigned, B: Bit> = Invalid;

    type IsZero = Invalid;
    type RmExtraBits = Invalid;
    type Mul<Rhs: Unsigned> = Invalid;
}

impl<Lsb_: Bit> Unsigned for Last<Lsb_> {
    const USIZE: usize = Lsb_::USIZE;
    type Lsb = Lsb_;
    type Msb = Invalid; // Shouldn't be accessed.

    type Inc = If<bit::IsZero<Lsb_>, /*Then*/ Last<B1>, /*Else*/ UInt<Self, B0>>;
    type Dec = If<bit::IsZero<Lsb_>, /*Then*/ Invalid, /*Else*/ Last<B0>>;

    type Bsr = Last<B0>;
    type Bsl = If<bit::IsZero<Lsb_>, /*Then*/ Last<B0>, UInt<Self, B0>>;

    type Add<Rhs: Unsigned> = If<bit::IsZero<Lsb_>, /*Then*/ Rhs, /*Else*/ Inc<Rhs>>;
    type AddWithCarry<Rhs: Unsigned, C: Bit> = UInt<
        If<bit::IsZero<bit::FullCarry<Lsb_, Lsb<Rhs>, C>>,/*Then*/ Rhs::Msb,/*Else*/ Inc<Rhs::Msb>>,
        bit::FullAdd<Lsb_, Lsb<Rhs>, C>,
    >;

    type Sub<Rhs: Unsigned> = If<bit::IsZero<Lsb_>,/*Then*/ Rhs, /*Else*/ Dec<Rhs>>;
    type SubWithBorrow<Rhs: Unsigned, B: Bit> = If<bit::IsZero<Lsb_>,/*Then*/ Sub<Rhs, Last<B>>, /*Else*/ Sub<Dec<Rhs>, Last<B>>>;

    type IsZero = bit::IsZero<Lsb_>;
    type RmExtraBits = Self;
    type Mul<Rhs: Unsigned> = If<bit::IsZero<Lsb_>, Last<B0>, Rhs>;
}

impl<Msbs: Unsigned, Lsb_: Bit> Unsigned for UInt<Msbs, Lsb_> {
    const USIZE: usize = { (Msbs::USIZE << 1) | Lsb_::USIZE };
    type Msb = Msbs;
    type Lsb = Lsb_;
    type Bsr = Msbs;
    type Bsl = UInt<Self, B0>;

    type Inc = If<bit::IsZero<Lsb_>, /*Then*/ UInt<Msbs, B1>, /*Else*/ UInt<Inc<Msbs>, B0>>;
    type Dec = If<bit::IsZero<Lsb_>, /*Then*/ UInt<Dec<Msbs>, B1>, /*Else*/ UInt<Msbs, B0>>;

    type Add<Rhs: Unsigned> = AddWithCarry<Self, Rhs, B0>;
    type AddWithCarry<Rhs: Unsigned, C: Bit> = UInt<
        AddWithCarry<Msbs, Rhs::Msb, bit::FullCarry<Lsb_, Rhs::Lsb, C>>, 
        bit::FullAdd<Lsb_, Lsb<Rhs>, C>
    >;

    type Sub<Rhs: Unsigned> = RmExtraBits<SubWithBorrow<Self, Rhs, B0>>;
    type SubWithBorrow<Rhs: Unsigned, B: Bit> = UInt<
        SubWithBorrow<Msbs, Msb<Rhs>, bit::FullBorrow<Lsb_, Rhs::Lsb, B>>,
        bit::FullDiff<Lsb_, Lsb<Rhs>, B>,
    >;

    type IsZero = bool::And<bit::IsZero<Lsb_>, Msbs::IsZero>;
    type RmExtraBits = If<Self::IsZero, /*Then*/ Last<B0>, /*Else*/ UInt<RmExtraBits<Msbs>, Lsb_>>;
    type Mul<Rhs: Unsigned> = Add<
        If<bit::IsZero<Lsb_>, /*Then*/ Last<B0>, /*Else*/ Rhs>,
        Bsl<Mul<Msbs, Rhs>>,
    >;
}



pub type U0  = uint!(B0);
pub type U1  = uint!(B1);
pub type U2  = uint!(B1, B0);
pub type U3  = uint!(B1, B1);
pub type U4  = uint!(B1, B0, B0);
pub type U5  = uint!(B1, B0, B1);
pub type U6  = uint!(B1, B1, B0);
pub type U7  = uint!(B1, B1, B1);
pub type U8  = uint!(B1, B0, B0, B0);
pub type U9  = uint!(B1, B0, B0, B1);
pub type U10 = uint!(B1, B0, B1, B0);
pub type U11 = uint!(B1, B0, B1, B1);
pub type U12 = uint!(B1, B1, B0, B0);
pub type U13 = uint!(B1, B1, B0, B1);
pub type U14 = uint!(B1, B1, B1, B0);
pub type U15 = uint!(B1, B1, B1, B1);
pub type U16 = uint!(B1, B0, B0, B0, B0);
pub type U17 = uint!(B1, B0, B0, B0, B1);
pub type U18 = uint!(B1, B0, B0, B1, B0);
pub type U19 = uint!(B1, B0, B0, B1, B1);
pub type U20 = uint!(B1, B0, B1, B0, B0);
pub type U21 = uint!(B1, B0, B1, B0, B1);
pub type U22 = uint!(B1, B0, B1, B1, B0);
pub type U23 = uint!(B1, B0, B1, B1, B1);
pub type U24 = uint!(B1, B1, B0, B0, B0);
pub type U25 = uint!(B1, B1, B0, B0, B1);
pub type U26 = uint!(B1, B1, B0, B1, B0);
pub type U27 = uint!(B1, B1, B0, B1, B1);
pub type U28 = uint!(B1, B1, B1, B0, B0);
pub type U29 = uint!(B1, B1, B1, B0, B1);
pub type U30 = uint!(B1, B1, B1, B1, B0);
pub type U31 = uint!(B1, B1, B1, B1, B1);
pub type U32 = uint!(B1, B0, B0, B0, B0, B0);

#[cfg(test)]
mod test {
    use super::*;

    fn test_pair<A: Unsigned, B: Unsigned>() {
        assert_eq!(A::USIZE, B::USIZE);
    }

    #[test]
    fn test_two() {
        test_pair::<<U1 as Unsigned>::Add<U2>, U3>();
        test_pair::<<U2 as Unsigned>::Sub<U2>, U0>();
        test_pair::<<U4 as Unsigned>::Mul<U3>, U12>();
        test_pair::<<U8 as Unsigned>::Bsl, U16>();
        test_pair::<<U8 as Unsigned>::Bsr, U4>();
    }
}
