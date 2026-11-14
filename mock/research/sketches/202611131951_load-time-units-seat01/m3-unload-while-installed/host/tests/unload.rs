//! Seat 01 reply, merged call M3. Question: when a library is dropped while
//! a compiled host unit still holds its kernel as an occupant, what happens
//! on the trunk engine, and which shapes keep every later frame sound?
//!
//! Each scenario that touches an unmapped image runs in a child process (this
//! test binary re-executed with `M3_CHILD` naming the scenario), so a fault
//! kills the child and the parent reports how it died.
//!
//! - `every_frame_after_the_drop_is_sound_with_the_occupant_left_installed`:
//!   the contract asserted against the bare-resource arm (M3 a with nothing
//!   enforcing the rule). Expected red: the child dies on the first frame
//!   that reruns the host unit after the drop. The clean frame between the
//!   drop and that one survives, because the engine skips a unit whose inputs
//!   did not change, so the hazard is latent until something marks a store.
//! - `the_installer_swaps_before_it_drops_and_every_frame_after_is_sound`:
//!   the host-side owner, `Installed::uninstall`. Expected green.
//! - `an_installer_dropped_without_uninstall_keeps_the_image_and_stays_sound`:
//!   the owner's other exit. Expected green, with the image still mapped.
//! - `an_occupant_swap_needs_a_dirty_seed_and_no_plan_band`: a `Replaceable`
//!   occupant swapped with `replace_value` reruns the host and leaves the plan
//!   band shut. Expected green. In process; no library involved.

use std::cell::RefCell;
use std::io::Write as _;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Output};

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapCons, SnapNil};
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
    HasResourceProvider,
    ResourceProviderApi,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Column, Resource};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use m3_host::{
    Installed,
    Occupant,
    ext_expected,
    host_double,
    host_fallback,
    is_mapped,
    kernel_of,
    open_fresh,
};
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
struct A(i64);
#[derive(Copy, Clone)]
struct B(i64);

type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);
const N: usize = 20_000;

thread_local! {
    static SEEN: RefCell<Vec<(i64, i64)>> = const { RefCell::new(Vec::new()) };
    static HOST_RUNS: RefCell<u32> = const { RefCell::new(0) };
}

struct Loader;
impl BuilderInput for Loader {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Loader {
    type Ctx<'f> =
        EngineCtx<'f, Empty, Cons<Column<A>, Empty>, SnapNil, ColPtrNil, ColPtrCons<A, ColPtrNil>>;
    type Hint = Hints;
    type Read = Empty;
    type Write = Cons<Column<A>, Empty>;

    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        EachApi::run(ctx.each(), |i| unsafe {
            ctx.writer().write::<A, _>(i, A((i.0 as i64 % 97) - 40));
        });
    }
}

type HostRead = Cons<Resource<Occupant>, Cons<Column<A>, Empty>>;
type HostWrite = Cons<Column<B>, Empty>;

/// The compiled host unit: access set fixed at build, kernel read from the
/// occupant resource every time it runs, called once a morsel.
struct Host;
impl BuilderInput for Host {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Host {
    type Ctx<'f> = EngineCtx<
        'f,
        HostRead,
        HostWrite,
        SnapCons<Occupant, SnapNil>,
        ColPtrCons<A, ColPtrNil>,
        ColPtrCons<B, ColPtrNil>,
    >;
    type Hint = Hints;
    type Read = HostRead;
    type Write = HostWrite;

    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        HOST_RUNS.with(|c| *c.borrow_mut() += 1);
        let occ: &Occupant = ctx.resources().resource();
        let kernel = occ.0;
        BatchApi::run(ctx.batch(), |_, len| {
            let n = len.0;
            // The trunk context has no window accessor (context.rs:117, :134):
            // copy in, call once, copy out. Scaffolding, not part of the claim.
            let a: Vec<i64> = (0 .. n)
                .map(|i| unsafe { ctx.reader().read::<A, _>(USize(i)).0 })
                .collect();
            let mut out = vec![0i64; n];
            kernel(a.as_ptr(), out.as_mut_ptr(), n);
            for (i, v) in out.iter().enumerate() {
                unsafe { ctx.writer().write::<B, _>(USize(i), B(*v)) };
            }
        });
    }
}

