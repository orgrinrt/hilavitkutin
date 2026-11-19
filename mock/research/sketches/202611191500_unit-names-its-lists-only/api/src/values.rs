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

// Sketch edit (202611191500): the read product is the product of the list
// through a carrier, in the shape notko designs for `notko-hlist`
// (orgrinrt/notko `docs/design/notko-hlist-product-carrier.md` at `69915c8`,
// built here against a copy of that design's model crate). The shipped
// `RNil`, `RCons`, `ReadSet::Vals` and `ReadAt` are gone: the cells are the
// model's `PNil` and `PCons`, and a member is reached through its `Has`.

/// The carrier of a read product: what a value function reads of each member
/// for the frame `'f`.
///
/// The read list names no lifetime. The frame sits here, on the carrier, and
/// the carrier states one form per member: what `Readable` says the member
/// gives.
pub struct Read<'f>(core::marker::PhantomData<&'f ()>);

impl<'f, M: Readable> model::Carrier<M> for Read<'f> {
    type Of = M::Val<'f>;
}

/// The product of the read list `R` for the frame `'a`: the model's `PCons`
/// per member through `Read<'a>`, `PNil` at the end.
pub type ReadProd<'a, R> = <R as model::Project<Read<'a>>>::Prod;

/// A read list a value function can take: every member is `Readable`, so the
/// list has a product through `Read` for every frame.
#[diagnostic::on_unimplemented(
    message = "a value function cannot read the list `{Self}`",
    note = "Every member of the read list of a unit that calls `map` is `Readable`: a `Column`, a `Buffered`, either under a `Reach`, or a `Resource<T>` whose `T` implements `ResourceRead`."
)]
pub trait ReadSet: AccessSet + for<'a> model::Project<Read<'a>> {}

#[diagnostic::do_not_recommend]
impl ReadSet for Empty {}

#[diagnostic::do_not_recommend]
impl<H: Readable, T: ReadSet> ReadSet for Cons<H, T> {}

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
///
/// Sketch edit (202611191500): bounded on the list having a product for this
/// one frame, where the shipped struct was bounded on `ReadSet`. A read set
/// has a product for every frame, and that bound beside the one `Has` brings
/// for the frame `'a` is refused as ambiguous (attempt c1).
pub struct ReadVals<'a, R: model::Project<Read<'a>>>(pub ReadProd<'a, R>);

/// The values a function returns for a unit's write list at one record.
///
/// The shared list crate's product of values, built in the list's sequence
/// by `rec!`.
pub type WriteVals<W> = Vals<W>;

impl<'a, R: model::Project<Read<'a>>> ReadVals<'a, R> {
    /// The value of `member` at this record.
    ///
    /// Sketch edit (202611191500): bounded on the model's `Has` through
    /// `Read<'a>`, where the shipped method was bounded on this crate's own
    /// `ReadAt`.
    #[inline(always)]
    pub fn get<M: Readable, P: Place>(&self, member: M) -> M::Val<'a>
    where
        R: model::Has<Read<'a>, M, P>,
    {
        // The marker says which member and nothing else.
        core::mem::forget(member);
        *<R as model::Has<Read<'a>, M, P>>::pick(&self.0)
    }
}
