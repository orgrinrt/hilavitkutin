//! The block form, with a context derived from the unit's declaration.
//!
//! Stands in for an addition beside `hilavitkutin-api`: it names the api and
//! what the api depends on, and no engine crate.
//!
//! A unit in the block form implements `Block`: its read list, its write list,
//! its hint, and `run`, a body written against `Records`, the one trait the
//! context is reached through. The unit names no context type. The context is
//! `Cx<'c, R, W>`, a type of this crate that is a function of the declaration
//! alone, and it carries data: the runner's own context erased to a pointer,
//! and two tables of entry points, one per member of `R` and one per member of
//! `W`, in the order the lists name them. A read or write of a column finds its
//! entry by the column's position in the list, which the solver infers at the
//! call from the list alone. A column the list does not hold has no position,
//! and the call does not compile.
//!
//! The tables are built here, from api traits the runner's context already
//! implements (`ResolveColumnRead`, `ResolveColumnWrite`, `BatchApi`) and the
//! api's own `WritesColumn`, so a runner fills the context by handing over its
//! own and nothing else.

#![no_std]

use core::marker::PhantomData;
use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin_api::access::AccessSet;
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::context::{BatchApi, ResolveColumnRead, ResolveColumnWrite};
use hilavitkutin_api::hint::SchedulingHint;
use hilavitkutin_api::member::{Differs, Written};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::Always;
use hilavitkutin_api::write_at::WritesColumn;
use notko_hlist::{Cons, Empty, Here, Place, PlaceOf, There};

// ---------------------------------------------------------------------
// The unit.
// ---------------------------------------------------------------------

/// A unit of work in the block form.
///
/// The declaration is the read list, the write list and the hint; the body is
/// `run`, handed the records of one window through `Records`. No associated
/// type names a context, so an implementation assigns none.
pub trait Block<S = Always>: Send + Sync + 'static {
    /// The stores the body reads.
    type Read: AccessSet;
    /// The stores the body writes.
    type Write: AccessSet;
    /// The scheduling hint.
    type Hint: SchedulingHint;

    /// Run the body over the records of one window.
    fn run(&self, records: &impl Records<Self::Read, Self::Write>);
}

// ---------------------------------------------------------------------
// Positions.
// ---------------------------------------------------------------------

/// A position in a list, as the number of members before it.
pub trait Slot: Place {
    /// The number of members before the position.
    const AT: usize;
}

impl Slot for Here {
    const AT: usize = 0;
}

impl<P: Slot> Slot for There<P> {
    const AT: usize = P::AT + 1;
}

/// The write list holds a column of `T`, bare, under `Written` or under
/// `Differs`, at position `P`.
#[diagnostic::on_unimplemented(
    message = "the write list `{Self}` holds no column of `{T}`",
    note = "A block unit writes a column it lists in `type Write`, bare, under `Written<_>` or under `Differs<_>`."
)]
pub trait WritesAt<T, P> {}

impl<T, Tail> WritesAt<T, Here> for Cons<Column<T>, Tail> {}
impl<T, Tail> WritesAt<T, Here> for Cons<Written<Column<T>>, Tail> {}
impl<T, Tail> WritesAt<T, Here> for Cons<Differs<Column<T>>, Tail> {}
impl<H, Tail: WritesAt<T, P>, T, P> WritesAt<T, There<P>> for Cons<H, Tail> {}

// ---------------------------------------------------------------------
// What the body is handed.
// ---------------------------------------------------------------------

mod seal {
    pub trait Sealed {}
}

/// The records of one window, as the body of a block unit reaches them.
///
/// Sealed: the derived context `Cx` is the one implementation, so the body is
/// generic in name only, and a read resolves the same way under every runner.
pub trait Records<R, W>: seal::Sealed {
    /// The number of records in the window.
    fn len(&self) -> USize;

    /// Call `f` with each record index of the window, from the first.
    fn each<F: FnMut(USize)>(&self, f: F);

    /// The value of column `T` at record `i` of the window.
    ///
    /// # Safety
    ///
    /// `i` is below `len()`. The plan gave this unit the record.
    unsafe fn read<T: ColumnValue, P: Slot>(&self, i: USize) -> T
    where
        R: PlaceOf<Column<T>, P>;

