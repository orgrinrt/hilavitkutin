//! B v4: HList with `#[marker]` Contains. The natural shape (head match
//! + tail recurse) overlaps when H = X, but `#[marker]` allows it.
//!
//! Build: `rustup run nightly rustc --crate-type=lib --edition=2024 sketch_v4_marker.rs --emit=metadata`

#![allow(unused)]
#![no_std]
#![feature(marker_trait_attr)]

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

#[marker]
pub trait Contains<X>: AccessSet {}

// Head match. When H = X this overlaps with tail rule, allowed by #[marker].
impl<X, T: AccessSet> Contains<X> for Cons<X, T> {}

// Tail recurse.
impl<X, H, T: AccessSet> Contains<X> for Cons<H, T> where T: Contains<X> {}

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

fn require_contains<S: Contains<X>, X>() {}

pub fn demo_arity_4() {
    require_contains::<S4, M2>();
}

pub fn demo_arity_12() {
    require_contains::<S12, M9>();
}

#[cfg(feature = "show_missing_error")]
pub fn demo_missing() {
    require_contains::<S4, M9>();
}