struct Observer;
impl BuilderInput for Observer {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
type ObsRead = Cons<Column<A>, Cons<Column<B>, Empty>>;
impl WorkUnit<Always> for Observer {
    type Ctx<'f> =
        EngineCtx<'f, ObsRead, Empty, SnapNil, ColPtrCons<A, ColPtrCons<B, ColPtrNil>>, ColPtrNil>;
    type Hint = Hints;
    type Read = ObsRead;
    type Write = Empty;

    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        EachApi::run(ctx.each(), |i| {
            let r = ctx.reader();
            let t = unsafe { (r.read::<A, _>(i).0, r.read::<B, _>(i).0) };
            SEEN.with(|s| s.borrow_mut().push(t));
        });
    }
}

fn seen_take() -> Vec<(i64, i64)> {
    SEEN.with(|s| std::mem::take(&mut *s.borrow_mut()))
}

fn host_runs() -> u32 {
    HOST_RUNS.with(|c| *c.borrow())
}

macro_rules! scheduler {
    ($first:expr) => {
        Scheduler::builder()
            .with(Resource::new($first))
            .with(Column::<A>::new())
            .with(Column::<B>::new())
            .with(Loader)
            .with(Host)
            .with(Observer)
            .build(store(ZeroedHeap), USize(N))
            .unwrap_or_else(|_| panic!("build"))
    };
}

fn mark(line: &str) {
    println!("M3: {line}");
    std::io::stdout().flush().unwrap();
}

fn check_frame(label: &str, f: impl Fn(i64) -> i64) {
    let seen = seen_take();
    assert_eq!(seen.len(), N, "{label}: observer saw every record");
    assert!(
        seen.iter().all(|&(a, b)| b == f(a)),
        "{label}: every record is the expected kernel's"
    );
    mark(&format!("{label}: {} records, all match", seen.len()));
}

/// Run `body` in a child process when this is the child, else spawn the
/// child and return its output.
fn in_child(name: &str, body: fn()) -> Output {
    if std::env::var("M3_CHILD").as_deref() == Ok(name) {
        body();
        mark("child body finished");
        std::process::exit(0);
    }
    Command::new(std::env::current_exe().unwrap())
        .args(["--exact", name, "--nocapture", "--test-threads=1"])
        .env("M3_CHILD", name)
        .output()
        .unwrap()
}

fn report(name: &str, out: &Output) {
    let stdout = String::from_utf8_lossy(&out.stdout);
    for line in stdout
        .lines()
        .filter_map(|l| l.find("M3: ").map(|at| &l[at ..]))
    {
        println!("[{name}] {line}");
    }
    println!(
        "[{name}] child exit: code {:?}, signal {:?}",
        out.status.code(),
        out.status.signal()
    );
}

// Scenario 1: the library is dropped while its kernel is still installed.
fn drop_under_installed() {
    let mut s = scheduler!(Occupant(host_fallback));
    assert!(matches!(s.run(), Outcome::Ok(())));
    check_frame("frame 1, fallback", |a| a - 1000);

    // A library loaded after build, its kernel installed.
    let lib = open_fresh();
    s.replace_value(Occupant(kernel_of(&lib)));
    assert!(matches!(s.run(), Outcome::Ok(())));
    check_frame("frame 2, extension installed", ext_expected);

    // The host drops the library and forgets the occupant.
    drop(lib);
    assert!(!is_mapped(), "the image is unmapped once the Library drops");
    mark("library dropped, image unmapped, occupant still installed");

    let before = host_runs();
    assert!(matches!(s.run(), Outcome::Ok(())));
    mark(&format!(
        "frame 3, clean: survived, host runs {} -> {}",
        before,
        host_runs()
    ));
    let _ = seen_take();

    // Anything that reruns the host: a store it reads is marked.
    s.mark_dirty::<Column<A>, _>();
    mark("frame 4, Column<A> marked dirty: running");
    assert!(matches!(s.run(), Outcome::Ok(())));
    mark("frame 4 survived");
}

