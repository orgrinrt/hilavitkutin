//! Provider kits: reusable Resource, Column, and projection bundles
//! that self-install onto the scheduler builder via the same chainable
//! API consumers use directly.
//!
//! A Kit is a concrete type (e.g. `InternerKit<M, CAP>`) shipped by a
//! provider crate. Consumers construct a Kit and pass it to
//! `.install()`. Kit authors chain `.resource::<T>(...)`,
//! `.column::<T>()`, and `.install(other)` inside the trait method
//! body; no parallel registration API exists.
//!
//! The trait is parameterised on the builder type so this crate stays
//! free of any dependency on `hilavitkutin`. The engine crate ships
//! the call site and the builder-specific `.install` sugar.

/// A kit that registers itself onto a scheduler builder of type `B`.
///
/// Implementors return the mutated builder via `Out`. The Out type
/// reflects whatever Demand and Supply advancement the kit's chained
/// calls produce; for callers, the interesting thing is that the Out
/// is itself a builder ready for the next chain step.
pub trait ProviderKit<B> {
    /// The mutated builder returned after the kit's registrations.
    type Out;

    /// Apply the kit's registrations to `builder` and return the
    /// mutated builder.
    fn install(self, builder: B) -> Self::Out;
}
