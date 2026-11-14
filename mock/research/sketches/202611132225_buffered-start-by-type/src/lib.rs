//! Sketch: `Buffered::<T>::start()` means the type's trivial constructor
//! where the type has one, else its default, and does not exist where the
//! type has neither.
//!
//! Two blanket impls of one trait over `T: HasTrivialCtor` and over
//! `T: Default` overlap, and telling them apart by rank is specialisation,
//! which is forbidden. The sequence is had another way: an inherent
//! associated function is looked up before a trait's, and an inherent impl
//! whose bound does not hold is passed over.

use core::marker::PhantomData;

use notko::HasTrivialCtor;

/// The marker a unit names the store by.
pub struct Buffered<T>(PhantomData<T>);

/// What is registered: the marker's store with the value it starts from.
pub struct Staged<T>(pub T);

impl<T> Buffered<T> {
    /// The marker, with no word on any start. Always there.
    pub const fn new() -> Self {
        Buffered(PhantomData)
    }

    /// The explicit start. Always there.
    pub fn start_from(value: T) -> Staged<T> {
        Staged(value)
    }
}

/// The first tier, inherent: the type's trivial constructor.
#[cfg(not(feature = "mutant-no-inherent"))]
impl<T: HasTrivialCtor> Buffered<T> {
    pub fn start() -> Staged<T> {
        Staged(T::new())
    }
}

/// The second tier, a trait's function: the type's default.
pub trait StartsFromDefault {
    type Staged;
    fn start() -> Self::Staged;
}

impl<T: Default> StartsFromDefault for Buffered<T> {
    type Staged = Staged<T>;

    fn start() -> Staged<T> {
        Staged(T::default())
    }
}

/// Has a trivial constructor and no default.
#[derive(Debug, PartialEq)]
pub struct Built(pub u32);
impl HasTrivialCtor for Built {
    fn new() -> Self {
        Built(7)
    }
}

/// Has a default and no trivial constructor.
#[derive(Debug, PartialEq)]
pub struct Defaulted(pub u32);
impl Default for Defaulted {
    fn default() -> Self {
        Defaulted(9)
    }
}

/// Has both, and they differ, so which one ran can be read off the value.
#[derive(Debug, PartialEq)]
pub struct Both(pub u32);
impl HasTrivialCtor for Both {
    fn new() -> Self {
        Both(7)
    }
}
impl Default for Both {
    fn default() -> Self {
        Both(9)
    }
}

/// Has neither.
#[derive(Debug, PartialEq)]
pub struct Neither(pub u32);

/// A generic caller that knows only that `T` has a default.
pub fn knowing_the_default<T: Default>() -> Staged<T> {
    Buffered::<T>::start()
}

/// A generic caller that knows both.
#[cfg(not(feature = "mutant-no-inherent"))]
pub fn knowing_both<T: Default + HasTrivialCtor>() -> Staged<T> {
    Buffered::<T>::start()
}

#[cfg(feature = "refused-no-start")]
pub fn refused() -> Staged<Neither> {
    Buffered::<Neither>::start()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(feature = "mutant-no-inherent"))]
    fn a_type_with_a_trivial_constructor_starts_from_it() {
        assert_eq!(Buffered::<Built>::start().0, Built(7));
    }

    #[test]
    fn a_type_with_a_default_alone_starts_from_it() {
        assert_eq!(Buffered::<Defaulted>::start().0, Defaulted(9));
    }

    #[test]
    fn a_type_with_both_starts_from_its_constructor() {
        assert_eq!(Buffered::<Both>::start().0, Both(7));
    }

    #[test]
    fn a_type_with_neither_still_takes_an_explicit_start() {
        assert_eq!(Buffered::<Neither>::start_from(Neither(3)).0, Neither(3));
    }

    /// The limit: the choice is made where the call is written, from what is
    /// known of the type there.
    #[test]
    fn a_generic_caller_chooses_by_its_own_bounds() {
        assert_eq!(knowing_the_default::<Both>().0, Both(9));
        #[cfg(not(feature = "mutant-no-inherent"))]
        assert_eq!(knowing_both::<Both>().0, Both(7));
    }
}
