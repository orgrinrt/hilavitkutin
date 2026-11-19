//! The model: notko-hlist's product with the carrier as a parameter of the
//! product trait, over the shipped list, places and `Elem`.
//!
//! The carrier is a typestate. It is a marker whose type, lifetimes included,
//! is the state, and for each member it states exactly one form, through one
//! impl of `Carrier<M>` with an associated type. The product type names the
//! carrier and every member, `PCons<C, H, T>`, so a product never stands for
//! any other list or any other carrier, even where the field types coincide.
#![no_std]
#![feature(const_trait_impl)]

use core::marker::PhantomData;

pub use notko_hlist::{Cons, Elem, Empty, Here, List, Place, PlaceOf, There};

mod seal {
    pub struct Token;
}

/// Carrier `Self` holds `Of` for member `M`. One impl per pair, so one form.
pub trait Carrier<M> {
    type Of;
}

/// A carrier holding one form for every `Elem` member, over its value.
pub trait Uniform {
    type Of<E: Elem>;
}

impl<C: Uniform, E: Elem> Carrier<E> for C {
    type Of = C::Of<E>;
}

pub struct Val;
pub struct Col<'a>(PhantomData<&'a ()>);
pub struct Arr<const N: usize>;

impl Uniform for Val {
    type Of<E: Elem> = E::Value;
}
impl<'a> Uniform for Col<'a> {
    type Of<E: Elem> = &'a [E::Value];
}
impl<const N: usize> Uniform for Arr<N> {
    type Of<E: Elem> = [E::Value; N];
}

pub const trait ByRow: Uniform {
    fn at<E: Elem>(c: &Self::Of<E>, i: usize) -> E::Value;
}
const impl<'a> ByRow for Col<'a> {
    fn at<E: Elem>(c: &Self::Of<E>, i: usize) -> E::Value {
        c[i]
    }
}
const impl<const N: usize> ByRow for Arr<N> {
    fn at<E: Elem>(c: &Self::Of<E>, i: usize) -> E::Value {
        c[i]
    }
}

/// A cell: what `C` holds for `H`, then the rest. Names `C` and `H`.
pub struct PCons<C: Carrier<H>, H, T> {
    pub head: <C as Carrier<H>>::Of,
    pub tail: T,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PNil;

/// The product of the list `Self` through the carrier `C`. Sealed.
pub trait Project<C>: List {
    type Prod;
    #[doc(hidden)]
    fn sealed(_: seal::Token);
}

impl<C> Project<C> for Empty {
    type Prod = PNil;

    fn sealed(_: seal::Token) {}
}

impl<C: Carrier<H>, H, T: Project<C>> Project<C> for Cons<H, T> {
    type Prod = PCons<C, H, T::Prod>;

    fn sealed(_: seal::Token) {}
}

/// The field of member `M` through carrier `C`, at place `P`.
pub const trait Has<C: Carrier<M>, M, P: Place>: Project<C> + PlaceOf<M, P> {
    fn pick(p: &Self::Prod) -> &<C as Carrier<M>>::Of;
}

const impl<C: Carrier<M>, M, T: Project<C>> Has<C, M, Here> for Cons<M, T> {
    fn pick(p: &Self::Prod) -> &<C as Carrier<M>>::Of {
        &p.head
    }
}

const impl<C, H, M, P, T> Has<C, M, There<P>> for Cons<H, T>
where
    C: Carrier<H> + Carrier<M>,
    P: Place,
    T: [const] Has<C, M, P>,
{
    fn pick(p: &Self::Prod) -> &<C as Carrier<M>>::Of {
        T::pick(&p.tail)
    }
}

/// A walk between two carriers' products: the values of one row. The
/// supertraits are what let the recursion see the tail's two products.
pub const trait Rows<C: Uniform>: Project<C> + Project<Val> {
    fn read_row(p: &<Self as Project<C>>::Prod, i: usize) -> <Self as Project<Val>>::Prod
    where
        C: [const] ByRow;
}

const impl<C: Uniform> Rows<C> for Empty {
    fn read_row(_: &PNil, _: usize) -> PNil
    where
        C: [const] ByRow,
    {
        PNil
    }
}

const impl<C: Uniform, H: Elem, T: [const] Rows<C>> Rows<C> for Cons<H, T> {
    fn read_row(p: &<Self as Project<C>>::Prod, i: usize) -> <Self as Project<Val>>::Prod
    where
        C: [const] ByRow,
    {
        PCons {
            head: C::at::<H>(&p.head, i),
            tail: T::read_row(&p.tail, i),
        }
    }
}

/// A walk whose second product carries a lifetime: a shared column over each
/// member's array. The second supertrait is higher-ranked.
pub trait Cols<const N: usize>: Project<Arr<N>> + for<'a> Project<Col<'a>> {
    fn cols(p: &<Self as Project<Arr<N>>>::Prod) -> <Self as Project<Col<'_>>>::Prod;
}

impl<const N: usize> Cols<N> for Empty {
    fn cols(_: &PNil) -> PNil {
        PNil
    }
}

impl<const N: usize, H: Elem, T: Cols<N>> Cols<N> for Cons<H, T> {
    fn cols(p: &<Self as Project<Arr<N>>>::Prod) -> <Self as Project<Col<'_>>>::Prod {
        PCons {
            head: &p.head,
            tail: T::cols(&p.tail),
        }
    }
}
