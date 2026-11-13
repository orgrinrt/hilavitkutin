//! What a buffered column is registered with, by its element type.
//!
//! `start` is the type's trivial constructor where it has one and its default
//! where it has none, and `start_from` takes the value for every type. The
//! two refusals are held elsewhere: `start` on a type with neither by a
//! `compile_fail` doctest on `Buffered::start`, and the bare marker handed to
//! a builder by a fixture of the trybuild harness.

#![no_std]
#![feature(const_trait_impl)]

use hilavitkutin_api::builder_input::BuilderInput;
use hilavitkutin_api::{Buffered, Exchange, StagedBuffered, StartsFromDefault, StoreDispatch};

#[derive(Copy, Clone, PartialEq, Debug)]
struct Built(u32);

impl notko::HasTrivialCtor for Built {
    fn new() -> Self {
        Built(7)
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Defaulted(u32);

// A constructor and a default that disagree, so the value says which one
// `start` took.
#[derive(Copy, Clone, PartialEq, Debug)]
struct Both(u32);

impl notko::HasTrivialCtor for Both {
    fn new() -> Self {
        Both(7)
    }
}

impl Default for Both {
    fn default() -> Self {
        Both(9)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Neither(u32);

#[test]
fn a_type_with_a_constructor_starts_from_it() {
    assert_eq!(Buffered::<Built>::start().into_inner(), Built(7));
}

#[test]
fn a_type_with_a_default_alone_starts_from_it() {
    assert_eq!(Buffered::<Defaulted>::start().into_inner(), Defaulted(0));
}

#[test]
fn a_type_with_both_starts_from_its_constructor() {
    assert_eq!(Buffered::<Both>::start().into_inner(), Both(7));
    assert_eq!(Buffered::<Both, Exchange>::start().into_inner(), Both(7));
}

// The trait's function is still there by its own path, and it is the default.
#[test]
fn the_default_of_a_type_with_both_is_reached_by_the_trait_s_path() {
    let staged = <Buffered<Both> as StartsFromDefault>::start();
    assert_eq!(staged.into_inner(), Both(9));
}

#[test]
fn an_explicit_start_is_there_for_every_type() {
    assert_eq!(
        Buffered::<Neither>::start_from(Neither(3)).into_inner(),
        Neither(3)
    );
    assert_eq!(Buffered::<Both>::start_from(Both(1)).into_inner(), Both(1));
}

// Where the constructor is const the start is too, so a store's start can be
// a constant of the program.
#[derive(Copy, Clone, PartialEq, Debug)]
struct Fixed(u32);

const impl notko::HasTrivialCtor for Fixed {
    fn new() -> Self {
        Fixed(4)
    }
}

const FIXED: StagedBuffered<Fixed> = Buffered::<Fixed>::start();
const GIVEN: StagedBuffered<Neither, Exchange> = Buffered::start_from(Neither(6));

#[test]
fn a_start_is_made_in_a_constant() {
    assert_eq!(FIXED.into_inner(), Fixed(4));
    assert_eq!(GIVEN.into_inner(), Neither(6));
}

// The carrier is registered as the store its marker names, in its mode, and
// hands the builder the element value.
#[test]
fn the_carrier_registers_the_marker_s_store() {
    trait SameAs<B> {}
    impl<T> SameAs<T> for T {}
    fn dispatch<I: BuilderInput<Dispatch = D, Init = V>, D, V>(_: &I)
    where
        D: SameAs<StoreDispatch<Buffered<Neither, Exchange>>>,
        V: SameAs<Neither>,
    {
    }
    let staged: StagedBuffered<Neither, Exchange> = Buffered::start_from(Neither(3));
    dispatch(&staged);
}
