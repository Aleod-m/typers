#![doc = include_str!("./list.md")]
use std::marker;

use crate::{
    bool::{Bool, False, If, True},
    num::unsigned::{self, IsZero, Unsigned, U0},
    seal, Invalid,
};

/// A macro to define simple TList types.
/// ```ignore
/// type Numbers = TList![i32, i16, i8];
/// // Expands to:
/// type Numbers = Cons<i32, Cons<i16, Cons<i8, End>>>;
/// ```
#[macro_export]
macro_rules! Tlist {
    ($ty1:ident, $($ty2:ident),*) => {
        Cat<$ty1, tlist!($($ty2),*)>
    };
    () => {
        End
    }
}

/// A macro to define TList values.
/// ```ignore
/// let a: TList![i32, i16, i8] = tlist![0i32, 1i16, 2i8];
/// // Expands to:
/// let a:  Cons<i32, Cons<i16, Cons<i8, End>>> = Cons {
///     head: 0i32,
///     tail: Cons {
///         head:1i16,
///         tail: Cons {
///             head:2i8,
///             tail:End,
///         }
///     }
/// };
/// ```
#[macro_export]
macro_rules! tlist {
    ($v1:expr, $($v2:expr),*) => {
        Cat{head:$v1, tail: tlist!($($v2),*)}
    };
    ($v:expr) => {
        Cat{head: $v, tail: End}
    };
    () => {
        End
    }
}

/// The Type List trait. Implemented by [struct@Cat] and [struct@End].
pub trait TList: Sized + seal::Sealed {
    /// Short hand for the usize value of the Len.
    const LEN: usize = <Self::Len as Unsigned>::USIZE;

    #[doc(hidden)]
    type IsEmpty: Bool;
    #[doc(hidden)]
    type Len: Unsigned;
    #[doc(hidden)]
    type Push<T>: TList;
    #[doc(hidden)]
    type Concat<L: TList>: TList;
    #[doc(hidden)]
    type _Reverse<L: TList>: TList;

    /// The length of the [trait@TList].
    fn len(&self) -> usize {
        Self::LEN
    }

    fn push<E>(self, elem: E) -> Push<Self, E>;
    fn reverse(self) -> Reverse<Self>;
    fn concat<L: TList>(self, other: L) -> Concat<Self, L>;

    // This method is hidden and sealed as its not intended to be called by the user.
    #[doc(hidden)]
    fn _reverse<T: TList>(self, list: T, _key: seal::Key) -> Self::_Reverse<T>;
}
impl TList for Invalid {
    type IsEmpty = Invalid;

    type Len = Invalid;

    type Push<T> = Invalid;

    type Concat<L: TList> = Invalid;

    type _Reverse<L: TList> = Invalid;

    fn push<E>(self, elem: E) -> Push<Self, E> {
        unreachable!()
    }

    fn reverse(self) -> Reverse<Self> {
        unreachable!()
    }

    fn concat<L: TList>(self, other: L) -> Concat<Self, L> {
        unreachable!()
    }

    fn _reverse<T: TList>(self, list: T, _key: seal::Key) -> Self::_Reverse<T> {
        unreachable!()
    }
}

/// Returns [struct@True] if [trait@TList] is empty [struct@False] otherwise.
pub type IsEmpty<L> = <L as TList>::IsEmpty;
/// Returns the Length of the TList as a [trait@Unsigned].
pub type Len<L> = <L as TList>::Len;
/// Construct the [trait@TList] with `T` as its head and `Self` as its tail.
pub type Push<L, T> = <L as TList>::Push<T>;
/// Return the [trait@TList] resulting of the Concatenation of `Lhs` and `Rhs`.
pub type Concat<Lhs, Rhs> = <Lhs as TList>::Concat<Rhs>;
/// Return the [trait@TList] that is the reverse of `L`.
pub type Reverse<L> = <L as TList>::_Reverse<End>;

/// A struct representing the head of the list concatenated with the rest (tail) of it.
#[derive(Debug)]
pub struct Cat<H, T: TList> {
    pub head: H,
    pub tail: T,
}

impl<H, T: TList> seal::Sealed for Cat<H, T> {}

