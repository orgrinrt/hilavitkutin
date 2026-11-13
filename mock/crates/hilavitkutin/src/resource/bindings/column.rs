//! The column binding, for a plain column and for a buffered one.
//!
//! One node serves both. Its kind says how many reservations stand behind the
//! store and which of them a read and a write reach: a plain column has one
//! and both reach it, a buffered column has a frozen side and a live side.
//! The node also keeps the store's changed mark, which a write sets and the
//! close of the pass reads.

use core::cell::Cell;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, Ordering};

use arvo::{Bool, USize};
use hilavitkutin_api::store::{Buffered, Column, Exchange, Snapshot};
use hilavitkutin_api::{ColumnStorage, ColumnValue, StoreId};

use crate::resource::provenance::ColumnPtr;
use crate::scheduler::BuildError;

mod sealed {
    pub trait Sealed {}
}

/// Reserve `count` records of `T` at the next storage id.
///
/// The one reservation every record-count store makes: a plain column, each
/// side of a buffered column, and an accumulator's capacity buffer. A
/// reservation of no bytes (no records, or a zero-sized `T`) comes back as a
/// dangling, aligned pointer that is never dereferenced.
pub(super) fn reserve_records<T: ColumnValue, CS: ColumnStorage>(
    cs: &mut CS,
    next_id: &mut USize,
    count: USize,
) -> notko::Outcome<ColumnPtr<T>, BuildError> {
    let id = StoreId(*next_id);
    *next_id = USize(next_id.0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal drain-order counter; tracked: #72
    match cs.reserve::<T>(id, count) {
        notko::Outcome::Ok(()) => {},
        notko::Outcome::Err(_) => return notko::Outcome::Err(BuildError::AllocationFailed),
    }
    // SAFETY: `id` names a column just reserved for `count` records of `T`;
    // the store returns its 64-byte-aligned base pointer.
    let typed = unsafe { cs.column_ptr_mut::<T>(id) };
    let ptr = if typed.is_null() {
        // SAFETY: `NonNull::dangling` is non-null and `T`-aligned, and an
        // empty reservation is never read or written.
        unsafe { ColumnPtr::new_unchecked(core::ptr::NonNull::<T>::dangling().as_ptr()) }
    } else {
        // SAFETY: `typed` is non-null (checked), 64-byte aligned, and sized
        // for `count` records (just reserved).
        unsafe { ColumnPtr::new_unchecked(typed) }
    };
    notko::Outcome::Ok(ptr)
}

/// A pointer to a store's changed mark, as a projected context carries it.
///
/// Copied out of the binding when a context is bound, like a `ColumnPtr`, and
/// valid for the frame the binding outlives.
pub struct MarkPtr(*const AtomicBool);

impl Copy for MarkPtr {}
impl Clone for MarkPtr {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

// SAFETY: the mark is an atomic, so setting it from any thread is sound; the
// pointer itself is only a handle to it.
unsafe impl Send for MarkPtr {}
unsafe impl Sync for MarkPtr {}

/// The mark behind a column node built by hand, which no close reads.
static UNREAD: AtomicBool = AtomicBool::new(false);

impl MarkPtr {
    /// A mark nobody reads, for a column node built outside the bindings.
    #[inline(always)]
    pub const fn unread() -> Self {
        MarkPtr(&UNREAD)
    }

    /// Record that the store was changed in this pass.
    #[inline(always)]
    pub fn set(self) {
        // SAFETY: the pointer came from a binding that outlives the frame, or
        // from the static above.
        unsafe { (*self.0).store(true, Ordering::Relaxed) }
    }
}

/// The changed mark of one column store.
///
/// The binding the scheduler holds owns the flag. A per-core copy of the
/// bindings holds a pointer to that same flag, so a write on any core marks
/// the one store.
pub(crate) enum ChangeMark {
    /// The flag itself, in the scheduler's own binding.
    Own(AtomicBool),
    /// The flag of the binding this one was copied from.
    Of(MarkPtr),
}

impl ChangeMark {
    /// A clear mark, for a binding at the drain.
    #[inline]
    pub(crate) const fn clear() -> Self {
        ChangeMark::Own(AtomicBool::new(false))
    }

    /// The pointer a context sets the mark through.
    #[inline(always)]
    pub(crate) fn ptr(&self) -> MarkPtr {
        match self {
            ChangeMark::Own(flag) => MarkPtr(flag),
            ChangeMark::Of(ptr) => *ptr,
        }
    }

    /// A mark for a per-core copy of the binding, pointing at this flag.
    #[inline]
    pub(crate) fn shared(&self) -> Self {
        ChangeMark::Of(self.ptr())
    }

    /// Read the mark and clear it. For the close, with the workers parked.
    #[inline]
    pub(crate) fn take(&self) -> Bool {
        // SAFETY: as `MarkPtr::set`.
        Bool(unsafe { (*self.ptr().0).swap(false, Ordering::Relaxed) })
    }
}

/// How many reservations stand behind a column store and which of them a
/// read and a write reach.
///
/// `Plain` is the kind of a `Column<T>`. The two buffer modes of the api,
/// `Snapshot` and `Exchange`, are the kinds of a `Buffered<T, M>`. Sealed:
/// those three and no other.
#[allow(private_bounds)]
pub trait ColKind: sealed::Sealed + 'static {
    /// What the binding keeps beside its first reservation.
    type Side<T>;

    /// The store marker a column of this kind is registered under, which is
    /// what its position in the store list is found by.
    type Marker<T>;

    /// Whether the store has a frozen side apart from the live one.
    const BUFFERED: Bool;

    /// Reserve what `Side` holds, at the storage ids after the first
    /// reservation's.
    fn reserve_side<T: ColumnValue, CS: ColumnStorage>(
        cs: &mut CS,
        next_id: &mut USize,
        count: USize,
    ) -> notko::Outcome<Self::Side<T>, BuildError>;

    /// Copy `side` for a per-core copy of the binding.
    fn dup_side<T>(side: &Self::Side<T>) -> Self::Side<T>;

    /// The base every read of the pass sees.
    fn frozen<T>(first: ColumnPtr<T>, side: &Self::Side<T>) -> ColumnPtr<T>;

    /// The base every write of the pass lands on.
    fn live<T>(first: ColumnPtr<T>, side: &Self::Side<T>) -> ColumnPtr<T>;

    /// Make the live side the frozen one. The boundary between two passes.
    ///
    /// # Safety
    ///
    /// No context bound from this binding is alive: the last unit of the pass
    /// has returned and the workers are parked. Both sides hold `count`
    /// initialised records.
    unsafe fn bring_in<T>(first: ColumnPtr<T>, side: &Self::Side<T>, count: USize);
}

/// The kind of a plain column: one reservation, read and written in place.
pub struct Plain;

impl sealed::Sealed for Plain {}
impl ColKind for Plain {
    type Marker<T> = Column<T>;
    type Side<T> = PhantomData<T>;

    const BUFFERED: Bool = Bool::FALSE;

    #[inline]
    fn reserve_side<T: ColumnValue, CS: ColumnStorage>(
        _cs: &mut CS,
        _next_id: &mut USize,
        _count: USize,
    ) -> notko::Outcome<PhantomData<T>, BuildError> {
        notko::Outcome::Ok(PhantomData)
    }

    #[inline(always)]
    fn dup_side<T>(_side: &PhantomData<T>) -> PhantomData<T> {
        PhantomData
    }

    #[inline(always)]
    fn frozen<T>(first: ColumnPtr<T>, _side: &PhantomData<T>) -> ColumnPtr<T> {
        first
    }

    #[inline(always)]
    fn live<T>(first: ColumnPtr<T>, _side: &PhantomData<T>) -> ColumnPtr<T> {
        first
    }

    #[inline(always)]
    unsafe fn bring_in<T>(_first: ColumnPtr<T>, _side: &PhantomData<T>, _count: USize) {}
}

// `Snapshot`: the first reservation is the live side for good and the second
// the frozen one. The boundary copies the live side over the frozen.
// FIXME(8e2e2326d588): the copy is of the whole column, where a mark kept per morsel would let it copy the morsels that changed
impl sealed::Sealed for Snapshot {}
impl ColKind for Snapshot {
    type Marker<T> = Buffered<T, Snapshot>;
    type Side<T> = ColumnPtr<T>;

    const BUFFERED: Bool = Bool::TRUE;

    #[inline]
    fn reserve_side<T: ColumnValue, CS: ColumnStorage>(
        cs: &mut CS,
        next_id: &mut USize,
        count: USize,
    ) -> notko::Outcome<ColumnPtr<T>, BuildError> {
        reserve_records::<T, CS>(cs, next_id, count)
    }

    #[inline(always)]
    fn dup_side<T>(side: &ColumnPtr<T>) -> ColumnPtr<T> {
        *side
    }

    #[inline(always)]
    fn frozen<T>(_first: ColumnPtr<T>, side: &ColumnPtr<T>) -> ColumnPtr<T> {
        *side
    }

    #[inline(always)]
    fn live<T>(first: ColumnPtr<T>, _side: &ColumnPtr<T>) -> ColumnPtr<T> {
        first
    }

    #[inline]
    unsafe fn bring_in<T>(first: ColumnPtr<T>, side: &ColumnPtr<T>, count: USize) {
        // SAFETY: two reservations of `count` records each, made apart at the
        // drain, so they do not overlap; the caller holds that nothing reads
        // or writes either while the copy runs.
        unsafe { core::ptr::copy_nonoverlapping(first.as_ptr(), side.as_ptr(), count.0) }
    }
}

/// The second reservation of an `Exchange` column, and which of the two is
/// frozen in the running pass.
pub struct Flip<T> {
    other:  ColumnPtr<T>,
    turned: Cell<Bool>,
}

// `Exchange`: the two reservations trade places at the boundary. Nothing is
// copied and no base moves; the index says which one a read reaches, and a
// context reads it when it is bound.
impl sealed::Sealed for Exchange {}
impl ColKind for Exchange {
    type Marker<T> = Buffered<T, Exchange>;
    type Side<T> = Flip<T>;

    const BUFFERED: Bool = Bool::TRUE;

    #[inline]
    fn reserve_side<T: ColumnValue, CS: ColumnStorage>(
        cs: &mut CS,
        next_id: &mut USize,
        count: USize,
    ) -> notko::Outcome<Flip<T>, BuildError> {
        match reserve_records::<T, CS>(cs, next_id, count) {
            notko::Outcome::Ok(other) => {
                notko::Outcome::Ok(Flip {
                    other,
                    turned: Cell::new(Bool::FALSE),
                })
            },
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }

    #[inline(always)]
    fn dup_side<T>(side: &Flip<T>) -> Flip<T> {
        Flip {
            other:  side.other,
            turned: Cell::new(side.turned.get()),
        }
    }

    #[inline(always)]
    fn frozen<T>(first: ColumnPtr<T>, side: &Flip<T>) -> ColumnPtr<T> {
        if side.turned.get().0 { first } else { side.other }
    }

    #[inline(always)]
    fn live<T>(first: ColumnPtr<T>, side: &Flip<T>) -> ColumnPtr<T> {
        if side.turned.get().0 { side.other } else { first }
    }

    #[inline]
    unsafe fn bring_in<T>(_first: ColumnPtr<T>, side: &Flip<T>, _count: USize) {
        side.turned.set(Bool(!side.turned.get().0));
    }
}

/// Bindings cons-cell for one registered `Column<T>` or `Buffered<T, M>`.
///
/// Holds the first reservation's base (sized by the build-time record count),
/// what the kind `K` keeps beside it, the record count and the store's
/// changed mark. A zero-record (or zero-sized) column records a dangling,
/// well-aligned pointer, never dereferenced since the morsel is empty.
///
/// `pos` is the store's position in the scheduler's store list, the bit the
/// access masks and the dirty seed know it by. The drain leaves it at zero and
/// `build` sets it through `PlaceIn`, since a store with no value of its own
/// (one a kit owns) has a position in that list and no node here, so a node's
/// place along the bindings does not give it.
pub struct ColumnBinding<T, Tail, K: ColKind = Plain> {
    pub(crate) ptr:   ColumnPtr<T>,
    pub(crate) side:  K::Side<T>,
    pub(crate) count: USize,
    pub(crate) mark:  ChangeMark,
    pub(crate) pos:   USize,
    pub(crate) tail:  Tail,
}

impl<T, Tail, K: ColKind> ColumnBinding<T, Tail, K> {
    /// The first reservation's base pointer: the column itself for a plain
    /// column. Hidden accessor used by tests. Not part of the supported
    /// surface.
    #[doc(hidden)]
    pub fn __ptr(&self) -> ColumnPtr<T> {
        self.ptr
    }

    /// The base a read of the running pass sees. Hidden accessor used by the
    /// column `ColSelector` and by tests. Not supported surface.
    #[doc(hidden)]
    pub fn __frozen(&self) -> ColumnPtr<T> {
        K::frozen(self.ptr, &self.side)
    }

    /// The base a write of the running pass lands on. Hidden accessor used by
    /// the column `ColSelector` and by tests. Not supported surface.
    #[doc(hidden)]
    pub fn __live(&self) -> ColumnPtr<T> {
        K::live(self.ptr, &self.side)
    }

    /// The reserved record count. Hidden test accessor; not supported surface.
    #[doc(hidden)]
    pub fn __count(&self) -> USize {
        self.count
    }

    /// The pointer a context sets the store's changed mark through. Hidden
    /// accessor used by the column `ColSelector`. Not supported surface.
    #[doc(hidden)]
    pub fn __mark(&self) -> MarkPtr {
        self.mark.ptr()
    }

    /// The store's position in the scheduler's store list. Hidden test
    /// accessor; not supported surface.
    #[doc(hidden)]
    pub fn __pos(&self) -> USize {
        self.pos
    }

    /// The tail node. Hidden accessor used by the pass-through selectors
    /// and by tests. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __tail(&self) -> &Tail {
        &self.tail
    }
}

/// Drain one column store: the first reservation, then what its kind keeps.
pub(super) fn drain_column<T: ColumnValue, K: ColKind, CS: ColumnStorage, Tail>(
    cs: &mut CS,
    next_id: &mut USize,
    record_count: USize,
    tail: impl FnOnce(&mut CS, &mut USize) -> notko::Outcome<Tail, BuildError>,
) -> notko::Outcome<ColumnBinding<T, Tail, K>, BuildError> {
    // Records are written by producer units during the frame; the drain
    // reserves, it does not initialise.
    let ptr = match reserve_records::<T, CS>(cs, next_id, record_count) {
        notko::Outcome::Ok(ptr) => ptr,
        notko::Outcome::Err(e) => return notko::Outcome::Err(e),
    };
    let side = match K::reserve_side::<T, CS>(cs, next_id, record_count) {
        notko::Outcome::Ok(side) => side,
        notko::Outcome::Err(e) => return notko::Outcome::Err(e),
    };
    match tail(cs, next_id) {
        notko::Outcome::Ok(tail) => {
            notko::Outcome::Ok(ColumnBinding {
                ptr,
                side,
                count: record_count,
                mark: ChangeMark::clear(),
                pos: USize(0), // lint:allow(no-bare-numeric) reason: unplaced store position, set by PlaceIn at build; tracked: #121
                tail,
            })
        },
        notko::Outcome::Err(e) => notko::Outcome::Err(e),
    }
}
