#![feature(const_trait_impl)]
//! Arm A4: a compiled slot unit whose body is a native kernel loaded at run
//! time through `hilavitkutin-linking`, and a name the extension minted.
//!
//! `a_slot_unit_runs_an_extension_kernel_in_the_schedule`: existence. The
//! unit's access set is fixed in Rust; the kernel arrives after the schedule
//! is compiled. The trunk's Context hands a unit one record at a time
//! (`ColumnReaderApi::read`, hilavitkutin-api/src/context.rs:117) and no
//! window pointer, so the unit copies the morsel into scratch, calls the
//! kernel once, and writes the result back: two extra passes the trunk's
//! api forces on this arm.
//!
//! `a_name_minted_in_an_extension_resolves_in_the_host`: the case that must
//! fail. hilavitkutin-str keeps const strings in a linker section walked per
//! image (hilavitkutin-str/src/section.rs), and the loader opens libraries
//! RTLD_LOCAL (hilavitkutin-linking/src/backend/unix.rs:26). A name the
//! extension declares with `str_const!` lives in the extension's section, and
//! the host's interner never sees it. Expected red.

use std::cell::RefCell;
use std::os::unix::ffi::OsStrExt;

use a4_host::ext_path;
use arvo::{Bool, USize};
use arvo_bits::{Bits, Hot};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    BatchApi,
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasBatch,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_linking::Library;
use hilavitkutin_providers::ArenaColumnStorage;
use hilavitkutin_str::{ArenaInterner, Str, StringInterner, static_entries, str_const};
use notko::{Maybe, Outcome};

type Kernel = extern "C" fn(*const i32, *const i32, *mut i32, usize);
type Mint = extern "C" fn() -> u32;
type Entries = extern "C" fn() -> usize;

fn open() -> Library {
    let mut p = ext_path().as_os_str().as_bytes().to_vec();
    p.push(0);
    // hilavitkutin-linking passes 4 as RTLD_LOCAL (backend/unix.rs:9), which
    // is RTLD_NOLOAD on linux glibc, so Library::load only succeeds for an
    // image already mapped. tests/loader.rs pins that red. To get past it
    // and reach the slot question, map the image first with the real flags;
    // the trunk's loader then finds it resident.
    let pre = unsafe {
        libc::dlopen(
            p.as_ptr() as *const libc::c_char,
            libc::RTLD_NOW | libc::RTLD_LOCAL,
        )
    };
    assert!(
        !pre.is_null(),
        "preload {:?}; run `cargo build -p a4-ext` first",
        ext_path()
    );
    match Library::load(&p) {
        Outcome::Ok(l) => l,
        Outcome::Err(_) => panic!("Library::load refused an image already mapped"),
    }
}

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
            std::alloc::dealloc(
                ptr,
                std::alloc::Layout::from_size_align(len.0, align.0).unwrap(),
            )
        }
    }

    unsafe fn protect(&self, _p: *mut u8, _l: USize, _r: Bool, _w: Bool) {}
}
fn store<M: MemoryProviderApi>(m: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(m)
}

#[derive(Copy, Clone)]
struct Pos(i32);
#[derive(Copy, Clone)]
struct Vel(i32);
#[derive(Copy, Clone)]
struct Out(i32);
type Inputs = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
type InPtrs = ColPtrCons<Pos, ColPtrCons<Vel, ColPtrNil>>;
type OutSet = Cons<Column<Out>, Empty>;
type OutPtrs = ColPtrCons<Out, ColPtrNil>;
type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);
const N: usize = 20_000;

thread_local! {
    static SEEN: RefCell<Vec<(i32, i32, i32)>> = RefCell::new(Vec::new());
}

struct Loader;
impl BuilderInput for Loader {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Loader {
    type Ctx<'f> = EngineCtx<'f, Empty, Inputs, SnapNil, ColPtrNil, InPtrs>;
    type Hint = Hints;
    type Read = Empty;
    type Write = Inputs;

    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        EachApi::run(ctx.each(), |i| unsafe {
            ctx.writer().write::<Pos, _>(i, Pos(i.0 as i32 * 5 - 3));
            ctx.writer().write::<Vel, _>(i, Vel((i.0 % 13) as i32 - 6));
        });
    }
}

/// The slot unit. Its access set is the compile-time fact; `kernel` is what
/// the extension supplied at load.
struct Slot {
    kernel: Kernel,
}
impl BuilderInput for Slot {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Slot {
    type Ctx<'f> = EngineCtx<'f, Inputs, OutSet, SnapNil, InPtrs, OutPtrs>;
    type Hint = Hints;
    type Read = Inputs;
    type Write = OutSet;

    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        BatchApi::run(ctx.batch(), |_, len| {
            let n = len.0;
            // No window accessor on the trunk's Context: copy in.
            let pos: Vec<i32> = (0 .. n)
                .map(|i| unsafe { ctx.reader().read::<Pos, _>(USize(i)).0 })
                .collect();
            let vel: Vec<i32> = (0 .. n)
                .map(|i| unsafe { ctx.reader().read::<Vel, _>(USize(i)).0 })
                .collect();
            let mut out = vec![0i32; n];
            (self.kernel)(pos.as_ptr(), vel.as_ptr(), out.as_mut_ptr(), n);
            // And copy out.
            for (i, v) in out.iter().enumerate() {
                unsafe { ctx.writer().write::<Out, _>(USize(i), Out(*v)) };
            }
        });
    }
}

