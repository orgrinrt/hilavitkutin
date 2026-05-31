//! Resource arena: the data-plane storage built from registered store
//! values, reserved through the unified `ColumnStorage`.
//!
//! The arena is keyed on the builder's `StoreValues` list, not on the
//! `Stores` access set. Each `.with`-registered store value contributes
//! one arena node: a `Resource<T>` carrier (`StagedResource<T>`)
//! contributes an `ArenaResourceNode<T, _>` holding the moved-in value's
//! `ResourcePtr<T>`; a `Column<T>` marker contributes an
//! `ArenaColumnNode` (a no-alloc placeholder, since column buffers need
//! the plan-phase record count); a `Virtual<T>` marker contributes an
//! `ArenaVirtualNode` (no backing storage).
//!
//! Keying on `StoreValues` (rather than `Stores`) keeps the drain a
//! trivial single-list walk and sidesteps the Kit case: a Kit's owned
//! stores enter the `Stores` access set via `Concat` with NO
//! `.with`-value (their values come from `HasTrivialCtor`, a later
//! round), so they contribute no `StoreValues` node and no arena node
//! here. That is the correct behaviour: the drain handles the explicitly
//! registered store values only.
//!
//! `DrainStores` walks the value list, reserving a one-record column per
//! `Resource<T>` through the supplied `ColumnStorage` and recording the
//! column base pointer. Resources are `ColumnValue` (`Copy + 'static`),
//! so the arena holds only raw pointers: the store owns the bytes and
//! frees them on its own `Drop`, and there is no per-resource destructor
//! to run. A zero-sized resource occupies no bytes, so it reserves no
//! column and records a dangling, well-aligned pointer.

use core::marker::PhantomData;
use core::mem::size_of;

use arvo::strategy::Identity;
use arvo::USize;
use hilavitkutin_api::store::{Column, StagedResource, Virtual};
use hilavitkutin_api::store_values::{StoreValues, Sv, SvEmpty};
use hilavitkutin_api::{ColumnStorage, ColumnValue, StoreId};

use crate::resource::provenance::{ColumnPtr, ResourcePtr};
use crate::scheduler::BuildError;

mod sealed {
    pub trait Sealed {}
}

/// Arena tail: the empty arena (matches the empty value list).
pub struct ArenaTail;

/// Arena node for one registered `Resource<T>`.
///
/// Holds the `ResourcePtr<T>` for the moved-in value (pointing into the
/// store-reserved column, or a dangling pointer for a ZST) and the tail
/// node for the remaining store values.
pub struct ArenaResourceNode<T, Tail> {
    pub(crate) ptr: ResourcePtr<T>,
    pub(crate) tail: Tail,
}

impl<T, Tail> ArenaResourceNode<T, Tail> {
    /// The recorded resource pointer. Hidden test accessor: lets tests
    /// deref the moved-in value. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __ptr(&self) -> ResourcePtr<T> {
        self.ptr
    }

    /// The tail node. Hidden test accessor; not supported surface.
    #[doc(hidden)]
    pub fn __tail(&self) -> &Tail {
        &self.tail
    }
}

/// Arena node for one registered `Column<T>`.
///
/// Placeholder: holds a null `ColumnPtr<T>` and a zero record count.
/// Column buffer reservation needs the plan-phase record count, a later
/// round.
pub struct ArenaColumnNode<T, Tail> {
    pub(crate) _ptr: ColumnPtr<T>,
    pub(crate) _count: USize,
    // Structural cons-list link. The resource `Selector` only traverses
    // resource nodes, so nothing reads this tail until column-node access
    // lands; the `ArenaFor` mapping requires it to chain the list.
    #[allow(dead_code)]
    pub(crate) tail: Tail,
}

