//! Resource snapshot + erased-addressing contract tests (round 202606210600).
//!
//! Pins the domain-19 storage model: the Context projection copies each
//! read-set resource value into the Context (the stack-local snapshot),
//! so a `resource()` read observes the projection-time value, never a
//! later canonical-storage mutation; and the binding addresses the value
//! through an erased base plus static shape, with the typed view
//! recovered by backcast.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::strategy::Identity;
use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrNil, EngineCtx};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::resource::shape::ValueShape;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::context::{HasResourceProvider, ResourceProviderApi};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Resource;
use hilavitkutin_providers::ArenaColumnStorage;

/// Wrap a provider in the default-capacity bindings store (`D = Dim<256>`).
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Stack-backed test memory provider (mirrors engine_ctx.rs).
struct BumpProvider<const N: usize> {
    buf: UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
}

impl<const N: usize> BumpProvider<N> {
    fn new() -> Self {
        Self {
            buf: UnsafeCell::new([const { MaybeUninit::uninit() }; N]),
            used: Cell::new(0),
        }
    }
}

unsafe impl<const N: usize> Send for BumpProvider<N> {}
unsafe impl<const N: usize> Sync for BumpProvider<N> {}

impl<const N: usize> MemoryProviderApi for BumpProvider<N> {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let base = self.buf.get() as *mut u8;
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > N {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        // SAFETY: `aligned + len <= N`, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

type ReadU32 = Cons<Resource<u32>, Empty>;

#[test]
fn resource_read_is_projection_time_snapshot() {
    // The projection copies the value into the Context; a canonical-storage
    // mutation AFTER projection must not be observed through `resource()`.
    // This is the semantic face of the domain-19 stack-local caching (the
    // codegen face, register promotion, is the A2b asm-gate fixture).
    let provider = BumpProvider::<256>::new();
    let scheduler = Scheduler::builder()
        .with(Resource::new(11u32))
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let bindings = scheduler.__bindings();

    let meta = hilavitkutin::meta::MetaBlock::default();
    let ctx: EngineCtx<'_, ReadU32, Empty, _, _, _> = EngineCtx::project(
        bindings,
        &ColPtrNil,
        &meta,
        USize::ZERO,
        MorselRange::new(USize::ZERO, USize::ZERO),
    );

    // Mutate the canonical blob through the binding's typed base pointer,
    // after the Context was projected.
    // SAFETY: the pointer names the drained one-record u32 blob; no live
    // borrow of the canonical storage exists (the Context snapshot is a
    // copy), and the test is single-threaded.
    unsafe {
        core::ptr::write(bindings.__ptr().as_ptr(), 22u32);
    }

    let value: &u32 = ctx.resources().resource();
    assert_eq!(
        *value, 11,
        "resource() must return the projection-time snapshot, not the mutated canonical value"
    );
}

#[test]
fn erased_backcast_roundtrip() {
    // The binding records an erased base plus the value's static shape; the
    // typed view recovered by backcast reads the staged value back exactly.
    let provider = BumpProvider::<256>::new();
    let scheduler = Scheduler::builder()
        .with(Resource::new(0xC0FFEEu32))
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let bindings = scheduler.__bindings();

    // `__ptr()` is the backcast: erased base -> `ResourcePtr<u32>`.
    // SAFETY: the drained one-record blob holds an initialised u32.
    let read_back = unsafe { *bindings.__ptr().as_ptr() };
    assert_eq!(read_back, 0xC0FFEEu32);

    // The recorded shape is the value's static shape.
    assert_eq!(bindings.__shape(), ValueShape::of::<u32>());
    assert_eq!(ValueShape::of::<u32>().size, USize(4));
    assert_eq!(ValueShape::of::<u32>().align, USize(4));
}

#[test]
#[ignore = "catalogue: Seq/Map collection members not yet wired into resource values; the live-stream asserts (snapshot copies ptr+len only, elements stream from the column) land with that wiring; tracked #344"]
fn collection_members_live_stream_not_snapshot_copied() {
    // Intended behaviour, per the domain-19 design and bench axis D: a
    // `Seq`/`Map` member of a resource value is a pointer-plus-length view
    // over its own column; projecting a Context copies ONLY that view, and
    // an element written to the collection column after projection IS
    // observed through the streamed accessor (elements are live), while the
    // scalar members are not (they are snapshot). Unbuilt: resource values
    // cannot yet carry wired collection members.
    unreachable!("catalogued gap: build the Seq/Map member wiring, then assert the split above");
}
