//! Type-level set inclusion and difference, inspired by frunk's approach: https://archive.is/YwDMX

use std::marker::PhantomData;

/* ------------------------- Helpers ----------------------- */

/// The final element of a type-level Cons list.
pub enum End {}

/// A type that signals to the compiler that we focus non-recursively
/// on a particular element, while importantly differentiating it from
/// `There` which allows the compiler to understand that there are no
/// conflicting trait implementations that overlap.
pub enum Here {}

/// The complement to `Here`, this allows the compiler to reason about
/// the recursive case of trait resolution in contrast to the `Here`
/// base case. We have to keep wrapping `Index` in a way that is similar
/// in structure to `Cons` because it prevents the compiler from
/// detecting a conflicting implementation in the recursive case.
pub struct There<Index>(PhantomData<Index>);

/// A compile-time list of types, similar to other basic functional list structures.
pub struct Cons<Head, Tail>(PhantomData<Head>, Tail);

/* ------------------------- TypeSet implemented for tuples ----------------------- */

pub trait TypeSet {
    type TList;
}

impl TypeSet for () {
    type TList = End;
}

impl<A> TypeSet for (A,) {
    type TList = Cons<A, End>;
}

impl<A, B> TypeSet for (A, B) {
    type TList = Cons<A, Cons<B, End>>;
}

impl<A, B, C> TypeSet for (A, B, C) {
    type TList = Cons<A, Cons<B, Cons<C, End>>>;
}

impl<A, B, C, D> TypeSet for (A, B, C, D) {
    type TList = Cons<A, Cons<B, Cons<C, Cons<D, End>>>>;
}

impl<A, B, C, D, E> TypeSet for (A, B, C, D, E) {
    type TList = Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, End>>>>>;
}

impl<A, B, C, D, E, F> TypeSet for (A, B, C, D, E, F) {
    type TList = Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, Cons<F, End>>>>>>;
}

/* ------------------------- Contains ----------------------- */

/// A trait that assists with compile-time type set inclusion testing.
/// The `Index` parameter is either `Here` or `There` depending on
/// whether the trait implementation is a base case or the recursive
/// case.
pub trait Contains<T, Index> {}

/// Base case implementation for when the Cons Head is T.
impl<T, Tail> Contains<T, Here> for Cons<T, Tail> {}

/// Recursive case for when the Cons Tail contains T.
impl<T, Index, Head, Tail> Contains<T, There<Index>> for Cons<Head, Tail> where
    Tail: Contains<T, Index>
{
}

/* ------------------------- Narrow ----------------------- */

/// A trait for pulling a specific type out of a TList at compile-time
/// and having access to the other types as the Remainder.
pub trait Narrow<Target, Index> {
    type Remainder;
}

/// Base case where the search Target is in the Head of the TList.
impl<Target, Tail> Narrow<Target, Here> for Cons<Target, Tail> {
    type Remainder = Tail;
}

/// Recursive case where the search Target is in the Tail of the TList.
impl<Head, Tail, Target, Index> Narrow<Target, There<Index>> for Cons<Head, Tail>
where
    Tail: Narrow<Target, Index>,
{
    type Remainder = Cons<Head, <Tail as Narrow<Target, Index>>::Remainder>;
}

fn _narrow_test() {
    fn can_narrow<Types, Target, Remainder, Index>()
    where
        Types: Narrow<Target, Index, Remainder = Remainder>,
    {
    }

    type T0 = <(u32, String) as TypeSet>::TList;

    can_narrow::<T0, u32, _, _>();
    can_narrow::<T0, String, Cons<u32, End>, _>();
}

/* ------------------------- SupersetOf ----------------------- */

/// When all types in a TList are present in a second TList
pub trait SupersetOf<Other, Index> {}

/// Base case
impl<T> SupersetOf<End, End> for T {}

/// Recursive case - more complex because we have to reason about the Index itself as a
/// heterogenous list.
impl<SubHead, SubTail, SuperHead, SuperTail, HeadIndex, TailIndex>
    SupersetOf<Cons<SubHead, SubTail>, Cons<HeadIndex, TailIndex>> for Cons<SuperHead, SuperTail>
where
    Cons<SuperHead, SuperTail>: Narrow<SubHead, HeadIndex>,
    <Cons<SuperHead, SuperTail> as Narrow<SubHead, HeadIndex>>::Remainder:
        SupersetOf<SubTail, TailIndex>,
{
}

fn _superset_test() {
    fn is_superset<S1, S2, Index>()
    where
        S1: SupersetOf<S2, Index>,
    {
    }

    type T0 = <(u32,) as TypeSet>::TList;
    type T1 = <(u32, String) as TypeSet>::TList;
    type T2 = <(u32, String, i32) as TypeSet>::TList;

    is_superset::<T0, T0, _>();
    is_superset::<T1, T1, _>();
    is_superset::<T2, T2, _>();
    is_superset::<T1, T0, _>();
    is_superset::<T2, T0, _>();
    is_superset::<T2, T1, _>();
}
