//! Kit preset trait for the hilavitkutin scheduler builder.
//!
//! Round 4 declarative shape: a Kit names what it owns
//! (`type Owned: StoreBundle`) and what work it contributes
//! (`type Units: WorkUnitBundle`). The engine's `.add_kit::<K>()`
//! reads these at compile time. No `install` method, no `B`
//! parameter, no `Output`.

#![no_std]
#![recursion_limit = "512"]

use hilavitkutin_api::{StoreBundle, WorkUnitBundle};

/// A declarative bundle of WorkUnits and Owned stores.
///
/// `Units` names the WorkUnits the kit contributes (Cons-list of
/// WorkUnit types satisfying `WorkUnitBundle`). `Owned` names the
/// Resources / Columns / Virtuals the kit owns (Cons-list of
/// store-marker types satisfying `StoreBundle`).
///
/// Kit authors implement `Kit` by naming `Units` and `Owned` as
/// associated types. The engine's `.add_kit::<K>()` reads these at
/// compile time and accumulates them into the SchedulerBuilder
/// typestate. App-side wiring (default values for Resources,
/// Replaceable opt-in) lives at the call site.
pub trait Kit {
    /// Cons-list of WorkUnit types the kit contributes. Use the
    /// `read!` / `write!` macros from `hilavitkutin-api` to construct
    /// the shape, or `hilavitkutin_api::Empty` for none.
    type Units: WorkUnitBundle;

    /// Cons-list of Owned store-marker types the kit owns
    /// (`Resource<T>` / `Column<T>` / `Virtual<T>`). Use the same
    /// macros, or `hilavitkutin_api::Empty` for none.
    type Owned: StoreBundle;
}
