//! The binding of one registered `Resource<T>`: its cons-cell, its place in
//! the value-list mapping, its drain and its frame-start reset.

use core::marker::PhantomData;
use core::mem::size_of;

use arvo::USize;
use hilavitkutin_api::access::Cons;
use hilavitkutin_api::store::{Resource, StagedResource};
use hilavitkutin_api::store_values::{StoreValues, Sv};
use hilavitkutin_api::{ColumnStorage, ColumnValue, StoreId};

use super::{BindingsFor, DrainStores, ResetAccumulators, sealed};
use crate::resource::provenance::{ErasedResourcePtr, ResourcePtr};
use crate::resource::shape::ValueShape;
use crate::scheduler::BuildError;

/// Bindings cons-cell for one registered `Resource<T>`.
///
/// Records the moved-in value's one-record blob base in erased form
/// (`ErasedResourcePtr`, pointing into the store-reserved column, or a
/// dangling base for a ZST) plus the value's static shape, and the tail
/// node for the remaining store values. The typed `ResourcePtr<T>` is
/// recovered by backcast at projection time; the binding's type
/// parameter is the backcast witness and carries the value type's
/// Send/Sync gating through `PhantomData<T>`.
pub struct ResourceBinding<T, Tail> {
    pub(crate) erased: ErasedResourcePtr,
    pub(crate) shape:  ValueShape,
    pub(crate) _ty:    PhantomData<T>,
    pub(crate) tail:   Tail,
}

impl<T, Tail> ResourceBinding<T, Tail> {
    /// The typed resource pointer, backcast from the erased base. Hidden
    /// accessor used by the projection and by tests. Not part of the
    /// supported surface.
    #[doc(hidden)]
    pub fn __ptr(&self) -> ResourcePtr<T> {
        // Forward-looking guard: tautological in-process (this binding's
        // own `T` recorded the shape), a real check only at a future
        // extension boundary where a foreign shape is supplied.
        debug_assert_eq!(self.shape, ValueShape::of::<T>());
        // SAFETY: the binding's type parameter witnesses that the erased
        // base was recorded for a `T` at drain (the drain writes the
        // staged `T` and erases the same pointer).
        unsafe { self.erased.typed::<T>() }
    }

    /// The recorded static shape. Hidden test accessor; not supported
    /// surface.
    #[doc(hidden)]
    pub fn __shape(&self) -> ValueShape {
        self.shape
    }

    /// The tail node. Hidden test accessor; not supported surface.
    #[doc(hidden)]
    pub fn __tail(&self) -> &Tail {
        &self.tail
    }
}

impl<T: 'static, L: StoreValues + BindingsFor> sealed::BindingsSealed for Sv<StagedResource<T>, L> {}
impl<T: 'static, L: StoreValues + BindingsFor> BindingsFor for Sv<StagedResource<T>, L> {
    type Bindings = ResourceBinding<T, <L as BindingsFor>::Bindings>;
    // The value list carries the `StagedResource<T>` carrier, but the builder's
    // `Stores` access set lists `Resource<T>` (`StagedResource<T>` dispatches as
    // `StoreDispatch<Resource<T>>`). Recover the access marker, not the carrier,
    // so `MaskProject`/`Locate` over `Markers` find the same bit positions the
    // WU access sets (which reference `Resource<T>`) use.
    type Markers = Cons<Resource<T>, <L as BindingsFor>::Markers>;
}

impl<T: ColumnValue, L> DrainStores for Sv<StagedResource<T>, L>
where
    L: StoreValues + BindingsFor + DrainStores,
{
    fn drain<CS: ColumnStorage>(
        self,
        cs: &mut CS,
        next_id: &mut USize,
        record_count: USize,
    ) -> notko::Outcome<Self::Bindings, BuildError> {
        let (carrier, rest) = self.into_parts();
        let value = carrier.into_inner();
        let zero_sized = size_of::<T>() == 0; // lint:allow(no-bare-numeric) reason: size_of returns usize by contract; tracked: #345
        let typed_base = if zero_sized {
            // Zero-sized resource: no bytes to store. A column reserve
            // would hand back a null base pointer (nothing allocated),
            // and the recorded base is non-null by construction. Record a
            // dangling, well-aligned pointer; writing and reading a ZST
            // through any aligned non-null pointer touches no memory.
            let dangling = core::ptr::NonNull::<T>::dangling().as_ptr();
            // SAFETY: a ZST write touches no memory; `dangling` is the
            // type's alignment, which is a valid address for a ZST.
            unsafe {
                core::ptr::write(dangling, value);
            }
            dangling
        } else {
            let id = StoreId(*next_id);
            *next_id = USize(next_id.0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal drain-order counter; tracked: #72
            // Reserve one record's worth of column for this resource.
            let one_record = USize(1); // lint:allow(no-bare-numeric) reason: one-record column reservation count; tracked: #345
            match cs.reserve::<T>(id, one_record) {
                notko::Outcome::Ok(()) => {},
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
            typed
        };
        // Record the base in erased form plus the value's static shape;
        // the typed view is backcast at projection time.
        // SAFETY: `typed_base` is non-null on both arms (dangling for a
        // ZST, checked for a reserved column).
        let erased = unsafe { ErasedResourcePtr::new_unchecked(typed_base as *mut u8) }; // lint:allow(no-bare-numeric) reason: erased byte base is the addressing contract; tracked: #654
        match <L as DrainStores>::drain(rest, cs, next_id, record_count) {
            notko::Outcome::Ok(tail) => {
                notko::Outcome::Ok(ResourceBinding {
                    erased,
                    shape: ValueShape::of::<T>(),
                    _ty: PhantomData,
                    tail,
                })
            },
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
}

impl<T, Tail: ResetAccumulators> ResetAccumulators for ResourceBinding<T, Tail> {
    #[inline]
    fn reset_accumulators(&self) {
        self.tail.reset_accumulators();
    }
}
