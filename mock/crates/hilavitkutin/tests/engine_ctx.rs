//! Per-WU projected Context tests (B3).
//!
//! Resources project out of a hand-built B2a arena (via the engine
//! scheduler's stack-backed `MemoryProvider`); columns project out of a
//! hand-provided column buffer, since the B2a arena column nodes are
//! dangling placeholders and per-frame column buffers belong to the
//! run-loop. These integration tests live under `tests/` so the bare
//! byte buffers backing the column store do not trip the src-tree
//! primitive lints.
//!
//! Bundle internals are reached through the engine's hidden `__new`
//! constructors and the `project_reads` projection helper.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::strategy::Identity;
use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrCons, PtrNil};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::resource::provenance::ColumnPtr;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::context::{
    BatchApi, ColumnReaderApi, ColumnWriterApi, EachApi, HasBatch, HasEach, HasResourceProvider,
    ResourceProviderApi,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Column, Resource};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};

// ---------------------------------------------------------------------
// Stack-backed test memory provider (mirrors resource_arena.rs).
// ---------------------------------------------------------------------

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

// Access-set aliases used across the tests.
type ReadU32 = Cons<Resource<u32>, Empty>;
type ColU32 = Cons<Column<u32>, Empty>;

#[test]
fn context_resolves_resource() {
    // Build a one-resource arena, project the Context off it through the
    // public projecting constructor, resolve the value.
    let provider = BumpProvider::<256>::new();
    let scheduler = Scheduler::builder()
        .with(Resource::new(99u32))
        .build(provider)
        .unwrap_or_else(|_| panic!("build should succeed"));
    let arena = scheduler.__arena();

    let ctx: EngineCtx<'_, ReadU32, Empty, _, _> =
        EngineCtx::project(arena, &ColPtrNil, MorselRange::new(USize::ZERO, USize::ZERO));

    let value: &u32 = ctx.resources().resource::<u32>();
    assert_eq!(*value, 99);
}

#[test]
fn context_column_read_after_write() {
    // Hand-provide a column buffer; the Context writes then reads back.
    let mut buf = [0u32; 8];
    // SAFETY: `buf` outlives `ctx`; the pointer is non-null and aligned.
    let col_ptr = unsafe { ColumnPtr::new_unchecked(buf.as_mut_ptr()) };
    // The per-frame column source the run-loop hands in. The projecting
    // constructor re-projects it over the access set.
    let col_source = ColPtrCons::__new(col_ptr, ColPtrNil);

    // The column is both read and written, so it appears in `R` and `W`.
    // No resources are declared, so the resource source is empty.
    let ctx: EngineCtx<'_, ColU32, ColU32, _, _> =
        EngineCtx::project(&PtrNil, &col_source, MorselRange::new(USize::ZERO, USize(8)));

    // SAFETY: the morsel covers records 0..8; the buffer is 8 long.
    unsafe {
        ctx.write::<u32>(USize(3), 4242u32);
        let got = ctx.read::<u32>(USize(3));
        assert_eq!(got, 4242);
    }
    // The raw buffer reflects the write at the morsel-offset index.
    assert_eq!(buf[3], 4242);
}

#[test]
fn context_each_covers_morsel() {
    let ctx: EngineCtx<'_, Empty, Empty, _, _> =
        EngineCtx::project(&PtrNil, &ColPtrNil, MorselRange::new(USize(5), USize(3)));

    let mut visited: [usize; 3] = [0; 3];
    let mut n = 0usize;
    // `each()` and `batch()` both return `&Self` (the Context is its own
    // provider), and `Self` implements both `EachApi::run` and
    // `BatchApi::run`, so name the trait explicitly.
    EachApi::run(ctx.each(), |i| {
        visited[n] = i.0;
        n += 1;
    });
    assert_eq!(n, 3);
    assert_eq!(visited, [5, 6, 7]);
}

#[test]
fn context_batch_full_range() {
    let ctx: EngineCtx<'_, Empty, Empty, _, _> =
        EngineCtx::project(&PtrNil, &ColPtrNil, MorselRange::new(USize(5), USize(3)));

    let seen = Cell::new((0usize, 0usize));
    BatchApi::run(ctx.batch(), |start, end| {
        seen.set((start.0, end.0));
    });
    // Batch hands the full half-open range [start, start + len) in one call.
    assert_eq!(seen.get(), (5, 8));
}

// A sample WorkUnit whose execute reads a resource through the Context,
// proving the Context satisfies the HasX bounds end to end.
struct ReadResourceWu;

impl BuilderInput for ReadResourceWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for ReadResourceWu {
    type Read = ReadU32;
    type Write = Empty;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<'frame, ReadU32, Empty, PtrCons<u32, PtrNil>, ColPtrNil>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // Resolve the resource through the projected Context. Writing the
        // observed value into a process-static cell lets the test confirm
        // the body ran and saw the registered value.
        let v: &u32 = ctx.resources().resource::<u32>();
        OBSERVED.with(|c| c.set(*v));
    }
}

thread_local! {
    static OBSERVED: Cell<u32> = const { Cell::new(0) };
}

#[test]
fn context_drives_wu_execute() {
    let provider = BumpProvider::<256>::new();
    let scheduler = Scheduler::builder()
        .with(Resource::new(7u32))
        .build(provider)
        .unwrap_or_else(|_| panic!("build should succeed"));
    let arena = scheduler.__arena();

    let ctx: <ReadResourceWu as WorkUnit>::Ctx<'_> =
        EngineCtx::project(arena, &ColPtrNil, MorselRange::new(USize::ZERO, USize::ZERO));

    let wu = ReadResourceWu;
    wu.execute(&ctx);
    assert_eq!(OBSERVED.with(|c| c.get()), 7);
}

// Negative coverage is wired as a `compile_fail` doctest on
// `EngineCtx::project` in `src/dispatch/engine_ctx.rs`: an `EngineCtx`
// for a Read set containing `Resource<u32>` cannot be projected from an
// empty source, because the `Project` bound is then unsatisfiable. The
// physical-projection invariant is enforced by the type system, not by
// construction discipline.
