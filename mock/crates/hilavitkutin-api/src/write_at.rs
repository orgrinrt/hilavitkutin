//! The positional write, resolved over the write list and the context.
//!
//! `write` reaches a column the unit lists bare, under `Written` or under
//! `Differs`, and does what the member says. `WritesColumn` finds the column
//! of `T` in the write list and carries the member's rule. `WriteAt` holds
//! for the list and the context together, so the accessor's one inferred
//! index covers both.

use arvo::USize;
use notko_hlist::Cons;

use crate::access::AccessSet;
use crate::column_value::ColumnValue;
use crate::context::ResolveColumnWrite;
use crate::member::{Differs, Member, Written};
use crate::store::Column;

/// Index: the head of the write list is the bare `Column<T>`.
pub struct AtBare;
/// Index: the head of the write list is `Written<Column<T>>`.
pub struct AtWritten;
/// Index: the head of the write list is `Differs<Column<T>>`.
pub struct AtDiffers;
/// Index: the column is in the tail, at `K`.
pub struct Next<K>(core::marker::PhantomData<K>);

/// The write list holds a column of `T` under one of the three write
/// members, at the inferred index `K`.
///
/// A bare `Column<T>` and `Written<Column<T>>` store the value and count the
/// store as changed. `Differs<Column<T>>` compares the value with the one
/// the record holds, and neither stores nor counts an equal one.
#[diagnostic::on_unimplemented(
    message = "the write list `{Self}` holds no column of `{T}` to write by position",
    note = "The positional `write` reaches a `Column<T>` the unit lists in `type Write` bare, under `Written<_>` or under `Differs<_>`. A `Buffered<T, M>` is written through the value function alone, and an `Accum<T>` by `append`."
)]
pub trait WritesColumn<T: ColumnValue, K>: AccessSet {
    /// Land `v` at record `i` through `ctx`, by the member's rule.
    ///
    /// # Safety
    ///
    /// As `ResolveColumnWrite::resolve_write`: `i` is below the record
    /// count of the window and the plan gave this unit the record.
    unsafe fn land<C: ResolveColumnWrite<T, B> + ?Sized, B>(ctx: &C, i: USize, v: T);
}

#[diagnostic::do_not_recommend]
impl<T: ColumnValue, Tail: AccessSet> WritesColumn<T, AtBare> for Cons<Column<T>, Tail> {
    // FIXME(placeholders_tagged_29cdc662ca14): a bare column under the positional write is taken as under `Written`, pending the bench of both
    #[inline(always)]
    unsafe fn land<C: ResolveColumnWrite<T, B> + ?Sized, B>(ctx: &C, i: USize, v: T) {
        // SAFETY: the caller's obligation, passed on unchanged.
        unsafe { ctx.resolve_write(i, v) }
    }
}

#[diagnostic::do_not_recommend]
impl<T: ColumnValue, Tail: AccessSet> WritesColumn<T, AtWritten>
    for Cons<Written<Column<T>>, Tail>
{
    #[inline(always)]
    unsafe fn land<C: ResolveColumnWrite<T, B> + ?Sized, B>(ctx: &C, i: USize, v: T) {
        // SAFETY: the caller's obligation, passed on unchanged.
        unsafe { ctx.resolve_write(i, v) }
    }
}

#[diagnostic::do_not_recommend]
impl<T: ColumnValue + PartialEq, Tail: AccessSet> WritesColumn<T, AtDiffers>
    for Cons<Differs<Column<T>>, Tail>
{
    #[inline(always)]
    unsafe fn land<C: ResolveColumnWrite<T, B> + ?Sized, B>(ctx: &C, i: USize, v: T) {
        // SAFETY: the caller's obligation covers the read of the record
        // it is about to write as well as the write.
        unsafe {
            if ctx.resolve_held(i) != v {
                ctx.resolve_write(i, v);
            }
        }
    }
}

#[diagnostic::do_not_recommend]
impl<H: Member, T: ColumnValue, K, Tail: WritesColumn<T, K>> WritesColumn<T, Next<K>>
    for Cons<H, Tail>
{
    #[inline(always)]
    unsafe fn land<C: ResolveColumnWrite<T, B> + ?Sized, B>(ctx: &C, i: USize, v: T) {
        // SAFETY: the caller's obligation, passed on unchanged.
        unsafe { Tail::land::<C, B>(ctx, i, v) }
    }
}

/// A write list and a context together can write a column of `T`.
///
/// Holds for `(W, C)` where the list `W` holds a column of `T` under a
/// write member (`WritesColumn<T, K>`) and the context `C` resolves it
/// (`ResolveColumnWrite<T, B>`). `I` is the pair `(K, B)` and is inferred as
/// one, so a call names the value type and nothing else. `Ctx` is the
/// pair's context half, which the accessor's bound holds equal to itself.
#[diagnostic::on_unimplemented(
    message = "a column of `{T}` cannot be written by position here",
    note = "The positional `write` reaches a `Column<T>` the unit lists in `type Write` bare, under `Written<_>` or under `Differs<_>`. A `Buffered<T, M>` is written through the value function alone."
)]
pub trait WriteAt<T: ColumnValue, I> {
    /// The context half of the pair.
    type Ctx: ?Sized;

    /// Write `v` at record `i` through `ctx`, by the member's rule.
    ///
    /// # Safety
    ///
    /// As `ColumnWriterApi::write`.
    unsafe fn write_at(ctx: &Self::Ctx, i: USize, v: T);
}

#[diagnostic::do_not_recommend]
impl<W, C: ?Sized, T: ColumnValue, K, B> WriteAt<T, (K, B)> for (W, C)
where
    W: WritesColumn<T, K>,
    C: ResolveColumnWrite<T, B>,
{
    type Ctx = C;

    #[inline(always)]
    unsafe fn write_at(ctx: &C, i: USize, v: T) {
        // SAFETY: the caller's obligation, passed on unchanged.
        unsafe { W::land::<C, B>(ctx, i, v) }
    }
}
