//! B v3: try `feature(min_specialization)` to break the overlap. The
//! workspace forbids unstable specialisation per other rules; this sketch
//! exists only to confirm whether the overlap goes away when the feature
//! is enabled (data point) and to record the finding even though the
//! resolution path is closed.

#![allow(unused, incomplete_features)]
#![no_std]
#![feature(min_specialization)]

use core::marker::PhantomData;

pub struct M0;
pub struct M1;
pub struct M2;
pub struct M3;

pub struct Empty;
pub struct Cons<H, T>(PhantomData<(H, T)>);

pub trait AccessSet {}
impl AccessSet for Empty {}
impl<H, T: AccessSet> AccessSet for Cons<H, T> {}

pub trait Find<X>: AccessSet {
    fn _seal(&self) {}
}

// `default` makes the recursive impl specialisable; the head impl
// overrides it for the concrete `Cons<X, T>` shape.
impl<X, H, T> Find<X> for Cons<H, T>
where
    T: Find<X> + AccessSet,
{
    default fn _seal(&self) {}
}

// Head impl is the specialisation. Override the seal method.
impl<X, T: AccessSet> Find<X> for Cons<X, T> {
    fn _seal(&self) {}
}

type S4 = Cons<M0, Cons<M1, Cons<M2, Cons<M3, Empty>>>>;

fn require_find<S: Find<X>, X>() {}

pub fn demo_arity_4() {
    require_find::<S4, M2>();
}
