//! Sketch: a column, plain or buffered, is registered through one carrier of
//! the value its records start from, and the engine's store list takes that
//! carrier in one arm.
//!
//! The sequence of tiers is the one `202611132225_buffered-start-by-type`
//! showed for the buffered marker alone. What is asked here is whether it
//! holds when the second tier is one blanket impl over a sealed trait both
//! markers implement, whether the constructors stay `const`, and whether one
//! arm over `Started<S>` can tell a plain column from a buffered one.

#![feature(const_trait_impl)]

use core::marker::PhantomData;

use notko::HasTrivialCtor;

mod sealed {
    pub trait Sealed {}
}

/// The stores that are registered with a start. Sealed: the two markers.
pub trait Starts: sealed::Sealed {
    type Value;
}

/// The default buffer mode.
pub struct Snapshot;
/// The other buffer mode.
pub struct Exchange;

/// The marker a unit names a plain column by.
pub struct Column<T>(PhantomData<T>);
/// The marker a unit names a buffered column by.
pub struct Buffered<T, M = Snapshot>(PhantomData<(T, M)>);

impl<T> sealed::Sealed for Column<T> {}
impl<T> Starts for Column<T> {
    type Value = T;
}
impl<T, M> sealed::Sealed for Buffered<T, M> {}
impl<T, M> Starts for Buffered<T, M> {
    type Value = T;
}

/// What is registered: the store `S` with the value it starts from.
pub struct Started<S: Starts>(S::Value);

impl<S: Starts> Started<S> {
    /// The start, taken out. Not `const`: see the findings, attempt 1.
    pub fn into_inner(self) -> S::Value {
        self.0
    }
}

impl<T> Column<T> {
    /// The marker, with no word on any start. Always there.
    pub const fn new() -> Self {
        Column(PhantomData)
    }

    /// The explicit start. Always there.
    pub const fn start_from(value: T) -> Started<Self> {
        Started(value)
    }
}

impl<T, M> Default for Buffered<T, M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, M> Buffered<T, M> {
    pub const fn new() -> Self {
        Buffered(PhantomData)
    }

    pub const fn start_from(value: T) -> Started<Self> {
        Started(value)
    }
}

/// The first tier, inherent: the type's trivial constructor.
#[cfg(not(feature = "mutant-no-inherent"))]
impl<T: HasTrivialCtor> Column<T> {
    pub const fn start() -> Started<Self>
    where
        T: [const] HasTrivialCtor,
    {
        Started(<T as HasTrivialCtor>::new())
    }
}

#[cfg(not(feature = "mutant-no-inherent"))]
impl<T: HasTrivialCtor, M> Buffered<T, M> {
    pub const fn start() -> Started<Self>
    where
        T: [const] HasTrivialCtor,
    {
        Started(<T as HasTrivialCtor>::new())
    }
}

/// The second tier, a trait's function: the type's default. One impl for
/// every store that starts, and no impl a consumer can add beside it.
pub trait StartsFromDefault: Starts {
    fn start() -> Started<Self>
    where
        Self: Sized;
}

impl<S: Starts> StartsFromDefault for S
where
    S::Value: Default,
{
    fn start() -> Started<S> {
        Started(<S::Value as Default>::default())
    }
}

// The engine's side: one arm of the store list for both kinds of column.

/// How many reservations a column of this kind has.
pub trait ColKind {
    const SIDES: usize;
}
pub struct Plain;
impl ColKind for Plain {
    const SIDES: usize = 1;
}
impl ColKind for Snapshot {
    const SIDES: usize = 2;
}
impl ColKind for Exchange {
    const SIDES: usize = 2;
}

/// The kind a marker's column is reserved as. The engine's own trait: the
/// api says which stores start, and the engine says how each is kept.
pub trait Kept: Starts {
    type Kind: ColKind;
}
#[cfg(not(feature = "mutant-plain-twice"))]
impl<T> Kept for Column<T> {
    type Kind = Plain;
}
#[cfg(feature = "mutant-plain-twice")]
impl<T> Kept for Column<T> {
    type Kind = Snapshot;
}
impl<T, M: ColKind> Kept for Buffered<T, M> {
    type Kind = M;
}

