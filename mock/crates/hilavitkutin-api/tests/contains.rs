//! Compile-time membership: `Holds<S>` for a bare marker at every place of a
//! list, and `HoldsAll<L>` for the stores a whole list stands for.
//!
//! Each `requires_*` call forces the bound at monomorphisation against a
//! `read!`-built list. The test is that this file compiles.

#![no_std]

use arvo::Cap;
use hilavitkutin_api::access::{AccessSet, Contains, Holds, HoldsAll};
use hilavitkutin_api::{
    Accum,
    Buffered,
    Column,
    Differs,
    Exchange,
    Reach,
    Resource,
    Virtual,
    Written,
    read,
    write,
};

#[derive(Clone, Copy, PartialEq)]
struct A;
#[derive(Clone, Copy, PartialEq)]
struct B;
#[derive(Clone, Copy)]
struct C;
struct D;
struct E;
struct F;

const ONE: Cap = arvo_tensor::cap(1);

fn requires<S, Ts: AccessSet + Holds<S>>() {}

// `Holds` holds exactly where the shared crate's `Contains` does.
fn requires_contains<S, Ts: AccessSet + Contains<S>>() {}

type Six = read![Column<A>, Resource<D>, Buffered<B>, Accum<A>, Virtual<E>, Resource<F>];

#[test]
fn holds_at_the_head() {
    requires::<Column<A>, read![Column<A>]>();
    requires_contains::<Column<A>, read![Column<A>]>();
}

#[test]
fn holds_at_every_place_of_a_list_of_six() {
    requires::<Column<A>, Six>();
    requires::<Resource<D>, Six>();
    requires::<Buffered<B>, Six>();
    requires::<Accum<A>, Six>();
    requires::<Virtual<E>, Six>();
    requires::<Resource<F>, Six>();
    requires_contains::<Resource<F>, Six>();
}

// A buffered column's mode is part of the marker.
#[test]
fn holds_tells_the_two_modes_apart() {
    requires::<Buffered<A, Exchange>, read![Buffered<A>, Buffered<A, Exchange>]>();
    requires::<Buffered<A>, read![Buffered<A>, Buffered<A, Exchange>]>();
}

fn requires_all<L, Ts: AccessSet + HoldsAll<L>>() {}

type Stores = read![Column<A>, Column<C>, Buffered<B>, Resource<D>, Accum<A>, Virtual<E>];

#[test]
fn holds_all_of_the_empty_list() {
    requires_all::<read![], read![]>();
    requires_all::<read![], Stores>();
}

#[test]
fn holds_all_of_bare_markers() {
    requires_all::<read![Column<A>, Resource<D>], Stores>();
    requires_all::<write![Accum<A>, Virtual<E>, Buffered<B>], Stores>();
}

// A wrapper asks of registration what the bare store would: the walk reads
// `Member::Store`, so no wrapper is itself looked for in the store list.
#[test]
fn holds_all_resolves_a_wrapper_to_its_store() {
    requires_all::<read![Reach<Column<A>, ONE>, Reach<Buffered<B>, ONE>, Resource<D>], Stores>();
    requires_all::<write![Written<Column<C>>, Differs<Column<A>>, Differs<Buffered<B>>], Stores>();
}

// The same store named twice, once bare and once wrapped, is held once.
#[test]
fn holds_all_of_a_store_named_twice() {
    requires_all::<read![Column<A>, Reach<Column<A>, ONE>], Stores>();
}
