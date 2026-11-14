//! The value function's two products: what a body reads and what it returns.
//!
//! A unit's body written as a function takes the values of its read list at
//! one record and returns the values of its write list at that record.
//! `Readable` and `Writable` say what value a member gives in each list,
//! `ReadSet` and `WriteSet` hold for the lists made of such members, and
//! `ReadVals` and `WriteVals` are the two products.
//!
//! The write product is the shared list crate's `Vals`, built by `rec!`. The
//! read product is this crate's own, because a value read may borrow (a
//! resource read by reference, a `Window` on a column) and the shared
//! crate's product holds `'static` values only.
// FIXME(placeholders_tagged_e86353ea1157): the read product moves to the shared list crate once its product can hold a borrowed value

use arvo::{Bool, Cap};
use notko::Maybe;
use notko_hlist::{Cons, Elem, Empty, Project};
pub use notko_hlist::{Here, PCons, PNil, Place, PlaceOf, There, Vals};

use crate::access::AccessSet;
use crate::column_value::ColumnValue;
use crate::member::{Differs, Member, Reach, Window, Written};
use crate::store::{Accum, BufferMode, Buffered, Column, Resource, Virtual};

/// A member a value function can read.
///
/// `Val<'a>` is what the function is handed for the member at one record.
/// A column and a buffered column give their `T`, either under a `Reach`
/// gives a `Window`, and a resource gives what its value type's
/// `ResourceRead` says. An accumulator, a virtual and a change-rule wrapper
/// are not read by a function.
#[diagnostic::on_unimplemented(
    message = "a value function cannot read `{Self}`",
    note = "A function reads a `Column<T>`, a `Buffered<T, M>`, either of them under `Reach<_, N>`, and a `Resource<T>` whose `T` implements `ResourceRead` (the `ResourceFootprint` derive writes it). An accumulator is read by a maintenance unit through `appended()`, and a resource with no `ResourceRead` through `resources()` in a positional body."
)]
pub trait Readable: Member {
    /// What a function reads of this member at one record.
    type Val<'a>: Copy;
}

/// A member a value function can return a value for.
///
/// `Val` is what the function returns for the member. A column and a
/// buffered column take their `T`, bare or under a change rule. A bare one
/// is taken as under `Differs`, so its `T` is `PartialEq`, and a value type
/// with no comparison is listed under `Written`. An accumulator takes a
/// `Maybe<T>` (append the value, or append nothing) and a virtual takes a
/// `Bool` (fire, or do not).
#[diagnostic::on_unimplemented(
    message = "a value function cannot return a value for `{Self}`",
    note = "A function writes a `Column<T>` or a `Buffered<T, M>` (bare where `T: PartialEq`, else under `Written<_>`), an `Accum<T>` and a `Virtual<V>`. A resource is not written by a function."
)]
pub trait Writable: Member {
    /// What a function returns for this member.
    type Val;
}

/// The form a resource value type is read in by a value function.
///
/// A value type whose fields are all scalars is read by value, and one that
/// holds a `Seq` or a `Map` is read by reference for the length of one
/// application. The `ResourceFootprint` derive writes the implementation
/// from the same fields it sizes.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not say how a value function reads it",
    note = "Derive `ResourceFootprint` on the resource value type, which writes `ResourceRead` from the same fields, or implement it by hand: `type Val<'a> = Self` for a `Copy` value read by value, `type Val<'a> = &'a Self` for one read by reference."
)]
pub trait ResourceRead: 'static {
    /// `Self` for a resource of scalar fields, `&'a Self` for one that holds
    /// a collection.
    type Val<'a>: Copy
    where
        Self: 'a;

    /// The value a function is handed, formed from the resource in place.
    fn read<'a>(&'a self) -> Self::Val<'a>;
}

impl<T: ColumnValue> Readable for Column<T> {
    type Val<'a> = T;
}

impl<T: ColumnValue, M: BufferMode> Readable for Buffered<T, M> {
    type Val<'a> = T;
}

impl<T: ColumnValue, const N: Cap> Readable for Reach<Column<T>, N> {
    type Val<'a> = Window<'a, T, N>;
}

impl<T: ColumnValue, M: BufferMode, const N: Cap> Readable for Reach<Buffered<T, M>, N> {
    type Val<'a> = Window<'a, T, N>;
}

impl<T: ResourceRead> Readable for Resource<T> {
    type Val<'a> = <T as ResourceRead>::Val<'a>;
}

impl<T: ColumnValue + PartialEq> Writable for Column<T> {
    type Val = T;
}

impl<T: ColumnValue + PartialEq, M: BufferMode> Writable for Buffered<T, M> {
    type Val = T;
}

impl<T: ColumnValue> Writable for Written<Column<T>> {
    type Val = T;
}

impl<T: ColumnValue, M: BufferMode> Writable for Written<Buffered<T, M>> {
    type Val = T;
}

impl<T: ColumnValue + PartialEq> Writable for Differs<Column<T>> {
    type Val = T;
}

impl<T: ColumnValue + PartialEq, M: BufferMode> Writable for Differs<Buffered<T, M>> {
    type Val = T;
}

impl<T: ColumnValue> Writable for Accum<T> {
    type Val = Maybe<T>;
}

impl<V: 'static> Writable for Virtual<V> {
    type Val = Bool;
}

