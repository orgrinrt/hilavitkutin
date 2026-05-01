//! Kit preset trait for the hilavitkutin scheduler builder.
//!
//! `no_std`, zero deps. Ships exactly one trait, [`Kit<B>`], that
//! lets a consumer bundle a related set of `Resource<T>` /
//! `Column<T>` / `Virtual<T>` registrations into a single
//! `.add_kit(k)` call on the engine's `SchedulerBuilder`.

#![no_std]
#![recursion_limit = "512"]

/// A preset that bundles a set of registrations onto a builder.
///
/// `B` is the input builder type. `Self::Output` is the resulting
/// builder type after the Kit's contributions have been applied.
/// The implementation calls the builder's existing registration
/// methods; the type-state evolves through whatever mechanism the
/// builder already uses (engine-side, per
/// `hilavitkutin/DESIGN.md`, the builder is parameterised by `Wus`
/// and `Stores` tuples).
///
/// A Kit's `install` body is typically a chain like
/// `builder.resource(Foo).column::<Bar>()`. Nothing Kit-specific
/// lives at the type level; the bundle is just a name plus a
/// fixed sequence of builder calls.
pub trait Kit<B> {
    /// Builder type produced after `install` runs.
    type Output;

    /// Apply the Kit's registrations to `builder`.
    fn install(self, builder: B) -> Self::Output;
}