/// A struct not holding any value representing the end of the Type list.
#[derive(Debug)]
pub struct End;
impl seal::Sealed for End {}

impl TList for End {
    type IsEmpty = True;
    type Len = U0;

    type Push<T> = Cat<T, End>;
    type Concat<L: TList> = L;
    type _Reverse<L: TList> = L;

    fn push<E>(self, elem: E) -> Push<Self, E> {
        Cat {
            head: elem,
            tail: self,
        }
    }

    fn reverse(self) -> Reverse<Self> {
        self
    }

    fn concat<L: TList>(self, other: L) -> Concat<Self, L> {
        other
    }

    fn _reverse<T: TList>(self, list: T, _key: seal::Key) -> Self::_Reverse<T> {
        list
    }
}

impl<H, T: TList> TList for Cat<H, T> {
    type IsEmpty = False;
    type Len = <T::Len as Unsigned>::Inc;

    type Push<E> = Cat<E, Self>;
    type Concat<L: TList> = Cat<H, T::Concat<L>>;
    type _Reverse<L: TList> = T::_Reverse<Cat<H, L>>;

    fn push<E>(self, elem: E) -> Push<Self, E> {
        Cat {
            head: elem,
            tail: self,
        }
    }

    fn reverse(self) -> Reverse<Self> {
        self._reverse(End, seal::Key {})
    }

    fn concat<L: TList>(self, other: L) -> Concat<Self, L> {
        Cat {
            head: self.head,
            tail: self.tail.concat(other),
        }
    }

    fn _reverse<L: TList>(self, list: L, _key: seal::Key) -> Self::_Reverse<L> {
        self.tail._reverse(
            Cat {
                head: self.head,
                tail: list,
            },
            seal::Key {},
        )
    }
}

/// A trait implemented for non empty [trait@TList].
pub trait NonEmpty: TList + seal::Sealed {
    #[doc(hidden)]
    type Head;
    #[doc(hidden)]
    type Tail: TList;

    fn head(&self) -> &Head<Self>;
    fn tail(&self) -> &Tail<Self>;
}

impl<H, T: TList> NonEmpty for Cat<H, T> {
    type Head = H;
    type Tail = T;

    fn head(&self) -> &Head<Self> {
        &self.head
    }

    fn tail(&self) -> &Tail<Self> {
        &self.tail
    }
}
/// The type representing the head of a [trait@NonEmpty] [trait@TList].
pub type Head<L> = <L as NonEmpty>::Head;
/// The type representing the tail of a [trait@NonEmpty] [trait@TList].
pub type Tail<L> = <L as NonEmpty>::Tail;

/// A trait to index the type list using type inference. It only works if the type list contains
/// no duplicate types.
pub trait TLFind<T, I> {
    fn find(&self) -> &T;
    fn find_mut(&mut self) -> &mut T;
}

/// Same as TLFind but clones the value.
pub trait TLClone<T: Clone, I>: TLFind<T, I> {
    fn find_and_clone(&self) -> T;
}

impl<T, I, L> TLClone<T, I> for L
where
    T: Clone,
    L: TLFind<T, I>,
{
    fn find_and_clone(&self) -> T {
        self.find().clone()
    }
}

/// A helper struct used in the [trait@TLFind] trait implementation. It should not be used directly it is constructed by the type
/// inference.
pub struct Here;
/// A helper struct used in [trait@TLFind] trait implementation. It should not be used directly it is constructed by the type
/// inference.
pub struct There<T>(marker::PhantomData<T>);
impl<T, Tail: TList> TLFind<T, Here> for Cat<T, Tail> {
    fn find(&self) -> &T {
        &self.head
    }

    fn find_mut(&mut self) -> &mut T {
        &mut self.head
    }
}

impl<Head, T, TailIdx, Tail: TList + TLFind<T, TailIdx>> TLFind<T, There<TailIdx>>
    for Cat<Head, Tail>
{
    fn find(&self) -> &T {
        self.tail.find()
    }

    fn find_mut(&mut self) -> &mut T {
        self.tail.find_mut()
    }
}