// The shared product holds, for each member, the value type the member
// names through `Elem`. These are the write members' values once more, in
// the shared crate's terms, and `WriteSet` below holds the two equal.

impl<T: ColumnValue> Elem for Column<T> {
    type Value = T;
}

impl<T: ColumnValue, M: BufferMode> Elem for Buffered<T, M> {
    type Value = T;
}

impl<T: ColumnValue> Elem for Written<Column<T>> {
    type Value = T;
}

impl<T: ColumnValue, M: BufferMode> Elem for Written<Buffered<T, M>> {
    type Value = T;
}

impl<T: ColumnValue + PartialEq> Elem for Differs<Column<T>> {
    type Value = T;
}

impl<T: ColumnValue + PartialEq, M: BufferMode> Elem for Differs<Buffered<T, M>> {
    type Value = T;
}

impl<T: ColumnValue> Elem for Accum<T> {
    type Value = Maybe<T>;
}

impl<V: 'static> Elem for Virtual<V> {
    type Value = Bool;
}

/// The end of a read product.
#[derive(Clone, Copy)]
pub struct RNil;

/// A cell of a read product: the head member's value, then the rest.
///
/// The fields are public so that a runner builds one per record.
#[derive(Clone, Copy)]
pub struct RCons<V, T> {
    /// What the function reads of the head member.
    pub head: V,
    /// The product of the tail.
    pub tail: T,
}

/// A read list a value function can take: every member is `Readable`.
#[diagnostic::on_unimplemented(
    message = "a value function cannot read the list `{Self}`",
    note = "Every member of the read list of a unit that calls `map` is `Readable`: a `Column`, a `Buffered`, either under a `Reach`, or a `Resource<T>` whose `T` implements `ResourceRead`."
)]
pub trait ReadSet: AccessSet {
    /// The product of the members' values at one record: `RCons` per cell,
    /// `RNil` at the end.
    type Vals<'a>: Copy;
}

#[diagnostic::do_not_recommend]
impl ReadSet for Empty {
    type Vals<'a> = RNil;
}

#[diagnostic::do_not_recommend]
impl<H: Readable, T: ReadSet> ReadSet for Cons<H, T> {
    type Vals<'a> = RCons<H::Val<'a>, T::Vals<'a>>;
}

/// A write list a value function can return: every member is `Writable`.
#[diagnostic::on_unimplemented(
    message = "a value function cannot return the list `{Self}`",
    note = "Every member of the write list of a unit that calls `map` is `Writable`: a `Column<T>` or a `Buffered<T, M>` (bare where `T: PartialEq`, else under `Written<_>`), an `Accum<T>` or a `Virtual<V>`."
)]
pub trait WriteSet: AccessSet + Project {}

#[diagnostic::do_not_recommend]
impl WriteSet for Empty {}

#[diagnostic::do_not_recommend]
impl<H, T: WriteSet> WriteSet for Cons<H, T> where H: Writable + Elem<Value = <H as Writable>::Val> {}

/// The values of a unit's read list at one record.
///
/// A read is `r.get(MEMBER)`, with the member passed as a value and its
/// place in the list inferred from it. The bound is the read list holding
/// the member, so a member the unit did not declare does not compile.
pub struct ReadVals<'a, R: ReadSet>(pub R::Vals<'a>);

/// The values a function returns for a unit's write list at one record.
///
/// The shared list crate's product of values, built in the list's sequence
/// by `rec!`.
pub type WriteVals<W> = Vals<W>;

/// The read list holds the readable member `M` at the place `P`.
///
/// What `ReadVals::get` is bounded on. Closed the way the shared crate's
/// `Has` is: the supertrait `PlaceOf` already answers every place a member
/// can sit at, and the two impls below cover each.
#[diagnostic::on_unimplemented(
    message = "the read list `{Self}` has no value for `{M}`",
    note = "A value function reads the members its unit declared in `type Read`, each named as it is written there (a `Reach` is read as the `Reach`). If the compiler reports `type annotations needed` instead, the list names the member twice."
)]
pub trait ReadAt<M: Readable, P: Place>: ReadSet + PlaceOf<M, P> {
    /// The value of `M` in `vals`.
    fn at<'a>(vals: &Self::Vals<'a>) -> M::Val<'a>;
}

#[diagnostic::do_not_recommend]
impl<M: Readable, T: ReadSet> ReadAt<M, Here> for Cons<M, T> {
    #[inline(always)]
    fn at<'a>(vals: &Self::Vals<'a>) -> M::Val<'a> {
        vals.head
    }
}

#[diagnostic::do_not_recommend]
impl<H: Readable, M: Readable, P: Place, T: ReadAt<M, P>> ReadAt<M, There<P>> for Cons<H, T> {
    #[inline(always)]
    fn at<'a>(vals: &Self::Vals<'a>) -> M::Val<'a> {
        T::at(&vals.tail)
    }
}

impl<'a, R: ReadSet> ReadVals<'a, R> {
    /// The value of `member` at this record.
    #[inline(always)]
    pub fn get<M: Readable, P: Place>(&self, member: M) -> M::Val<'a>
    where
        R: ReadAt<M, P>,
    {
        // The marker says which member and nothing else.
        core::mem::forget(member);
        R::at(&self.0)
    }
}