/// The value list the builder keeps.
pub struct Sv<H, L>(pub H, pub L);
pub struct Nil;

/// Another carrier, beside the column's, as `StagedResource<T>` is.
pub struct Resource<T>(pub T);

/// The drain: how many times each column's start is written, in sequence.
pub trait Drain {
    fn writes(self, out: &mut [usize; 4], at: usize) -> usize;
}
impl Drain for Nil {
    fn writes(self, _out: &mut [usize; 4], at: usize) -> usize {
        at
    }
}
impl<S: Kept, L: Drain> Drain for Sv<Started<S>, L> {
    fn writes(self, out: &mut [usize; 4], at: usize) -> usize {
        let _start: S::Value = self.0.into_inner();
        out[at] = <S::Kind as ColKind>::SIDES;
        self.1.writes(out, at + 1)
    }
}
impl<T, L: Drain> Drain for Sv<Resource<T>, L> {
    fn writes(self, out: &mut [usize; 4], at: usize) -> usize {
        self.1.writes(out, at)
    }
}

/// What the builder's `with` takes. The carrier is one, the bare marker not.
pub trait Input {}
impl<S: Starts> Input for Started<S> {}
impl<T> Input for Resource<T> {}
pub fn with<I: Input>(_input: I) {}

/// Has a trivial constructor and no default.
#[derive(Debug, PartialEq)]
pub struct Built(pub u32);
const impl HasTrivialCtor for Built {
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
const impl HasTrivialCtor for Both {
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

/// The constructor tier and the explicit start are both there at compile
/// time, for both markers.
#[cfg(not(feature = "mutant-no-inherent"))]
pub const PLAIN_AT_COMPILE_TIME: Started<Column<Built>> = Column::<Built>::start();
#[cfg(not(feature = "mutant-no-inherent"))]
pub const BUFFERED_AT_COMPILE_TIME: Started<Buffered<Both, Exchange>> =
    Buffered::<Both, Exchange>::start();
pub const EXPLICIT_AT_COMPILE_TIME: Started<Column<Neither>> =
    Column::<Neither>::start_from(Neither(3));

#[cfg(feature = "refused-no-start")]
pub fn refused() -> Started<Column<Neither>> {
    Column::<Neither>::start()
}

#[cfg(feature = "refused-bare-marker")]
pub fn refused_marker() {
    with(Column::<Built>::new());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(feature = "mutant-no-inherent"))]
    fn a_type_with_a_trivial_constructor_starts_from_it() {
        assert_eq!(Column::<Built>::start().into_inner(), Built(7));
        assert_eq!(Buffered::<Built>::start().into_inner(), Built(7));
    }

    #[test]
    fn a_type_with_a_default_alone_starts_from_it() {
        assert_eq!(Column::<Defaulted>::start().into_inner(), Defaulted(9));
        assert_eq!(
            Buffered::<Defaulted, Exchange>::start().into_inner(),
            Defaulted(9)
        );
    }

    #[test]
    fn a_type_with_both_starts_from_its_constructor() {
        assert_eq!(Column::<Both>::start().into_inner(), Both(7));
        assert_eq!(Buffered::<Both>::start().into_inner(), Both(7));
    }

    #[test]
    fn a_type_with_neither_still_takes_an_explicit_start() {
        assert_eq!(
            Column::<Neither>::start_from(Neither(3)).into_inner(),
            Neither(3)
        );
        assert_eq!(EXPLICIT_AT_COMPILE_TIME.into_inner(), Neither(3));
    }

    #[test]
    fn one_arm_writes_a_plain_column_once_and_a_buffered_one_twice() {
        let list = Sv(
            Column::<Defaulted>::start(),
            Sv(
                Resource(1u8),
                Sv(
                    Buffered::<Both, Exchange>::start_from(Both(1)),
                    Sv(Buffered::<Defaulted>::start(), Nil),
                ),
            ),
        );
        let mut out = [0usize; 4];
        let n = list.writes(&mut out, 0);
        assert_eq!(n, 3);
        assert_eq!(out, [1, 2, 2, 0]);
    }

    #[test]
    fn the_carrier_is_an_input() {
        with(Column::<Defaulted>::start());
        with(Buffered::<Neither>::start_from(Neither(1)));
        with(Resource(1u8));
    }
}