/// Arena node for one registered `Virtual<T>`.
///
/// Carries no pointer: a `Virtual<T>` store is a DAG-edge marker with
/// no backing storage.
pub struct ArenaVirtualNode<T, Tail> {
    pub(crate) _marker: PhantomData<T>,
    // Structural cons-list link, unread until virtual-node access lands
    // (the resource `Selector` only traverses resource nodes).
    #[allow(dead_code)]
    pub(crate) tail: Tail,
}

/// Maps a `StoreValues` list to its concrete arena shape.
///
/// Sealed: the four arms (`SvEmpty`, and `Sv` headed by
/// `StagedResource<T>` / `Column<T>` / `Virtual<T>`) are the only
/// store-value shapes the builder places on the list. The heads are
/// distinct concrete types, so the arms are non-overlapping.
#[allow(private_bounds)]
pub trait ArenaFor: sealed::Sealed {
    /// The concrete arena cons-list for this value list.
    ///
    /// The arena holds only `Copy` pointers; the store frees resource
    /// memory on its own `Drop`, so the scheduler runs no arena walk on
    /// drop and `Arena` carries no destructor bound.
    type Arena;
}

impl sealed::Sealed for SvEmpty {}
impl ArenaFor for SvEmpty {
    type Arena = ArenaTail;
}

impl<T: 'static, L: StoreValues + ArenaFor> sealed::Sealed for Sv<StagedResource<T>, L> {}
impl<T: 'static, L: StoreValues + ArenaFor> ArenaFor for Sv<StagedResource<T>, L> {
    type Arena = ArenaResourceNode<T, <L as ArenaFor>::Arena>;
}

impl<T: 'static, L: StoreValues + ArenaFor> sealed::Sealed for Sv<Column<T>, L> {}
impl<T: 'static, L: StoreValues + ArenaFor> ArenaFor for Sv<Column<T>, L> {
    type Arena = ArenaColumnNode<T, <L as ArenaFor>::Arena>;
}

impl<T: 'static, L: StoreValues + ArenaFor> sealed::Sealed for Sv<Virtual<T>, L> {}
impl<T: 'static, L: StoreValues + ArenaFor> ArenaFor for Sv<Virtual<T>, L> {
    type Arena = ArenaVirtualNode<T, <L as ArenaFor>::Arena>;
}

/// Reserves and populates the arena by consuming a `StoreValues` list.
///
/// Implemented on the value list. Per `Resource<T>` value: reserve a
/// one-record column through the store, write the carrier's value in,
/// record the column base pointer. A zero-sized `T` reserves no column
/// and records a dangling pointer. Per `Column<T>` / `Virtual<T>` value:
/// no reservation (the column buffer round and virtual markers carry no
/// storage here).
///
/// Sealed: only the arms below inhabit it.
#[allow(private_bounds)]
pub trait DrainStores: ArenaFor + sealed::Sealed {
    /// Reserve and populate the arena, consuming the value list.
    ///
    /// `next_id` is the running drain-order column id, advanced once per
    /// non-zero-sized resource. On reservation failure the store frees
    /// every column reserved so far when it is dropped (the resources are
    /// `Copy`, so no destructor is skipped), and
    /// `Err(BuildError::AllocationFailed)` returns.
    fn drain<CS: ColumnStorage>(
        self,
        cs: &mut CS,
        next_id: &mut USize,
    ) -> notko::Outcome<Self::Arena, BuildError>;
}

impl DrainStores for SvEmpty {
    #[inline]
    fn drain<CS: ColumnStorage>(
        self,
        _cs: &mut CS,
        _next_id: &mut USize,
    ) -> notko::Outcome<Self::Arena, BuildError> {
        notko::Outcome::Ok(ArenaTail)
    }
}

