//! Per-WU projected Context tests (B3).
//!
//! Resources project out of a hand-built B2a bindings (via the engine
//! scheduler's stack-backed `MemoryProvider`); columns project out of a
//! hand-provided column buffer, since the B2a bindings column nodes are
//! dangling placeholders and per-frame column buffers belong to the
//! run-loop. These integration tests live under `tests/` so the bare
//! byte buffers backing the column store do not trip the src-tree
//! primitive lints.
//!
//! Bundle internals are reached through the engine's hidden `__new`
//! constructors and the `project_reads` projection helper.

mod stable_provider;

use core::cell::Cell;

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapCons, SnapNil};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::resource::provenance::ColumnPtr;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    BatchApi,
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasBatch,
    HasEach,
    HasResourceProvider,
    ResourceProviderApi,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Column, Resource};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use stable_provider::stable_provider;

/// Wrap a provider in the default-capacity bindings store (`D = Dim<256>`).
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// ---------------------------------------------------------------------
// Stack-backed test memory provider (mirrors resource_bindings.rs).
// ---------------------------------------------------------------------

// Access-set aliases used across the tests.
type ReadU32 = Cons<Resource<u32>, Empty>;
type ColU32 = Cons<Column<u32>, Empty>;
// Distinct read/write column sets: `u16` is read AND written; `u8` is
// read-only, `u32` write-only. Exercises `ColSelector` index resolution
// to both `Here` and `There<Here>` on each side of the bundle split.
type ReadU8U16 = Cons<Column<u8>, Cons<Column<u16>, Empty>>;
type WriteU16U32 = Cons<Column<u16>, Cons<Column<u32>, Empty>>;

#[test]
fn context_resolves_resource() {
    // Build a one-resource bindings, project the Context off it through the
    // public projecting constructor, resolve the value.
    let provider = stable_provider(256, 0);
    let scheduler = Scheduler::builder()
        .with(Resource::new(99u32))
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let bindings = scheduler.__bindings();

    let meta = hilavitkutin::meta::MetaBlock::default();
    let ctx: EngineCtx<'_, ReadU32, Empty, _, _, _> = EngineCtx::project(
        bindings,
        &ColPtrNil,
        &meta,
        <USize as Identity<Additive>>::IDENTITY,
        MorselRange::new(
            <USize as Identity<Additive>>::IDENTITY,
            <USize as Identity<Additive>>::IDENTITY,
        ),
    );

    // The binding annotation pins `T = u32`; the index `I` infers from the
    // concrete bundle, so no turbofish is needed.
    let value: &u32 = ctx.resources().resource();
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
    let col_source = ColPtrCons::__new(col_ptr, USize(8), ColPtrNil);

    // The column is both read and written, so it appears in `R` and `W`.
    // No resources are declared, so the resource source is empty.
    let meta = hilavitkutin::meta::MetaBlock::default();
    let ctx: EngineCtx<'_, ColU32, ColU32, _, _, _> = EngineCtx::project(
        &SnapNil,
        &col_source,
        &meta,
        <USize as Identity<Additive>>::IDENTITY,
        MorselRange::new(<USize as Identity<Additive>>::IDENTITY, USize(8)),
    );

    // SAFETY: the morsel covers records 0..8; the buffer is 8 long.
    unsafe {
        ctx.write(USize(3), 4242u32);
        let got: u32 = ctx.read(USize(3));
        assert_eq!(got, 4242);
    }
    // The raw buffer reflects the write at the morsel-offset index.
    assert_eq!(buf[3], 4242);
}