    /// Write `v` to column `T` at record `i` of the window, by the rule of the
    /// member the write list names it under.
    ///
    /// # Safety
    ///
    /// `i` is below `len()`. The plan gave this unit the exclusive writer slot.
    unsafe fn write<T: ColumnValue, P: Slot>(&self, i: USize, v: T)
    where
        W: WritesAt<T, P>;
}

/// The entry point of one read: the runner's context, a record index, and
/// where to put the value.
pub type ReadFn = unsafe fn(*const (), USize, *mut ());

/// The entry point of one write: the runner's context, a record index, and
/// where the value is.
pub type WriteFn = unsafe fn(*const (), USize, *const ());

/// The context derived from the declaration `(R, W)`.
///
/// `env` is the runner's context, erased. `reads` points at one `ReadFn` per
/// member of `R`, in list order; the entry at a member's position reads that
/// member as its value type. `writes` is the same over `W`. `len` is the
/// record count of the window. Only `enter` builds one, from typed tables, so
/// no value of this type exists whose tables disagree with its lists.
pub struct Cx<'c, R, W> {
    env:    *const (),
    reads:  *const ReadFn,
    writes: *const WriteFn,
    len:    USize,
    _p:     PhantomData<(&'c (), fn() -> (R, W))>,
}

impl<R, W> seal::Sealed for Cx<'_, R, W> {}

impl<R, W> Records<R, W> for Cx<'_, R, W> {
    #[inline(always)]
    fn len(&self) -> USize {
        self.len
    }

    #[inline(always)]
    fn each<F: FnMut(USize)>(&self, mut f: F) {
        let mut i = 0usize;
        while i < self.len.0 {
            f(USize(i));
            i += 1;
        }
    }

    #[inline(always)]
    unsafe fn read<T: ColumnValue, P: Slot>(&self, i: USize) -> T
    where
        R: PlaceOf<Column<T>, P>,
    {
        let mut out = MaybeUninit::<T>::uninit();
        // SAFETY: `R` holds `Column<T>` at `P`, and the entry at `P::AT` of the
        // read table reads that member as `T` into `out` (see `ReadTable`).
        // The table has one entry per member of `R`, so `P::AT` is in it. The
        // caller's obligation on `i` is passed on.
        unsafe {
            let f = *self.reads.add(P::AT);
            f(self.env, i, out.as_mut_ptr().cast());
            out.assume_init()
        }
    }

    #[inline(always)]
    unsafe fn write<T: ColumnValue, P: Slot>(&self, i: USize, v: T)
    where
        W: WritesAt<T, P>,
    {
        // SAFETY: `W` holds a column of `T` at `P`, and the entry at `P::AT` of
        // the write table writes a `T` to that member by its rule (see
        // `WriteTable`). The caller's obligation on `i` is passed on.
        unsafe {
            let f = *self.writes.add(P::AT);
            f(self.env, i, (&v as *const T).cast());
        }
    }
}

// ---------------------------------------------------------------------
// The tables, built from the runner's context type.
// ---------------------------------------------------------------------

/// One entry of a table, followed by the rest.
///
/// `repr(C)` over fields of one type: a table of `n` entries is laid out as
/// `n` entries in a row, so an entry is reached by its position.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Tab<F, Tail> {
    f:    F,
    tail: Tail,
}

/// The end of a table.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct End;

/// Read `T` at `i` through `E`'s column reader, at its witness `I`.
///
/// # Safety
///
/// `env` points at a live `E`; `out` at room for a `T`; the read obligation
/// of `ResolveColumnRead` holds for `i`.
#[inline(always)]
unsafe fn read_thunk<E: ResolveColumnRead<T, I>, T: ColumnValue, I>(
    env: *const (),
    i: USize,
    out: *mut (),
) {
    // SAFETY: the caller's obligations.
    unsafe { out.cast::<T>().write((*env.cast::<E>()).resolve_read(i)) }
}

/// Write the `T` at `v` to record `i` through `E`, by the rule of the member
/// the write list `Root` names the column under.
///
/// # Safety
///
/// `env` points at a live `E`; `v` at a `T`; the write obligation of
/// `ResolveColumnWrite` holds for `i`.
#[inline(always)]
unsafe fn write_thunk<E, Root, T, K, B>(env: *const (), i: USize, v: *const ())
where
    E: ResolveColumnWrite<T, B>,
    Root: WritesColumn<T, K>,
    T: ColumnValue,
{
    // SAFETY: the caller's obligations.
    unsafe { Root::land::<E, B>(&*env.cast::<E>(), i, *v.cast::<T>()) }
}

