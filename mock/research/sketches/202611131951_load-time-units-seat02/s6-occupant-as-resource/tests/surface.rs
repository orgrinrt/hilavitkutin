//! Seat 02 phase two. Question: can a unit on the trunk declare the record's
//! own seam, `ExtensionSurface<dyn F>`, in its read set, as the api design
//! says ("WUs can declare reads and writes against `LinkedBin<dyn
//! TraitFamily>` and the typestate proves coverage at `.build()` time",
//! hilavitkutin-api/DESIGN.md.tmpl:208-210), and what does its body reach?
//!
//! `a_unit_reading_the_surface_builds_and_runs` declares the surface beside a
//! column and registers the surface with `.with`. If this file does not
//! compile, the seam has no expressible form for a unit on the trunk, and the
//! compiler output is the finding.

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, ExtensionSurface, UnitDispatch};
use hilavitkutin_api::context::{ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

struct ZeroedHeap;
unsafe impl Send for ZeroedHeap {}
unsafe impl Sync for ZeroedHeap {}
impl MemoryProviderApi for ZeroedHeap {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        unsafe {
            std::alloc::alloc_zeroed(std::alloc::Layout::from_size_align(len.0, align.0).unwrap())
        }
    }

    unsafe fn deallocate(&self, ptr: *mut u8, len: USize, align: USize) {
        unsafe {
            std::alloc::dealloc(ptr, std::alloc::Layout::from_size_align(len.0, align.0).unwrap())
        }
    }

    unsafe fn protect(&self, _p: *mut u8, _l: USize, _r: Bool, _w: Bool) {}
}

fn store<M: MemoryProviderApi>(m: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(m)
}

/// A trait family a consumer would declare for its occupants.
pub trait Family {}

#[derive(Copy, Clone)]
struct A(i64);
#[derive(Copy, Clone)]
struct B(i64);

type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

type HostRead = Cons<ExtensionSurface<dyn Family>, Cons<Column<A>, Empty>>;
type HostWrite = Cons<Column<B>, Empty>;

struct Host;
impl BuilderInput for Host {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Host {
    type Ctx<'f> = EngineCtx<'f, HostRead, HostWrite, SnapNil, ColPtrCons<A, ColPtrNil>, ColPtrCons<B, ColPtrNil>>;
    type Hint = Hints;
    type Read = HostRead;
    type Write = HostWrite;

    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        // Nothing here can reach an occupant: the surface is a zero-data
        // marker (hilavitkutin-api/src/builder_input.rs:174) and no accessor
        // on the context names it.
        EachApi::run(ctx.each(), |i| unsafe {
            let a = ctx.reader().read::<A, _>(i).0;
            ctx.writer().write::<B, _>(i, B(a));
        });
    }
}

#[test]
fn a_unit_reading_the_surface_builds_and_runs() {
    let mut s = Scheduler::builder()
        .with(ExtensionSurface::<dyn Family>::new())
        .with(Column::<A>::new())
        .with(Column::<B>::new())
        .with(Host)
        .build(store(ZeroedHeap), USize(64))
        .unwrap_or_else(|_| panic!("build"));
    assert!(matches!(s.run(), Outcome::Ok(())));
}
