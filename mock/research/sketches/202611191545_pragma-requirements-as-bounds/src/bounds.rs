//! Route one: the requirements are trait bounds over the cons list.

use core::marker::PhantomData;

use crate::{
    BackendGroup,
    Bolt,
    Cranelift,
    FastMath,
    Free,
    LinkerGroup,
    MathPeephole,
    Mold,
    Pgo,
    Pragma,
    Profiling,
    Wild,
};

/// What a pragma asks of the set it sits in, as lists of pragma types.
pub trait Demands: Pragma {
    /// Every one of these is in the set.
    type Needs;
    /// One of these is in the set. `Nothing` asks for none.
    type OneOf;
}

/// The `OneOf` of a pragma that asks for none.
pub struct Nothing;

impl Demands for FastMath {
    type Needs = ();
    type OneOf = Nothing;
}
impl Demands for MathPeephole {
    type Needs = (FastMath, ());
    type OneOf = Nothing;
}
impl Demands for Pgo {
    type Needs = ();
    type OneOf = Nothing;
}
impl Demands for Bolt {
    type Needs = ();
    type OneOf = Nothing;
}
impl Demands for Profiling {
    type Needs = ();
    type OneOf = (Pgo, (Bolt, ()));
}
impl Demands for Mold {
    type Needs = ();
    type OneOf = Nothing;
}
impl Demands for Wild {
    type Needs = ();
    type OneOf = Nothing;
}
impl Demands for Cranelift {
    type Needs = ();
    type OneOf = Nothing;
}

// Membership. The position is a type the compiler infers, since a list
// cannot be asked whether its head equals a type without one.

pub struct Here;
pub struct There<I>(PhantomData<I>);

#[diagnostic::on_unimplemented(
    message = "the pragma set does not hold `{P}`",
    label = "a pragma in this set stands on `{P}`"
)]
pub trait Has<P, I> {}
impl<P, T> Has<P, Here> for (P, T) {}
impl<P, H, T, I> Has<P, There<I>> for (H, T) where T: Has<P, I> {}

/// Every member of the list `L` is in the set.
pub trait HasAll<L, I> {}
impl<S> HasAll<(), ()> for S {}
impl<S, H, T, IH, IT> HasAll<(H, T), (IH, IT)> for S
where
    S: Has<H, IH>,
    S: HasAll<T, IT>,
{
}

pub struct NoneAsked;
pub struct First<I>(PhantomData<I>);
pub struct Later<I>(PhantomData<I>);

/// One member of the list `L` is in the set.
#[diagnostic::on_unimplemented(
    message = "the pragma set holds none of `{L}`",
    label = "a pragma in this set stands on one of `{L}`"
)]
pub trait HasAny<L, I> {}
impl<S> HasAny<Nothing, NoneAsked> for S {}
impl<S, H, T, I> HasAny<(H, T), First<I>> for S where S: Has<H, I> {}
impl<S, H, T, I> HasAny<(H, T), Later<I>> for S where S: HasAny<T, I> {}

/// Every member of the list has what it asks of the set `S`.
pub trait Satisfied<S, I> {}
impl<S> Satisfied<S, ()> for () {}
impl<S, H, T, IA, IO, IT> Satisfied<S, (IA, IO, IT)> for (H, T)
where
    H: Demands,
    S: HasAll<H::Needs, IA>,
    S: HasAny<H::OneOf, IO>,
    T: Satisfied<S, IT>,
{
}

// Exclusion. Two pragmas may share a set when their groups differ, or
// when one of them is in no group. The pairs are written out, since a
// bound cannot say two types differ.

#[diagnostic::on_unimplemented(
    message = "the pragma set holds two pragmas of one group: `{Self}`",
    label = "a set holds one pragma of a group"
)]
pub trait Distinct {}
impl<G> Distinct for (Free, G) {}
impl Distinct for (LinkerGroup, Free) {}
impl Distinct for (LinkerGroup, BackendGroup) {}
impl Distinct for (BackendGroup, Free) {}
impl Distinct for (BackendGroup, LinkerGroup) {}

/// The pragma shares a group with no member of the list `L`.
pub trait Apart<L> {}
impl<P> Apart<()> for P {}
impl<P, H, T> Apart<(H, T)> for P
where
    P: Pragma,
    H: Pragma,
    (P::Group, H::Group): Distinct,
    P: Apart<T>,
{
}

/// The list holds one pragma of a group at most.
pub trait OnePerGroup {}
impl OnePerGroup for () {}
impl<H, T> OnePerGroup for (H, T)
where
    H: Apart<T>,
    T: OnePerGroup,
{
}

/// A set every requirement holds of.
pub trait Coherent<I> {}
impl<S, I> Coherent<I> for S where S: OnePerGroup + Satisfied<S, I> {}

/// Writes every member's text, a space between.
pub trait WriteAll {
    fn write_all<W: core::fmt::Write>(w: &mut W) -> core::fmt::Result;
}
impl WriteAll for () {
    fn write_all<W: core::fmt::Write>(_: &mut W) -> core::fmt::Result {
        Ok(())
    }
}
impl<H: Pragma, T: WriteAll> WriteAll for (H, T) {
    fn write_all<W: core::fmt::Write>(w: &mut W) -> core::fmt::Result {
        H::write(w)?;
        w.write_str(" ")?;
        T::write_all(w)
    }
}

/// What a build script would call: it compiles for a coherent set alone.
pub fn configure<S, I, W>(w: &mut W) -> core::fmt::Result
where
    S: Coherent<I> + WriteAll,
    W: core::fmt::Write,
{
    S::write_all(w)
}
