//! Kit preset trait for the hilavitkutin scheduler builder.
//!
//! Round 4 declarative shape: a Kit names what it owns
//! (`type Owned: StoreBundle`) and what work it contributes
//! (`type Units: WorkUnitBundle`). The engine's
//! `SchedulerBuilder::with(my_kit)` reads these at compile time.
//! No `install` method, no `B` parameter, no `Output`.
//!
//! Round 202605091700: `Kit` gains `Provider<Init = Self>` as a
//! supertrait, unifying the `SchedulerBuilder::with(value)` surface.
//! `KitDispatch<K>` is the matching `Dispatch` router; it concats
//! `K::Units` into `Wus` and `K::Owned` into `Stores`. Lives in
//! this crate (not in `hilavitkutin-api`) because referencing
//! `Kit::Units` / `Kit::Owned` requires the api -> kit layering
//! to invert, which is forbidden by the engine layering rules.

#![no_std]
#![recursion_limit = "512"]

use core::marker::PhantomData;

use hilavitkutin_api::builder_input::{BuilderInput, Dispatch};
use hilavitkutin_api::access::Concat;
use hilavitkutin_api::{StoreBundle, WorkUnitBundle};

/// A declarative bundle of WorkUnits and Owned stores.
///
/// `Units` names the WorkUnits the kit contributes (Cons-list of
/// WorkUnit types satisfying `WorkUnitBundle`). `Owned` names the
/// Resources / Columns / Virtuals the kit owns (Cons-list of
/// store-marker types satisfying `StoreBundle`).
///
/// Kit authors implement `Kit` by naming `Units` and `Owned` as
/// associated types AND by impling `Provider<Init = Self>` with
/// `Dispatch = KitDispatch<Self>` and `KIND = ProviderKind::Kit`.
/// The engine's `SchedulerBuilder::with(my_kit)` reads `K::Units`
/// and `K::Owned` at compile time and accumulates them into the
/// builder typestate via `KitDispatch`. App-side wiring (default
/// values for Resources, Replaceable opt-in) lives at the call site.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a Kit",
    note = "Implement `Kit` by declaring `type Units: WorkUnitBundle` (the WorkUnit cons-list, often built with `read!` / `write!`) and `type Owned: StoreBundle` (the Resource / Column / Virtual cons-list). Pair the impl with `impl BuilderInput for {Self} {{ type Init = Self; const KIND: ProviderKind = ProviderKind::Kit; type Dispatch = KitDispatch<Self>; }}`. The engine reads these at compile time on `.with(my_kit)`. If `.build()` reports `overflow evaluating the requirement` after composing many Kits, declare `#![recursion_limit = \"1024\"]` at your crate root."
)]
pub trait Kit: BuilderInput<Init = Self> {
    /// Cons-list of WorkUnit types the kit contributes. Use the
    /// `read!` / `write!` macros from `hilavitkutin-api` to construct
    /// the shape, or `hilavitkutin_api::Empty` for none.
    type Units: WorkUnitBundle;

    /// Cons-list of Owned store-marker types the kit owns
    /// (`Resource<T>` / `Column<T>` / `Virtual<T>`). Use the same
    /// macros, or `hilavitkutin_api::Empty` for none.
    type Owned: StoreBundle;
}

/// Router for Kit-kind providers. Concats `K::Units` into the WU
/// accumulator and `K::Owned` into the store accumulator; passes
/// the platform-tuple accumulator through.
///
/// The GATs carry where-clauses so the compiler verifies
/// `K::Units: Concat<Wus>` and `K::Owned: Concat<Stores>` at the
/// `.with(kit)` call site, exactly mirroring the round-4 typestate
/// proof.
///
/// `KitDispatch` lives in this crate (not in `hilavitkutin-api`)
/// because its `Dispatch` impl needs to reference `K::Units` and
/// `K::Owned`, which requires `K: Kit`. The api crate cannot depend
/// on kit (the layering is api -> kit). The router struct is still
/// named in the api `Provider` trait surface via its associated
/// type.
pub struct KitDispatch<K>(PhantomData<K>);

impl<K, Wus, Stores, Platform> Dispatch<Wus, Stores, Platform> for KitDispatch<K>
where
    K: Kit,
    <K as Kit>::Units: Concat<Wus>,
    <K as Kit>::Owned: Concat<Stores>,
{
    type NextWus = <<K as Kit>::Units as Concat<Wus>>::Out;
    type NextStores = <<K as Kit>::Owned as Concat<Stores>>::Out;
    type NextPlatform = Platform;
}
