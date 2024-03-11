use std::marker;

use crate::{
    bool::{Bool, False, True},
    seal, Invalid,
};

#[derive(Default)]
pub struct B0;
#[derive(Default)]
pub struct B1;

impl seal::Sealed for B0 {}
impl seal::Sealed for B1 {}

pub trait IBit: seal::Sealed {
    const USIZE: usize;
    type IsZero: Bool;
    type Not: IBit;
    type And<Bit: IBit>: IBit;
    type Or<Bit: IBit>: IBit;
    type Xor<Bit: IBit>: IBit;

    type Add<Bit: IBit>: IBit;
    type Carry<Bit: IBit>: IBit;
    type FullAdd<Bit: IBit, C: IBit>: IBit;
    type FullCarry<Bit: IBit, C: IBit>: IBit;

    type Diff<Bit: IBit>: IBit;
    type Borrow<Bit: IBit>: IBit;
    type FullDiff<Bit: IBit, B: IBit>: IBit;
    type FullBorow<Bit: IBit, B: IBit>: IBit;
}
impl IBit for Invalid {
    const USIZE: usize = { panic!("Invlid IBit type value!") };

    type IsZero = Invalid;

    type Not = Invalid;

    type And<Bit: IBit> = Invalid;

    type Or<Bit: IBit> = Invalid;

    type Xor<Bit: IBit> = Invalid;

    type Add<Bit: IBit> = Invalid;

    type Carry<Bit: IBit> = Invalid;

    type FullAdd<Bit: IBit, C: IBit> = Invalid;

    type FullCarry<Bit: IBit, C: IBit> = Invalid;

    type Diff<Bit: IBit> = Invalid;

    type Borrow<Bit: IBit> = Invalid;

    type FullDiff<Bit: IBit, B: IBit> = Invalid;

    type FullBorow<Bit: IBit, B: IBit> = Invalid;
}

impl IBit for B0 {
    const USIZE: usize = 0;
    type IsZero = True;
    type Not = B1;
    type And<Bit: IBit> = B0;
    type Or<Bit: IBit> = Bit;
    type Xor<Bit: IBit> = Bit;

    type Add<Bit: IBit> = Bit;
    type Carry<Bit: IBit> = B0;
    type FullAdd<Bit: IBit, C: IBit> = Bit::Add<C>;
    type FullCarry<Bit: IBit, C: IBit> = Bit::Carry<C>;

    type Diff<Bit: IBit> = Bit;
    type Borrow<Bit: IBit> = Bit;
    type FullDiff<Bit: IBit, B: IBit> = Bit::Xor<B>;
    type FullBorow<Bit: IBit, B: IBit> = Bit::Or<B>;
}

impl IBit for B1 {
    const USIZE: usize = 1;
    type IsZero = False;
    type Not = B0;
    type And<Bit: IBit> = B0;
    type Or<Bit: IBit> = Bit;
    type Xor<Bit: IBit> = Bit::Not;

    type Add<Bit: IBit> = Self::Xor<Bit>;
    type Carry<Bit: IBit> = Self::And<Bit>;
    type FullAdd<Bit: IBit, C: IBit> = <Self::Add<Bit> as IBit>::Add<C>;

    type FullCarry<Bit: IBit, C: IBit> =
        <<Self::Add<Bit> as IBit>::Carry<C> as IBit>::Xor<Self::Carry<Bit>>;

    type Diff<Bit: IBit> = Bit::Not;
    type Borrow<Bit: IBit> = B0;
    type FullDiff<Bit: IBit, B: IBit> = <Self::Diff<Bit> as IBit>::Diff<B>;
    type FullBorow<Bit: IBit, B: IBit> =
        <Self::Borrow<Bit> as IBit>::Xor<<Self::Diff<Bit> as IBit>::Borrow<B>>;
}
#[derive(Default)]
pub struct UInt<Msb: Unsigned, Lsb: IBit>(marker::PhantomData<(Msb, Lsb)>);
#[derive(Default)]
pub struct Msb<B: IBit>(marker::PhantomData<B>);

pub trait Unsigned {
    const USIZE: usize;
    type Msb: Unsigned;
    type Lsb: IBit;

    type Bsr: Unsigned;
    type Bsl: Unsigned;

    type Inc: Unsigned;
    type Dec: Unsigned;

    type Add<Rhs: Unsigned>: Unsigned;
    type AddWithCarry<Rhs: Unsigned, C: IBit>: Unsigned;

    type Sub<Rhs: Unsigned>: Unsigned;
    type SubWithBorrow<Rhs: Unsigned, B: IBit>: Unsigned;

    type IsZero: Bool;
    type RmExtraBits: Unsigned;
    type Mul<Rhs: Unsigned>: Unsigned;
}

impl Unsigned for Invalid {
    const USIZE: usize = { panic!("Invlid Unsigned Value!") };
    type Msb = Invalid;
    type Lsb = Invalid;

    type Bsr = Invalid;
    type Bsl = Invalid;

