//! Resource arena: the data-plane storage built from registered store
//! values.
//!
//! The arena is keyed on the builder's `StoreValues` list, not on the
//! `Stores` access set. Each `.with`-registered store value contributes
//! one arena node: a `Resource<T>` carrier (`StagedResource<T>`)
//! contributes an `ArenaResourceNode<T, _>` holding the allocated,
//! moved-in value's `ResourcePtr<T>`; a `Column<T>` marker contributes
//! an `ArenaColumnNode` (a no-alloc placeholder in B2a, since column
//! buffers need the plan-phase record count, which lands in B2b); a
//! `Virtual<T>` marker contributes an `ArenaVirtualNode` (no backing
//! storage).
//!
//! Keying on `StoreValues` (rather than `Stores`) keeps the drain a
//! trivial single-list walk and sidesteps the Kit case: a Kit's owned
//! stores enter the `Stores` access set via `Concat` with NO
//! `.with`-value (their values come from `HasTrivialCtor`, a later
//! round), so they contribute no `StoreValues` node and no arena node
//! here. That is the correct B2a behaviour: B2a drains the explicitly
//! registered store values only.
//!
//! `DrainStores` walks the value list, allocating each `Resource<T>`'s
//! block via the supplied `MemoryProviderApi` and recording its
//! pointer. `DropArena` runs the inverse on scheduler drop: each
//! moved-in resource value is dropped in place, then its block is
//! deallocated.

use core::marker::PhantomData;
use core::mem::{align_of, size_of};

use arvo::strategy::Identity;
use arvo::USize;
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Column, StagedResource, Virtual};
use hilavitkutin_api::store_values::{StoreValues, Sv, SvEmpty};

use crate::resource::provenance::{ColumnPtr, ResourcePtr};
use crate::scheduler::BuildError;

mod sealed {
    pub trait Sealed {}
}

/// Arena tail: the empty arena (matches the empty value list).
pub struct ArenaTail;

/// Arena node for one registered `Resource<T>`.
///
/// Holds the `ResourcePtr<T>` for the allocated, moved-in value and
/// the tail node for the remaining store values.
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
/// B2a placeholder: holds a null `ColumnPtr<T>` and a zero record
/// count. Column buffer allocation lands in B2b, which needs the
/// plan-phase record count.
pub struct ArenaColumnNode<T, Tail> {
    pub(crate) _ptr: ColumnPtr<T>,
    pub(crate) _count: USize,
    pub(crate) tail: Tail,
}