/// The read table of a list over the runner's context `E`.
///
/// One entry per member, in list order. `Is` carries, per member, the
/// witness `E` resolves that member's column at; it is inferred.
#[diagnostic::on_unimplemented(
    message = "the runner's context `{E}` cannot read every member of `{Self}`",
    note = "A block unit's read list holds bare columns; each is read through the runner's `ResolveColumnRead`."
)]
pub trait ReadTable<E, Is> {
    /// The table's type.
    type Out: Copy;
    /// The table.
    const TABLE: Self::Out;
}

impl<E> ReadTable<E, Empty> for Empty {
    type Out = End;

    const TABLE: End = End;
}

impl<E, T, Tail, I, Is> ReadTable<E, Cons<I, Is>> for Cons<Column<T>, Tail>
where
    E: ResolveColumnRead<T, I>,
    T: ColumnValue,
    Tail: ReadTable<E, Is>,
{
    type Out = Tab<ReadFn, Tail::Out>;

    const TABLE: Self::Out = Tab {
        f:    read_thunk::<E, T, I> as ReadFn,
        tail: Tail::TABLE,
    };
}

/// A member a block unit may write, and the value type it writes.
pub trait WriteMember {
    /// The value type.
    type Value: ColumnValue;
}

impl<T: ColumnValue> WriteMember for Column<T> {
    type Value = T;
}

impl<T: ColumnValue> WriteMember for Written<Column<T>> {
    type Value = T;
}

impl<T: ColumnValue> WriteMember for Differs<Column<T>> {
    type Value = T;
}

/// The write table of the list `Root`, from the sublist `Self` on, over the
/// runner's context `E`.
#[diagnostic::on_unimplemented(
    message = "the runner's context `{E}` cannot write every member of `{Self}`",
    note = "A block unit's write list holds columns, bare or under `Written` or `Differs`; each is written through the api's `WritesColumn` and the runner's `ResolveColumnWrite`."
)]
pub trait WriteTable<Root, E, Is> {
    /// The table's type.
    type Out: Copy;
    /// The table.
    const TABLE: Self::Out;
}

impl<Root, E> WriteTable<Root, E, Empty> for Empty {
    type Out = End;

    const TABLE: End = End;
}

impl<Root, E, M, Tail, K, B, Is> WriteTable<Root, E, Cons<(K, B), Is>> for Cons<M, Tail>
where
    M: WriteMember,
    E: ResolveColumnWrite<M::Value, B>,
    Root: WritesColumn<M::Value, K>,
    Tail: WriteTable<Root, E, Is>,
{
    type Out = Tab<WriteFn, Tail::Out>;

    const TABLE: Self::Out = Tab {
        f:    write_thunk::<E, Root, M::Value, K, B> as WriteFn,
        tail: Tail::TABLE,
    };
}

// ---------------------------------------------------------------------
// Entering a body.
// ---------------------------------------------------------------------

/// Derive the context of `(R, W)` from the runner's context `env` and hand it
/// to `f`.
///
/// The tables live on this call's frame and the context borrows them, so the
/// context cannot outlive the call.
#[inline(always)]
pub fn enter<E, R, W, RI, WI, F>(env: &E, f: F)
where
    E: BatchApi<R, W>,
    R: AccessSet + ReadTable<E, RI>,
    W: AccessSet + WriteTable<W, E, WI>,
    F: FnOnce(&Cx<'_, R, W>),
{
    let reads = <R as ReadTable<E, RI>>::TABLE;
    let writes = <W as WriteTable<W, E, WI>>::TABLE;
    let mut len = USize(0);
    env.run(|_, end| len = end);
    #[cfg(feature = "mutant-short-window")]
    {
        len = USize(len.0.saturating_sub(1));
    }
    let cx = Cx {
        env: (env as *const E).cast(),
        reads: (&reads as *const <R as ReadTable<E, RI>>::Out).cast(),
        writes: (&writes as *const <W as WriteTable<W, E, WI>>::Out).cast(),
        len,
        _p: PhantomData,
    };
    f(&cx);
}