    type Inc = Invalid;
    type Dec = Invalid;

    type Add<Rhs: Unsigned> = Invalid;
    type AddWithCarry<Rhs: Unsigned, C: IBit> = Invalid;

    type Sub<Rhs: Unsigned> = Invalid;
    type SubWithBorrow<Rhs: Unsigned, B: IBit> = Invalid;

    type IsZero = Invalid;
    type RmExtraBits = Invalid;
    type Mul<Rhs: Unsigned> = Invalid;
}

impl<Lsb: IBit> Unsigned for Msb<Lsb> {
    const USIZE: usize = Lsb::USIZE;
    type Lsb = Lsb;
    type Msb = Invalid; // Shouldn't be accessed.

    type Inc = <Lsb::IsZero as Bool>::Ifuint<Msb<B1>, UInt<Self, B0>>;
    type Dec = <Lsb::IsZero as Bool>::Ifuint<Invalid, Msb<B0>>;

    type Bsr = Msb<B0>;
    type Bsl = <Lsb::IsZero as Bool>::Ifuint<Msb<B0>, UInt<Self, B0>>;

    type Add<Rhs: Unsigned> = <Lsb::IsZero as Bool>::Ifuint<Rhs, Rhs::Inc>;
    type AddWithCarry<Rhs: Unsigned, C: IBit> = UInt<
        <Self::Msb as Unsigned>::AddWithCarry<Rhs::Msb, Lsb::FullCarry<Rhs::Lsb, C>>,
        Lsb::FullAdd<Rhs::Lsb, C>,
    >;
    type Sub<Rhs: Unsigned> = <Lsb::IsZero as Bool>::Ifuint<Rhs, Rhs::Dec>;
    type SubWithBorrow<Rhs: Unsigned, B: IBit> = <Self::Sub<Rhs> as Unsigned>::Sub<Msb<B>>;

    type IsZero = Lsb::IsZero;
    type RmExtraBits = Self;
    type Mul<Rhs: Unsigned> = <Lsb::IsZero as Bool>::Ifuint<Msb<B0>, Rhs>;
}

impl<Msbs: Unsigned, Lsb: IBit> Unsigned for UInt<Msbs, Lsb> {
    const USIZE: usize = { (Msbs::USIZE << 1) | Lsb::USIZE };
    type Msb = Msbs;
    type Lsb = Lsb;
    type Bsr = Msbs;
    type Bsl = UInt<Self, B0>;

    type Inc = <Lsb::IsZero as Bool>::Ifuint<UInt<Msbs, B1>, UInt<Msbs::Inc, B0>>;
    type Dec = <Lsb::IsZero as Bool>::Ifuint<UInt<Msbs::Dec, B1>, UInt<Msbs, B0>>;

    type Add<Rhs: Unsigned> = Self::AddWithCarry<Rhs, B0>;
    type AddWithCarry<Rhs: Unsigned, C: IBit> =
        UInt<Msbs::AddWithCarry<Rhs::Msb, Lsb::FullCarry<Rhs::Lsb, C>>, Lsb::FullAdd<Rhs::Lsb, C>>;

    type Sub<Rhs: Unsigned> = <Self::SubWithBorrow<Rhs, B0> as Unsigned>::RmExtraBits;
    type SubWithBorrow<Rhs: Unsigned, B: IBit> = UInt<
        Msbs::SubWithBorrow<Rhs::Msb, Lsb::FullBorow<Rhs::Lsb, B>>,
        Lsb::FullDiff<Rhs::Lsb, B>,
    >;

    type IsZero = <Lsb::IsZero as Bool>::And<Msbs::IsZero>;
    type RmExtraBits = <Self::IsZero as Bool>::Ifuint<Msb<B0>, UInt<Msbs::RmExtraBits, Lsb>>;
    type Mul<Rhs: Unsigned> = <<<Lsb::IsZero as Bool>::Ifuint<Msb<B0>, Rhs> as Unsigned>::Add<
        UInt<Msbs::Mul<Rhs>, B0>,
    > as Unsigned>::RmExtraBits;
}

#[macro_export]
macro_rules! uint {
    ($ty:ident $(, $rest:ident)*) => {
        uint!(@parse input: [$($rest),*], out: [Msb<$ty>])
    };

    (@parse input: [$ty:ident $(, $rest:ident)*], out: [$($out:tt)*]) => {
        uint!(@parse input: [$($rest),*], out: [UInt<$($out)*, $ty>])
    };
    (@parse input: [], out: [$($out:tt)*]) => {
        $($out)*
    };
}

pub type U0 = uint!(B0);
pub type U1 = uint!(B1);
pub type U2 = uint!(B1, B0);
pub type U3 = uint!(B1, B1);
pub type U4 = uint!(B1, B0, B0);
pub type U5 = uint!(B1, B0, B1);
pub type U6 = uint!(B1, B1, B0);
pub type U7 = uint!(B1, B1, B1);

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
    }
}
