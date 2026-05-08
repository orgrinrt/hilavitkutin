//! B v2: Move position witness to associated type to dodge unconstrained-
//! type-param error. The user writes `S: Find<X>` and the position lives
//! at `<S as Find<X>>::Pos`. Inference resolves it.

#![allow(unused)]
#![no_std]

use core::marker::PhantomData;

pub struct M0;
pub struct M1;
pub struct M2;
pub struct M3;
pub struct M4;
pub struct M5;
pub struct M6;
pub struct M7;
pub struct M8;
pub struct M9;
pub struct M10;
pub struct M11;

pub struct Empty;
pub struct Cons<H, T>(PhantomData<(H, T)>);

pub trait AccessSet {}
impl AccessSet for Empty {}
impl<H, T: AccessSet> AccessSet for Cons<H, T> {}

pub struct Here;
pub struct There<P>(PhantomData<P>);

// Find<X> says "X is in this set", with the position recoverable as Pos.
// Two rules: head match, or recurse into tail. Same coherence overlap
// risk as before; this variant exists to verify whether moving position
// to associated type changes the answer.
pub trait Find<X>: AccessSet {
    type Pos;
}

impl<X, T: AccessSet> Find<X> for Cons<X, T> {
    type Pos = Here;
}

impl<X, H, T> Find<X> for Cons<H, T>
where
    T: Find<X> + AccessSet,
{
    type Pos = There<<T as Find<X>>::Pos>;
}

type S4 = Cons<M0, Cons<M1, Cons<M2, Cons<M3, Empty>>>>;
type S12 = Cons<
    M0,
    Cons<
        M1,
        Cons<
            M2,
            Cons<
                M3,
                Cons<
                    M4,
                    Cons<M5, Cons<M6, Cons<M7, Cons<M8, Cons<M9, Cons<M10, Cons<M11, Empty>>>>>>>,
                >,
            >,
        >,
    >,
>;

fn require_find<S: Find<X>, X>() {}

pub fn demo_arity_4() {
    require_find::<S4, M2>();
}

pub fn demo_arity_12() {
    require_find::<S12, M9>();
}

#[cfg(feature = "show_missing_error")]
pub fn demo_missing() {
    require_find::<S4, M9>();
}