struct Observer;
impl BuilderInput for Observer {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Observer {
    type Ctx<'f> = EngineCtx<
        'f,
        Cons<Column<Pos>, Cons<Column<Vel>, OutSet>>,
        Empty,
        SnapNil,
        ColPtrCons<Pos, ColPtrCons<Vel, OutPtrs>>,
        ColPtrNil,
    >;
    type Hint = Hints;
    type Read = Cons<Column<Pos>, Cons<Column<Vel>, OutSet>>;
    type Write = Empty;

    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        EachApi::run(ctx.each(), |i| {
            let r = ctx.reader();
            let t = unsafe {
                (
                    r.read::<Pos, _>(i).0,
                    r.read::<Vel, _>(i).0,
                    r.read::<Out, _>(i).0,
                )
            };
            SEEN.with(|s| s.borrow_mut().push(t));
        });
    }
}

#[test]
fn a_slot_unit_runs_an_extension_kernel_in_the_schedule() {
    let lib = open();
    let kernel: Kernel = match lib.resolve::<Kernel>(b"ext_kernel\0") {
        Outcome::Ok(s) => s.get(),
        Outcome::Err(_) => panic!("resolve ext_kernel"),
    };
    SEEN.with(|s| s.borrow_mut().clear());
    let mut s = Scheduler::builder()
        .with(Column::<Pos>::new())
        .with(Column::<Vel>::new())
        .with(Column::<Out>::new())
        .with(Loader)
        .with(Slot {
            kernel,
        })
        .with(Observer)
        .build(store(ZeroedHeap), USize(N))
        .unwrap_or_else(|_| panic!("build"));
    assert!(matches!(s.run(), Outcome::Ok(())));
    let seen = SEEN.with(|s| s.borrow().clone());
    assert_eq!(seen.len(), N);
    let distinct: std::collections::BTreeSet<i32> = seen.iter().map(|t| t.2).collect();
    assert!(distinct.len() > 100);
    for (p, v, o) in seen {
        assert_eq!(o, p.wrapping_add(v.wrapping_mul(2)));
    }
    drop(s);
    drop(lib);
}

struct NoArena;
impl ArenaInterner for NoArena {
    fn arena_intern(&self, _s: &str) -> u32 {
        unreachable!("const handles only")
    }

    fn arena_resolve(&self, _id: u32) -> &str {
        unreachable!("const handles only")
    }
}

#[test]
fn a_name_minted_in_an_extension_resolves_in_the_host() {
    // The host has const strings of its own, so its section exists.
    let own: Str = str_const!("host.kind.pos");
    let interner = StringInterner::new(NoArena);
    assert!(
        matches!(interner.resolve(own), Maybe::Is("host.kind.pos")),
        "control: the host resolves its own name"
    );

    let lib = open();
    let mint: Mint = match lib.resolve::<Mint>(b"ext_mint\0") {
        Outcome::Ok(s) => s.get(),
        Outcome::Err(_) => panic!("resolve ext_mint"),
    };
    let entries: Entries = match lib.resolve::<Entries>(b"ext_entries\0") {
        Outcome::Ok(s) => s.get(),
        Outcome::Err(_) => panic!("resolve ext_entries"),
    };
    let theirs = Str::__make(Bits::<28, Hot>::from_raw(mint()));
    println!(
        "host section: {} entries; extension section: {} entries",
        static_entries().len(),
        entries()
    );
    let got = match interner.resolve(theirs) {
        Maybe::Is(t) => t,
        Maybe::Isnt => "(nothing)",
    };
    println!(
        "host id {} -> {:?}; extension id {} -> {:?}",
        own.id().to_raw(),
        "host.kind.pos",
        mint(),
        got
    );
    assert!(
        matches!(interner.resolve(theirs), Maybe::Is("mod.kind.heat")),
        "a name the extension declared with str_const! does not resolve in the host"
    );
}

/// A host arena that actually stores strings, for the case below.
struct VecArena(RefCell<Vec<&'static str>>);
impl ArenaInterner for VecArena {
    fn arena_intern(&self, s: &str) -> u32 {
        let leaked: &'static str = Box::leak(s.to_owned().into_boxed_str());
        let mut v = self.0.borrow_mut();
        v.push(leaked);
        (v.len() - 1) as u32
    }

    fn arena_resolve(&self, id: u32) -> &str {
        self.0.borrow()[id as usize]
    }
}

/// The second way the seam splits a name: the host meets the extension's
/// name as text (from its content, say) and interns it. The host's const
/// table does not hold it, so it goes to the arena and gets a run-time
/// handle; the extension holds a const handle for the same text. One name,
/// two handles, and equality on Str says they differ.
#[test]
fn one_name_has_one_handle_on_both_sides_of_the_seam() {
    let lib = open();
    let mint: Mint = match lib.resolve::<Mint>(b"ext_mint\0") {
        Outcome::Ok(s) => s.get(),
        Outcome::Err(_) => panic!("resolve ext_mint"),
    };
    let theirs = Str::__make(Bits::<28, Hot>::from_raw(mint()));
    let interner = StringInterner::new(VecArena(RefCell::new(Vec::new())));
    let ours = interner.intern("mod.kind.heat");
    println!(
        "host intern of the same text: const {} id {}; extension handle: const {} id {}",
        ours.is_const().0,
        ours.id().to_raw(),
        theirs.is_const().0,
        theirs.id().to_raw()
    );
    assert!(
        ours == theirs,
        "the host and the extension hold two different handles for one name"
    );
}