/// Arena node for one registered `Virtual<T>`.
///
/// Carries no pointer: a `Virtual<T>` store is a DAG-edge marker with
/// no backing storage.
pub struct ArenaVirtualNode<T, Tail> {
    pub(crate) _marker: PhantomData<T>,
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
    /// Bounded on `DropArena` so the scheduler's `Drop` can run the
    /// arena walk without an extra where-clause that the `Scheduler`
    /// struct would have to repeat (rustc E0367).
    type Arena: DropArena;
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

/// Allocates and populates the arena by consuming a `StoreValues` list.
///
/// Implemented on the value list. Per `Resource<T>` value: allocate a
/// block via the provider, write the carrier's value in, record the
/// pointer. Per `Column<T>` / `Virtual<T>` value: no allocation in B2a.
///
/// Sealed: only the arms below inhabit it.
#[allow(private_bounds)]
pub trait DrainStores: ArenaFor + sealed::Sealed {
    /// Allocate and populate the arena, consuming the value list.
    ///
    /// On allocation failure the prefix already built is dropped (its
    /// blocks deallocated) and `Err(BuildError::AllocationFailed)` is
    /// returned, so no block leaks.
    fn drain<M: MemoryProviderApi>(self, mp: &M) -> notko::Outcome<Self::Arena, BuildError>;
}

impl DrainStores for SvEmpty {
    #[inline]
    fn drain<M: MemoryProviderApi>(self, _mp: &M) -> notko::Outcome<Self::Arena, BuildError> {
        notko::Outcome::Ok(ArenaTail)
    }
}

impl<T: 'static, L> DrainStores for Sv<StagedResource<T>, L>
where
    L: StoreValues + ArenaFor + DrainStores,
{
    fn drain<M: MemoryProviderApi>(self, mp: &M) -> notko::Outcome<Self::Arena, BuildError> {
        // lint:allow(no-bare-numeric) reason: core::mem size/align return usize by contract; tracked: #345
        let len = USize(size_of::<T>());
        // lint:allow(no-bare-numeric) reason: core::mem size/align return usize by contract; tracked: #345
        let align = USize(align_of::<T>());
        let (carrier, rest) = self.into_parts();
        // SAFETY: len/align come from size_of/align_of of T, so the
        // request is a valid layout for one T; align_of is a power of
        // two. The returned pointer is checked for null below.
        let raw = unsafe { mp.allocate(len, align) };
        if raw.is_null() {
            return notko::Outcome::Err(BuildError::AllocationFailed);
        }
        let typed = raw as *mut T;
        let value = carrier.into_inner();
        // SAFETY: `typed` is a non-null, suitably-aligned block sized
        // for one T (allocated just above); writing initialises it
        // without reading or dropping prior (uninitialised) contents.
        unsafe {
            core::ptr::write(typed, value);
        }
        // SAFETY: non-null checked above.
        let ptr = unsafe { ResourcePtr::new_unchecked(typed) };
        match <L as DrainStores>::drain(rest, mp) {
            notko::Outcome::Ok(tail) => notko::Outcome::Ok(ArenaResourceNode { ptr, tail }),
            notko::Outcome::Err(e) => {
                // The tail drain failed after this node's block was
                // allocated and its value written. Drop that value and
                // free the block so nothing leaks before propagating.
                // SAFETY: `ptr` points at an initialised, owned T in a
                // block allocated from `mp` with the same len.
                unsafe {
                    core::ptr::drop_in_place(ptr.as_ptr());
                    mp.deallocate(ptr.as_ptr() as *mut u8, len); // lint:allow(no-bare-numeric) reason: allocator ABI takes raw byte pointer by contract; tracked: #72
                }
                notko::Outcome::Err(e)
            }
        }
    }
}

impl<T: 'static, L> DrainStores for Sv<Column<T>, L>
where
    L: StoreValues + ArenaFor + DrainStores,
{
    fn drain<M: MemoryProviderApi>(self, mp: &M) -> notko::Outcome<Self::Arena, BuildError> {
        // B2a allocates resources only. The column node is a no-alloc
        // placeholder: a dangling (well-aligned) pointer and a zero
        // record count. Real buffer allocation lands in B2b.
        let (_marker, rest) = self.into_parts();
        // SAFETY: `NonNull::dangling`-shaped pointer; never dereferenced
        // in B2a. `ColumnPtr::new_unchecked` only requires non-null, and
        // `NonNull::dangling` is non-null.
        let placeholder =
            unsafe { ColumnPtr::new_unchecked(core::ptr::NonNull::<T>::dangling().as_ptr()) };
        match <L as DrainStores>::drain(rest, mp) {
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
    fn drain<M: MemoryProviderApi>(self, mp: &M) -> notko::Outcome<Self::Arena, BuildError> {
        let (_marker, rest) = self.into_parts();
        match <L as DrainStores>::drain(rest, mp) {
            notko::Outcome::Ok(tail) => notko::Outcome::Ok(ArenaVirtualNode {
                _marker: PhantomData,
                tail,
            }),
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
}

/// Walks the built arena on scheduler drop, running each resource
/// value's destructor in place and then deallocating its block.
///
/// Implemented on the concrete arena node types, because drop happens
/// from the scheduler's `Drop`, which holds the arena by value and the
/// `T` width per node. Sealed: only the arena node types below inhabit
/// it.
#[allow(private_bounds)]
pub trait DropArena: sealed::Sealed {
    /// Drop every moved-in resource value and free its block.
    fn drop_arena<M: MemoryProviderApi>(&mut self, mp: &M);
}

impl sealed::Sealed for ArenaTail {}
impl DropArena for ArenaTail {
    #[inline]
    fn drop_arena<M: MemoryProviderApi>(&mut self, _mp: &M) {}
}

impl<T, Tail: DropArena> sealed::Sealed for ArenaResourceNode<T, Tail> {}
impl<T, Tail: DropArena> DropArena for ArenaResourceNode<T, Tail> {
    fn drop_arena<M: MemoryProviderApi>(&mut self, mp: &M) {
        // lint:allow(no-bare-numeric) reason: core::mem size/align return usize by contract; tracked: #345
        let len = USize(size_of::<T>());
        // SAFETY: `ptr` points at an initialised, owned T in a block
        // allocated from `mp` with this same len. Drop the value first
        // (destructor before free), then release the block. The arena
        // is owned by value and dropped once, so no double free.
        unsafe {
            core::ptr::drop_in_place(self.ptr.as_ptr());
            mp.deallocate(self.ptr.as_ptr() as *mut u8, len); // lint:allow(no-bare-numeric) reason: allocator ABI takes raw byte pointer by contract; tracked: #72
        }
        self.tail.drop_arena(mp);
    }
}

impl<T, Tail: DropArena> sealed::Sealed for ArenaColumnNode<T, Tail> {}
impl<T, Tail: DropArena> DropArena for ArenaColumnNode<T, Tail> {
    #[inline]
    fn drop_arena<M: MemoryProviderApi>(&mut self, mp: &M) {
        // B2a column node is a no-alloc placeholder; nothing to free.
        self.tail.drop_arena(mp);
    }
}

impl<T, Tail: DropArena> sealed::Sealed for ArenaVirtualNode<T, Tail> {}
impl<T, Tail: DropArena> DropArena for ArenaVirtualNode<T, Tail> {
    #[inline]
    fn drop_arena<M: MemoryProviderApi>(&mut self, mp: &M) {
        self.tail.drop_arena(mp);
    }
}