impl<T: ColumnValue, L> DrainStores for Sv<StagedResource<T>, L>
where
    L: StoreValues + ArenaFor + DrainStores,
{
    fn drain<CS: ColumnStorage>(
        self,
        cs: &mut CS,
        next_id: &mut USize,
    ) -> notko::Outcome<Self::Arena, BuildError> {
        let (carrier, rest) = self.into_parts();
        let value = carrier.into_inner();
        // lint:allow(no-bare-numeric) reason: size_of returns usize by contract; tracked: #345
        let ptr = if size_of::<T>() == 0 {
            // Zero-sized resource: no bytes to store. A column reserve
            // would hand back a null base pointer (nothing allocated),
            // and `ResourcePtr` is non-null by construction. Record a
            // dangling, well-aligned pointer; writing and reading a ZST
            // through any aligned non-null pointer touches no memory.
            let dangling = core::ptr::NonNull::<T>::dangling().as_ptr();
            // SAFETY: a ZST write touches no memory; `dangling` is the
            // type's alignment, which is a valid address for a ZST.
            unsafe {
                core::ptr::write(dangling, value);
            }
            // SAFETY: `NonNull::dangling` is non-null.
            unsafe { ResourcePtr::new_unchecked(dangling) }
        } else {
            let id = StoreId(*next_id);
            // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal drain-order counter; tracked: #72
            *next_id = USize((*next_id).0 + 1);
            // Reserve one record's worth of column for this resource.
            // lint:allow(no-bare-numeric) reason: one-record column reservation count; tracked: #345
            match cs.reserve::<T>(id, USize(1)) {
                notko::Outcome::Ok(()) => {}
                notko::Outcome::Err(_) => return notko::Outcome::Err(BuildError::AllocationFailed),
            }
            // SAFETY: `id` names a column just reserved for `T`; the store
            // returns its 64-byte-aligned base pointer sized for one `T`.
            let typed = unsafe { cs.column_ptr_mut::<T>(id) };
            if typed.is_null() {
                return notko::Outcome::Err(BuildError::AllocationFailed);
            }
            // SAFETY: `typed` is non-null (checked), suitably aligned, and
            // sized for one `T` (just reserved); the write initialises it
            // without reading or dropping prior (uninitialised) contents.
            unsafe {
                core::ptr::write(typed, value);
            }
            // SAFETY: non-null checked above.
            unsafe { ResourcePtr::new_unchecked(typed) }
        };
        match <L as DrainStores>::drain(rest, cs, next_id) {
            notko::Outcome::Ok(tail) => notko::Outcome::Ok(ArenaResourceNode { ptr, tail }),
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
}

impl<T: 'static, L> DrainStores for Sv<Column<T>, L>
where
    L: StoreValues + ArenaFor + DrainStores,
{
    fn drain<CS: ColumnStorage>(
        self,
        cs: &mut CS,
        next_id: &mut USize,
    ) -> notko::Outcome<Self::Arena, BuildError> {
        // Column buffers need the plan-phase record count, a later round.
        // The column node is a placeholder: a dangling (well-aligned)
        // pointer and a zero record count.
        let (_marker, rest) = self.into_parts();
        // SAFETY: `NonNull::dangling`-shaped pointer; never dereferenced.
        // `ColumnPtr::new_unchecked` only requires non-null, and
        // `NonNull::dangling` is non-null.
        let placeholder =
            unsafe { ColumnPtr::new_unchecked(core::ptr::NonNull::<T>::dangling().as_ptr()) };
        match <L as DrainStores>::drain(rest, cs, next_id) {
            notko::Outcome::Ok(tail) => notko::Outcome::Ok(ArenaColumnNode {
                _ptr: placeholder,
                _count: USize::ZERO,
                tail,
            }),
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
}

impl<T: 'static, L> DrainStores for Sv<Virtual<T>, L>
where
    L: StoreValues + ArenaFor + DrainStores,
{
    fn drain<CS: ColumnStorage>(
        self,
        cs: &mut CS,
        next_id: &mut USize,
    ) -> notko::Outcome<Self::Arena, BuildError> {
        let (_marker, rest) = self.into_parts();
        match <L as DrainStores>::drain(rest, cs, next_id) {
            notko::Outcome::Ok(tail) => notko::Outcome::Ok(ArenaVirtualNode {
                _marker: PhantomData,
                tail,
            }),
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
}