/// A trait that allows indexing into the Type List using an Unsigned type. OOB indexing returns
/// Unit.
pub trait TListIndex<'a>: NonEmpty
where
    Self: 'a,
{
    #[doc(hidden)]
    type Index<Idx: Unsigned>;
    #[doc(hidden)]
    type IndexMut<Idx: Unsigned>;

    fn index<Idx: Unsigned>(&'a self) -> Self::Index<Idx>;
    fn index_mut<Idx: Unsigned>(&'a mut self) -> Self::IndexMut<Idx>;
}
impl<'a, H> TListIndex<'a> for Cat<H, End>
where
    Self: 'a,
{
    type Index<Idx: Unsigned> = If<
        IsZero<Idx>,
        //Then
        &'a Head<Self>,
        //Else
        Invalid,
    >;
    type IndexMut<Idx: Unsigned> = If<
        IsZero<Idx>,
        //Then
        &'a mut Head<Self>,
        //Else
        Invalid,
    >;

    fn index<Idx: Unsigned>(&'a self) -> Self::Index<Idx> {
        IsZero::<Idx>::cond(|| &self.head, || Invalid)
    }

    fn index_mut<Idx: Unsigned>(&'a mut self) -> Self::IndexMut<Idx> {
        IsZero::<Idx>::cond(|| &mut self.head, || Invalid)
    }
}

impl<'a, H, T: TListIndex<'a>> TListIndex<'a> for Cat<H, T>
where
    Self: 'a,
{
    type Index<Idx: Unsigned> = If<
        IsZero<Idx>,
        //Then
        &'a Head<Self>,
        //Else
        Index<'a, Tail<Self>, unsigned::Dec<Idx>>,
    >;

    type IndexMut<Idx: Unsigned> = If<
        IsZero<Idx>,
        //Then
        &'a mut Head<Self>,
        //Else
        IndexMut<'a, Tail<Self>, unsigned::Dec<Idx>>,
    >;

    fn index<Idx: Unsigned>(&'a self) -> Self::Index<Idx> {
        IsZero::<Idx>::cond(|| &self.head, || self.tail.index::<unsigned::Dec<Idx>>())
    }

    fn index_mut<Idx: Unsigned>(&'a mut self) -> Self::IndexMut<Idx> {
        IsZero::<Idx>::cond(
            || &mut self.head,
            || self.tail.index_mut::<unsigned::Dec<Idx>>(),
        )
    }
}

/// The `&T` where `T` is at index `Idx` in the non empty [trait@TList] `L`.
pub type Index<'a, L, Idx> = <L as TListIndex<'a>>::Index<Idx>;
/// The `&mut T` where `T` is at index `Idx` in the non empty [trait@TList] `L`.
pub type IndexMut<'a, L, Idx> = <L as TListIndex<'a>>::IndexMut<Idx>;

#[cfg(test)]
mod test {

    use crate::num::unsigned::{U1, U4};

    use super::*;
    #[test]
    fn test_find() {
        let list: Push<Push<End, i32>, &'static str> = End.push(5i32).push("Foo");
        let a: i32 = *list.find();
        let b: &str = *list.find();
        assert!(a == 5i32);
        assert!(b == "Foo");
    }

    #[test]
    fn test_find_mut() {
        let mut list = End.push(5i32).push("Foo");
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
        let list = End.push("foo").push(5i32).push("bar");
        assert!(foo(&list) == 5);
    }

    #[test]
    fn test_tlist_concat() {
        let list_1 = End.push(0i32);
        let list_2 = End.push("Foo");
        let list_3 = list_1.concat(list_2);
        let a: i32 = *list_3.find();
        let b: &str = *list_3.find();
        assert!(a == 0i32);
        assert!(b == "Foo");
    }

    #[test]
    fn test_tlist_index() {
        let mut list = dbg!(tlist![2i32]);
        {
            let a = list.index_mut::<U0>();
            *a += 3;
        }
        assert!(list.index::<U0>() == &5i32);
    }

    #[test]
    #[should_panic]
    fn test_tlist_index_oob() {
        let list = dbg!(tlist![0i32, "Foo"]);
        let a: i32 = *list.index::<U0>();
        let b: &str = *list.index::<U1>();
        let c = list.index::<U4>();
        c.push(());
        assert!(a == 0i32);
        assert!(b == "Foo");
    }
}
