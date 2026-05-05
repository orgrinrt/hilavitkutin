//! S2 candidate B: recursive HList AccessSet.
//!
//! Pattern: AccessSet for `Empty` (terminator) and `Cons<H, T>`. Contains is
//! recursive: head matches X, or tail recursively contains X.
//!
//! The naive shape:
//!   impl<X, T> Contains<X> for Cons<X, T> {}                           // head
//!   impl<X, H, T: Contains<X>> Contains<X> for Cons<H, T> {}            // tail
//!
//! ...overlaps when H = X (same coherence problem as round-3 NotIn). This
//! sketch tries the natural shape first; if it fails (expected), it follows
//! up with the standard fix: position-witness via Here / There<P>.
//!
//! Build: `rustc --crate-type=lib --edition=2024 sketch.rs --emit=metadata`

#![allow(unused)]
#![no_std]

use core::marker::PhantomData;

// Markers.
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

// HList shape.
pub struct Empty;
pub struct Cons<H, T>(PhantomData<(H, T)>);

pub trait AccessSet {}
impl AccessSet for Empty {}
impl<H, T: AccessSet> AccessSet for Cons<H, T> {}

// Position witnesses.
pub struct Here;
pub struct There<P>(PhantomData<P>);

// Position-witnessed Contains. Distinct from naive shape because the
// position parameter disambiguates head from tail at the impl level.
pub trait Contains<X, P>: AccessSet {}

impl<X, T: AccessSet> Contains<X, Here> for Cons<X, T> {}
impl<X, H, T: AccessSet, P> Contains<X, There<P>> for Cons<H, T> where T: Contains<X, P> {}

// Existential wrapper: "there exists some P such that Contains<X, P>".
// Rust lacks first-class existentials at the type level, so we approximate
// via blanket impl. NOTE: this is the load-bearing question for B.
pub trait ContainsAny<X>: AccessSet {}
impl<X, S, P> ContainsAny<X> for S where S: Contains<X, P> + AccessSet {}

// Demonstration: build a 4-element list and assert membership.
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

fn require_contains_any<S: ContainsAny<X>, X>() {}

pub fn demo_arity_4() {
    require_contains_any::<S4, M2>();
}

pub fn demo_arity_12() {
    require_contains_any::<S12, M9>();
}

#[cfg(feature = "show_missing_error")]
pub fn demo_missing() {
    require_contains_any::<S4, M9>();
}
