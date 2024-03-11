use std::marker;

use crate::{
    bool::{Bool, False, True},
    num::{Unsigned, U0},
    seal, Invalid,
};

#[macro_export]
macro_rules! Tlist {
    ($ty1:ident, $($ty2:ident),*) => {
        Cons<$ty1, tlist!($($ty2),*)>
    };
    () => {
        Nil
    }
}

#[macro_export]
macro_rules! tlist {
    ($v1:expr, $($v2:expr),*) => {
        Cons{head:$v1, tail: tlist!($($v2),*)}
    };
    ($v:expr) => {
        Cons{head: $v, tail: Nil}
    };
    () => {
        Nil
    }
}

pub trait TList: Sized + seal::Sealed {
    type IsEmpty: Bool;
    type Len: Unsigned;

    type Push<T>: TList;
    type Pop: TList;
    type Concat<L: TList>: TList;
    type Reverse: TList;
    type _Reverse<L: TList>: TList;

    type Head;
    type Tail: TList;

    fn push<E>(self, elem: E) -> Self::Push<E>;
    fn reverse(self) -> Self::Reverse;
    fn concat<L: TList>(self, other: L) -> Self::Concat<L>;
    fn _reverse<T: TList>(self, list: T, _key: seal::Key) -> Self::_Reverse<T>;
    //    fn call_on<F, E>(&mut self, f: F) where F: Fn(&mut E);
}

impl TList for Invalid {
    type IsEmpty = Invalid;

    type Len = Invalid;

    type Push<T> = Invalid;

    type Pop = Invalid;

    type Concat<L: TList> = Invalid;

    type Reverse = Invalid;

    type _Reverse<L: TList> = Invalid;

    type Head = Invalid;

    type Tail = Invalid;

    fn push<E>(self, _elem: E) -> Self::Push<E> {
        panic!("Attempted to push to an Invalid TList!")
    }

    fn reverse(self) -> Self::Reverse {
        panic!("Attempted to reverse an Invalid TList!")
    }

    fn concat<L: TList>(self, _other: L) -> Self::Concat<L> {
        panic!("Attempted to concat an Invalid TList!")
    }

    fn _reverse<T: TList>(self, _list: T, _key: seal::Key) -> Self::_Reverse<T> {
        panic!("Attempted to reverse an Invalid TList!")
    }
}

#[derive(Debug)]
pub struct Cons<H, T: TList> {
    pub head: H,
    pub tail: T,
}
impl<H, T: TList> seal::Sealed for Cons<H, T> {}

#[derive(Debug)]
pub struct Nil;
impl seal::Sealed for Nil {}

impl TList for Nil {
    type Head = Nil;
    type Tail = Nil;

    type IsEmpty = True;
    type Len = U0;

    type Push<T> = Cons<T, Nil>;
    type Pop = Nil;
    type Concat<L: TList> = L;
    type Reverse = Nil;
    type _Reverse<L: TList> = L;

    fn push<E>(self, elem: E) -> Self::Push<E> {
        Cons {
            head: elem,
            tail: self,
        }
    }

    fn reverse(self) -> Self::Reverse {
        self
    }

    fn concat<L: TList>(self, other: L) -> Self::Concat<L> {
        other
    }

    fn _reverse<T: TList>(self, list: T, _key: seal::Key) -> Self::_Reverse<T> {
        list
    }
}

impl<H, T: TList> TList for Cons<H, T> {
    type IsEmpty = False;
    type Len = <T::Len as Unsigned>::Inc;

    type Push<E> = Cons<E, Self>;
    type Pop = T;

    type Concat<L: TList> = Cons<H, T::Concat<L>>;
    type Reverse = Self::_Reverse<Nil>;
    type _Reverse<L: TList> = T::_Reverse<Cons<H, L>>;
    type Head = H;
    type Tail = T;

    fn push<E>(self, elem: E) -> Self::Push<E> {
        Cons {
            head: elem,
            tail: self,
        }
    }

    fn reverse(self) -> Self::Reverse {
        self._reverse(Nil, seal::Key {})
    }

