//! `Identity<M>` and the marker `Additive`, stand-ins for `arvo-numeral`'s.

/// The identity of the operation `M` on `Self`, read as `<T as Identity<Additive>>::IDENTITY`.
pub trait Identity<M> {
    /// The value the operation leaves every other value unchanged by.
    const IDENTITY: Self;
}

/// The marker of addition.
pub struct Additive;