#[test]
fn context_multi_column_distinct_read_write() {
    // R reads u8 + u16; W writes u16 + u32. `u16` is both read and written
    // (one buffer). The single column source carries all three columns;
    // `project` builds read_cols over R (u8, u16) and write_cols over W
    // (u16, u32). Resolution must infer `Here` and `There<Here>` on each
    // side, the deeper path the single-element read-after-write test omits.
    let mut buf_a = [0u8; 8];
    let mut buf_b = [0u16; 8];
    let mut buf_c = [0u32; 8];
    buf_a[3] = 7; // pre-fill the read-only column
    // SAFETY: each buffer outlives `ctx`; pointers are non-null and aligned.
    let pa = unsafe { ColumnPtr::new_unchecked(buf_a.as_mut_ptr()) };
    let pb = unsafe { ColumnPtr::new_unchecked(buf_b.as_mut_ptr()) };
    let pc = unsafe { ColumnPtr::new_unchecked(buf_c.as_mut_ptr()) };
    // The source holds every column the access sets project over, in the
    // order [u8, u16, u32]; per-side projection pulls the relevant subset.
    let col_source = ColPtrCons::__new(
        pa,
        USize(8),
        ColPtrCons::__new(pb, USize(8), ColPtrCons::__new(pc, USize(8), ColPtrNil)),
    );

    let meta = hilavitkutin::meta::MetaBlock::default();
    let ctx: EngineCtx<'_, ReadU8U16, WriteU16U32, _, _, _> = EngineCtx::project(
        &SnapNil,
        &col_source,
        &meta,
        <USize as Identity<Additive>>::IDENTITY,
        MorselRange::new(<USize as Identity<Additive>>::IDENTITY, USize(8)),
    );

    // SAFETY: the morsel covers records 0..8; each buffer is 8 long.
    unsafe {
        // write side: u16 resolves at index `Here`, u32 at `There<Here>`.
        ctx.write(USize(3), 99u16);
        ctx.write(USize(3), 123u32);
        // read side: u8 resolves at `Here`, u16 at `There<Here>`.
        let a: u8 = ctx.read(USize(3));
        let b: u16 = ctx.read(USize(3));
        assert_eq!(a, 7); // read-only column, untouched by the writes
        assert_eq!(b, 99); // read-write column resolves to the written buffer
    }
    // Each column wrote its own buffer at the morsel offset.
    assert_eq!(buf_b[3], 99);
    assert_eq!(buf_c[3], 123);
}

#[test]
fn context_each_covers_morsel() {
    let meta = hilavitkutin::meta::MetaBlock::default();
    let ctx: EngineCtx<'_, Empty, Empty, _, _, _> = EngineCtx::project(
        &SnapNil,
        &ColPtrNil,
        &meta,
        <USize as Identity<Additive>>::IDENTITY,
        MorselRange::new(USize(5), USize(3)),
    );

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
    // `each` yields a morsel-relative index `[0, len)`; `read` / `write` add
    // `morsel.start`. With start = 5 the relative range [0, 3) differs from the
    // absolute range [5, 8), so this pins relative semantics, not absolute.
    assert_eq!(visited, [0, 1, 2]);
}

#[test]
fn context_batch_full_range() {
    let meta = hilavitkutin::meta::MetaBlock::default();
    let ctx: EngineCtx<'_, Empty, Empty, _, _, _> = EngineCtx::project(
        &SnapNil,
        &ColPtrNil,
        &meta,
        <USize as Identity<Additive>>::IDENTITY,
        MorselRange::new(USize(5), USize(3)),
    );

    let seen = Cell::new((0usize, 0usize));
    BatchApi::run(ctx.batch(), |start, end| {
        seen.set((start.0, end.0));
    });
    // Batch hands the morsel-relative half-open range [0, len) in one call; a
    // body looping it and calling `write(i)` lands at `morsel.start + i`. With
    // morsel start = 5 the relative range [0, 3) differs from the absolute
    // [5, 8), so this pins relative semantics.
    assert_eq!(seen.get(), (0, 3));
}

// A sample WorkUnit whose execute reads a resource through the Context,
// proving the Context satisfies the HasX bounds end to end.
struct ReadResourceWu;

impl BuilderInput for ReadResourceWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for ReadResourceWu {
    type Ctx<'frame> =
        EngineCtx<'frame, ReadU32, Empty, SnapCons<u32, SnapNil>, ColPtrNil, ColPtrNil>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadU32;
    type Write = Empty;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // Resolve the resource through the projected Context. Writing the
        // observed value into a process-static cell lets the test confirm
        // the body ran and saw the registered value.
        let v: &u32 = ctx.resources().resource();
        OBSERVED.with(|c| c.set(*v));
    }
}

thread_local! {
    static OBSERVED: Cell<u32> = const { Cell::new(0) };
}

#[test]
fn context_drives_wu_execute() {
    let provider = stable_provider(256, 0);
    let scheduler = Scheduler::builder()
        .with(Resource::new(7u32))
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let bindings = scheduler.__bindings();

    let meta = hilavitkutin::meta::MetaBlock::default();
    let ctx: <ReadResourceWu as WorkUnit>::Ctx<'_> = EngineCtx::project(
        bindings,
        &ColPtrNil,
        &meta,
        <USize as Identity<Additive>>::IDENTITY,
        MorselRange::new(
            <USize as Identity<Additive>>::IDENTITY,
            <USize as Identity<Additive>>::IDENTITY,
        ),
    );

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