    fn concat<L: TList>(self, other: L) -> Self::Concat<L> {
        Cons {
            head: self.head,
            tail: self.tail.concat(other),
        }
    }
    fn _reverse<L: TList>(self, list: L, _key: seal::Key) -> Self::_Reverse<L> {
        self.tail._reverse(
            Cons {
                head: self.head,
                tail: list,
            },
            seal::Key {},
        )
    }
}

pub trait TLFind<T, I> {
    fn find(&self) -> &T;
    fn find_mut(&mut self) -> &mut T;
}

pub trait TLClone<T: Clone, I>: TLFind<T, I> {
    fn find_and_copy(&self) -> T;
}

impl<T, I, L> TLClone<T, I> for L
where
    T: Clone,
    L: TLFind<T, I>,
{
    fn find_and_copy(&self) -> T {
        self.find().clone()
    }
}

pub struct Here;
pub struct There<T>(marker::PhantomData<T>);
impl<T, Tail: TList> TLFind<T, Here> for Cons<T, Tail> {
    fn find(&self) -> &T {
        &self.head
    }

    fn find_mut(&mut self) -> &mut T {
        &mut self.head
    }
}

impl<Head, T, TailIdx, Tail: TList + TLFind<T, TailIdx>> TLFind<T, There<TailIdx>>
    for Cons<Head, Tail>
{
    fn find(&self) -> &T {
        self.tail.find()
    }

    fn find_mut(&mut self) -> &mut T {
        self.tail.find_mut()
    }
}

pub trait TListIndex<'a>: TList
where
    Self: 'a,
{
    type Index<Idx: Unsigned>;
    fn index<Idx: Unsigned>(&'a self) -> Self::Index<Idx>;
}

impl TListIndex<'_> for Nil {
    type Index<Idx: Unsigned> = Invalid;

    fn index<Idx: Unsigned>(&self) -> Self::Index<Idx> {
        Invalid
    }
}

impl<'a, H, T: TListIndex<'a>> TListIndex<'a> for Cons<H, T>
where
    Self: 'a,
{
    type Index<Idx: Unsigned> =
        <Idx::IsZero as Bool>::If<&'a Self::Head, <Self::Tail as TListIndex<'a>>::Index<Idx::Dec>>;

    fn index<Idx: Unsigned>(&'a self) -> Self::Index<Idx> {
        <Idx::IsZero as Bool>::cond(|| &self.head, || self.tail.index::<Idx::Dec>())
    }
}

#[cfg(test)]
mod test {
    use crate::num::*;

    use super::*;
    #[test]
    fn test_find() {
        let list = Nil.push(5i32).push("Foo");
        let a: i32 = *list.find();
        let b: &str = *list.find();
        assert!(a == 5i32);
        assert!(b == "Foo");
    }

    #[test]
    fn test_find_mut() {
        let mut list = Nil.push(5i32).push("Foo");
        *list.find_mut() = 6i32;
        *list.find_mut() = "Bar";
        let a: i32 = *list.find();
        let b: &str = *list.find();
        assert!(a == 6i32);
        assert!(b == "Bar");
    }

    #[test]
    fn test_index_as_type_parameter() {
        fn foo<I, L: TLFind<i32, I>>(list: &L) -> i32 {
            *list.find()
        }
        let list = Nil.push("foo").push(5i32).push("bar");
        assert!(foo(&list) == 5);
    }

    #[test]
    fn test_tlist_concat() {
        let list_1 = Nil.push(0i32);
        let list_2 = Nil.push("Foo");
        let list_3 = list_1.concat(list_2);
        let a: i32 = *list_3.find();
        let b: &str = *list_3.find();
        assert!(a == 0i32);
        assert!(b == "Foo");
    }

    #[test]
    fn test_tlist_index() {
        let list = dbg!(tlist![0i32, "Foo"]);
        let a: i32 = *list.index::<U0>();
        let b: &str = *list.index::<U1>();
        assert!(a == 0i32);
        assert!(b == "Foo");
    }
}