#[test]
fn every_frame_after_the_drop_is_sound_with_the_occupant_left_installed() {
    let name = "every_frame_after_the_drop_is_sound_with_the_occupant_left_installed";
    let out = in_child(name, drop_under_installed);
    report(name, &out);
    assert!(
        out.status.success(),
        "a frame after the library was dropped under an installed occupant killed the process (signal {:?})",
        out.status.signal()
    );
}

// Scenario 2: the host-side owner swaps the fallback in before it drops.
fn installer_swaps_then_drops() {
    let mut s = scheduler!(Occupant(host_fallback));
    assert!(matches!(s.run(), Outcome::Ok(())));
    check_frame("frame 1, fallback", |a| a - 1000);

    let inst = Installed::load();
    s.replace_value(inst.occupant());
    assert!(matches!(s.run(), Outcome::Ok(())));
    check_frame("frame 2, extension installed", ext_expected);

    inst.uninstall(|fallback| s.replace_value(fallback));
    assert!(!is_mapped(), "uninstall released the library");
    mark("uninstalled: fallback swapped in, then the image unmapped");

    assert!(matches!(s.run(), Outcome::Ok(())));
    check_frame("frame 3, after uninstall", |a| a - 1000);

    s.mark_dirty::<Column<A>, _>();
    assert!(matches!(s.run(), Outcome::Ok(())));
    check_frame("frame 4, Column<A> marked dirty", |a| a - 1000);
}

#[test]
fn the_installer_swaps_before_it_drops_and_every_frame_after_is_sound() {
    let name = "the_installer_swaps_before_it_drops_and_every_frame_after_is_sound";
    let out = in_child(name, installer_swaps_then_drops);
    report(name, &out);
    assert!(
        out.status.success(),
        "child failed: signal {:?}",
        out.status.signal()
    );
}

// Scenario 3: the owner is dropped without uninstall.
fn installer_dropped_without_uninstall() {
    let mut s = scheduler!(Occupant(host_fallback));
    assert!(matches!(s.run(), Outcome::Ok(())));
    let _ = seen_take();

    let inst = Installed::load();
    s.replace_value(inst.occupant());
    assert!(matches!(s.run(), Outcome::Ok(())));
    check_frame("frame 2, extension installed", ext_expected);

    drop(inst);
    assert!(
        is_mapped(),
        "an owner dropped without uninstall leaks the mapping"
    );
    mark("owner dropped without uninstall: image still mapped");

    s.mark_dirty::<Column<A>, _>();
    assert!(matches!(s.run(), Outcome::Ok(())));
    check_frame("frame 3, Column<A> marked dirty", ext_expected);
}

#[test]
fn an_installer_dropped_without_uninstall_keeps_the_image_and_stays_sound() {
    let name = "an_installer_dropped_without_uninstall_keeps_the_image_and_stays_sound";
    let out = in_child(name, installer_dropped_without_uninstall);
    report(name, &out);
    assert!(
        out.status.success(),
        "child failed: signal {:?}",
        out.status.signal()
    );
}

#[test]
fn an_occupant_swap_needs_a_dirty_seed_and_no_plan_band() {
    seen_take();
    HOST_RUNS.with(|c| *c.borrow_mut() = 0);
    let mut s = scheduler!(Occupant(host_fallback));

    assert!(matches!(s.run(), Outcome::Ok(())));
    assert!(s.__plan_band().0, "the cold frame enters the plan band");
    check_frame("cold frame", |a| a - 1000);

    assert!(matches!(s.run(), Outcome::Ok(())));
    assert!(!s.__plan_band().0, "a clean frame does not");
    let r2 = host_runs();
    let _ = seen_take();

    s.replace_value(Occupant(host_double));
    assert!(matches!(s.run(), Outcome::Ok(())));
    let r3 = host_runs();
    println!(
        "occupant swap by replace_value: host runs {r2} -> {r3}, plan band {}",
        s.__plan_band().0
    );
    assert!(r3 > r2, "the host reran after the swap");
    assert!(
        !s.__plan_band().0,
        "a Replaceable occupant swap does not open the plan band"
    );
    check_frame("frame after the swap", |a| a * 2);
}
