//! Sketch: store-backed flat-CSR execution plan.
//!
//! De-risks the store-backed-plan SRC (round 202605302330) before the
//! entangled cut. Validates four things the topic flagged as open:
//!
//! 1. Flat CSR dissolves the 11 MB nested tree. A faithful reconstruction
//!    of the current `[Phase;32]->[Trunk;32]->[TrunkComponent;32]xFiber`
//!    nesting measures multi-MB; the flat-CSR handle measures bytes.
//! 2. The plan becomes a small `Copy` handle (meta-`StoreId`s + live
//!    counts) over a `ColumnStorage`. The handle carries no data, no
//!    lifetime.
//! 3. The step chain writes flat scratch (~tens of KB on the stack), then
//!    a two-pass reserve-by-live-count + copy lands the columns in the
//!    store. Reads go through `column_ptr` by `StoreId`.
//! 4. A `Scheduler`-shaped owner of a `!Send`/`!Sync` store is `Send`/`Sync`
//!    via a documented frozen-between-commit-and-replan invariant.
//!
//! Stubbed (std bin, no path deps, no feature gates): `StoreId`, the
//! `ColumnStorage` contract, and a heap-backed `ArenaStore` standing in for
//! the real no_std `ArenaColumnStorage<M>` over a `MemoryProvider`. The
//! structural questions (shape, handle threading, reserve/copy, Send/Sync)
//! are container-agnostic, so the stub is faithful enough to settle them.

use std::alloc::{alloc, dealloc, Layout};

// ---------------------------------------------------------------------------
// Stubbed substrate: StoreId + the ColumnStorage contract (mirrors the real
// hilavitkutin-api shapes; see hilavitkutin-api/src/storage.rs).
// ---------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct StoreId(usize);

/// Mirrors `hilavitkutin_api::ColumnStorage`: reserve a column by element
/// count, hand back raw bases, report the count, release. Raw-pointer access
/// (R6): a slice borrow alongside a resource pointer would defeat noalias.
trait ColumnStorage {
    fn reserve<T: Copy + 'static>(&mut self, id: StoreId, count: usize);
    /// # Safety
    /// `id` must name a column reserved for `T` with at least the read count.
    unsafe fn column_ptr<T>(&self, id: StoreId) -> *const T;
    /// # Safety
    /// `id` must name a column reserved for `T` with at least the write count.
    unsafe fn column_ptr_mut<T>(&self, id: StoreId) -> *mut T;
    fn count(&self, id: StoreId) -> usize;
}

/// Heap-backed stand-in for the no_std `ArenaColumnStorage<M>`. The real one
/// allocates through a `MemoryProvider`; here `std::alloc` plays that role.
/// The raw `*mut u8` in the slot table makes this `!Send`/`!Sync`, exactly
/// like the real arena (raw provider pointers). That is the property the
/// Send/Sync experiment below depends on.
struct ArenaStore {
    // index by StoreId.0: (base, byte_len, align, count)
    slots: Vec<Option<Slot>>,
}

#[derive(Copy, Clone)]
struct Slot {
    base: *mut u8,
    bytes: usize,
    align: usize,
    count: usize,
}

impl ArenaStore {
    fn new(max_columns: usize) -> Self {
        Self { slots: vec![None; max_columns] }
    }
}

impl ColumnStorage for ArenaStore {
    fn reserve<T: Copy + 'static>(&mut self, id: StoreId, count: usize) {
        let size = core::mem::size_of::<T>();
        let align = core::mem::align_of::<T>().max(64); // 64-byte line, like the real arena
        // checked_mul guard (the real soundness fix from PR #112 round 3).
        let bytes = count.checked_mul(size).expect("length overflow") .max(1);
        // free a prior reservation on the same id before re-reserving.
        if let Some(prev) = self.slots[id.0].take() {
            // SAFETY: prev came from this allocator with these params.
            unsafe { dealloc(prev.base, Layout::from_size_align(prev.bytes, prev.align).unwrap()); }
        }
        let layout = Layout::from_size_align(bytes, align).unwrap();
        // SAFETY: layout is non-zero (bytes >= 1).
        let base = unsafe { alloc(layout) };
        assert!(!base.is_null(), "alloc failed");
        self.slots[id.0] = Some(Slot { base, bytes, align, count });
    }

    unsafe fn column_ptr<T>(&self, id: StoreId) -> *const T {
        self.slots[id.0].as_ref().expect("unreserved column").base as *const T
    }

    unsafe fn column_ptr_mut<T>(&self, id: StoreId) -> *mut T {
        self.slots[id.0].as_ref().expect("unreserved column").base as *mut T
    }

    fn count(&self, id: StoreId) -> usize {
        self.slots[id.0].as_ref().map(|s| s.count).unwrap_or(0)
    }
}

impl Drop for ArenaStore {
    fn drop(&mut self) {
        for slot in self.slots.iter().flatten() {
            // SAFETY: each live slot came from this allocator with these params.
            unsafe { dealloc(slot.base, Layout::from_size_align(slot.bytes, slot.align).unwrap()); }
        }
    }
}

// ---------------------------------------------------------------------------
// PART 1: the dissolve. A faithful reconstruction of the CURRENT nested plan
// at the default dims, measured against the flat-CSR shape.
// ---------------------------------------------------------------------------

// Default dims (DefaultPlanDims): Phases=32, TrunksPerPhase=32,
// ComponentsPerTrunk=32, UnitsPerFiber=32, ColumnsPerFiber=16.
const PHASES: usize = 32;
const TRUNKS_PER_PHASE: usize = 32;
const COMPONENTS_PER_TRUNK: usize = 32;
const UNITS_PER_FIBER: usize = 32;
const COLUMNS_PER_FIBER: usize = 16;

// Leaf scalar widths matching the real arvo newtypes (repr-transparent).
type UnitIdW = u16; // UnitId = Uint<16>
type StoreIdW = u64; // StoreId(USize)
type FiberIdW = u16;
type TrunkIdW = u16;

#[derive(Copy, Clone, Default)]
#[allow(dead_code)]
struct AccumSlotN { store_id: StoreIdW, accum_type: u8 }
#[derive(Copy, Clone, Default)]
#[allow(dead_code)]
struct HeadTailN { head: AccumSlotN, tail: AccumSlotN, merge_target: AccumSlotN, merge_op: u8 }

#[derive(Copy, Clone)]
#[allow(dead_code)]
struct FiberN {
    id: FiberIdW,
    units: [UnitIdW; UNITS_PER_FIBER],
    unit_count: u64,
    columns: [StoreIdW; COLUMNS_PER_FIBER],
    column_count: u64,
    head_tail: Option<HeadTailN>,
    dispatch_approach: u8,
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
enum TrunkComponentN { Fiber(FiberN), Branch(u64), Bridge(u64) }

#[derive(Copy, Clone)]
#[allow(dead_code)]
struct TrunkN {
    id: TrunkIdW,
    components: [TrunkComponentN; COMPONENTS_PER_TRUNK],
    component_count: u64,
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
struct PhaseN {
    id: u8,
    trunks: [TrunkN; TRUNKS_PER_PHASE],
    trunk_count: u64,
    strategy: u8,
    config: u8,
}

// The current ExecutionPlan's dominant field, reconstructed.
#[allow(dead_code)]
struct ExecutionPlanNested {
    phases: [PhaseN; PHASES],
    phase_count: u64,
    // (unit_meta / column_class / dirty / morsel_sizes / rcm_order omitted;
    // they are a few KB combined, not the dominant term.)
}

// ---------------------------------------------------------------------------
// The flat-CSR shape. Plan columns are flat scalar arrays; the tree is
// expressed by offset/count indices (CSR). These are the SCRATCH arrays the
// step chain fills; the store-backed columns mirror them at live count.
// ---------------------------------------------------------------------------

// Plan-wide caps (DefaultPlanDims): Fibers=64, Trunks=64, Phases=32.
const FIBERS: usize = 64;
const TRUNKS: usize = 64;
// Flat CSR fan-out caps: total fiber-unit and fiber-column slots across the
// whole plan. Each unit lands in one fiber => Units(64); columns bounded by
// Fibers*ColumnsPerFiber but realistically by total distinct refs; cap at a
// flat budget.
const FIBER_UNIT_SLOTS: usize = 64; // = Units
const FIBER_COLUMN_SLOTS: usize = FIBERS * COLUMNS_PER_FIBER; // 1024

#[derive(Copy, Clone, Default)]
struct FlatPhase { id: u8, trunk_offset: u32, trunk_count: u32, strategy: u8, config: u8 }
#[derive(Copy, Clone, Default)]
struct FlatTrunk { id: TrunkIdW, fiber_offset: u32, fiber_count: u32 }
#[derive(Copy, Clone, Default)]
struct FlatFiber {
    id: FiberIdW,
    unit_offset: u32,
    unit_count: u32,
    col_offset: u32,
    col_count: u32,
    // head_tail kept inline as a flag + a parallel column would carry the
    // convergence; for the sketch, a flag suffices to prove the shape.
    has_head_tail: u8,
    dispatch_approach: u8,
}

/// Flat CSR scratch: what `compute_execution_plan` builds on the stack
/// instead of the nested tree. Sized by plan-wide caps.
struct FlatScratch {
    phases: [FlatPhase; PHASES],
    trunks: [FlatTrunk; TRUNKS],
    fibers: [FlatFiber; FIBERS],
    fiber_units: [UnitIdW; FIBER_UNIT_SLOTS],
    fiber_columns: [StoreIdW; FIBER_COLUMN_SLOTS],
    phase_count: usize,
    trunk_count: usize,
    fiber_count: usize,
    fiber_unit_count: usize,
    fiber_column_count: usize,
}

impl FlatScratch {
    fn new() -> Self {
        Self {
            phases: [FlatPhase::default(); PHASES],
            trunks: [FlatTrunk::default(); TRUNKS],
            fibers: [FlatFiber::default(); FIBERS],
            fiber_units: [0; FIBER_UNIT_SLOTS],
            fiber_columns: [0; FIBER_COLUMN_SLOTS],
            phase_count: 0,
            trunk_count: 0,
            fiber_count: 0,
            fiber_unit_count: 0,
            fiber_column_count: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// The handle. Tiny, Copy, no lifetime. Names its columns by a FIXED meta
// StoreId enumeration plus the live counts. (Fixed enumeration chosen over
// dynamic ids: plan columns are a closed set, so a const namespace beats
// threading ids through.)
// ---------------------------------------------------------------------------

mod plan_col {
    use super::StoreId;
    // A reserved meta-StoreId namespace. Real impl: a high base offset above
    // any consumer-declared store id, or a separate meta ColumnStorage.
    pub const PHASES: StoreId = StoreId(0);
    pub const TRUNKS: StoreId = StoreId(1);
    pub const FIBERS: StoreId = StoreId(2);
    pub const FIBER_UNITS: StoreId = StoreId(3);
    pub const FIBER_COLUMNS: StoreId = StoreId(4);
    pub const COUNT: usize = 5;
}

#[derive(Copy, Clone, Default)]
struct PlanHandle {
    phase_count: u32,
    trunk_count: u32,
    fiber_count: u32,
    fiber_unit_count: u32,
    fiber_column_count: u32,
}

// ---------------------------------------------------------------------------
// The step chain (mini). Builds flat scratch, NOT the nested tree. This
// mirrors `project_fiber_components` writing flat CSR instead of [[Trunk]].
// The point is to prove the flat-write pattern is expressible; the real chain
// keeps its algorithm and only changes its output target.
// ---------------------------------------------------------------------------

fn build_flat_scratch(unit_count: usize) -> FlatScratch {
    let mut s = FlatScratch::new();
    // Toy plan: one phase, one trunk, ceil(unit_count/8) fibers of <=8 units.
    s.phase_count = if unit_count == 0 { 0 } else { 1 };
    s.trunk_count = if unit_count == 0 { 0 } else { 1 };

    let mut fiber_unit_cursor = 0usize;
    let mut fid = 0usize;
    let mut u = 0usize;
    while u < unit_count && fid < FIBERS {
        let take = (unit_count - u).min(8);
        let unit_offset = fiber_unit_cursor as u32;
        for k in 0..take {
            if fiber_unit_cursor < FIBER_UNIT_SLOTS {
                s.fiber_units[fiber_unit_cursor] = (u + k) as UnitIdW;
                fiber_unit_cursor += 1;
            }
        }
        // one column ref per fiber for the toy.
        let col_offset = s.fiber_column_count as u32;
        if s.fiber_column_count < FIBER_COLUMN_SLOTS {
            s.fiber_columns[s.fiber_column_count] = fid as StoreIdW;
            s.fiber_column_count += 1;
        }
        s.fibers[fid] = FlatFiber {
            id: fid as FiberIdW,
            unit_offset,
            unit_count: take as u32,
            col_offset,
            col_count: 1,
            has_head_tail: 0,
            dispatch_approach: 0,
        };
        fid += 1;
        u += take;
    }
    s.fiber_count = fid;
    s.fiber_unit_count = fiber_unit_cursor;

    // single trunk spans all fibers; single phase spans the trunk.
    if s.trunk_count > 0 {
        s.trunks[0] = FlatTrunk { id: 0, fiber_offset: 0, fiber_count: s.fiber_count as u32 };
        s.phases[0] = FlatPhase { id: 0, trunk_offset: 0, trunk_count: 1, strategy: 1, config: 1 };
    }
    s
}

/// Two-pass assembly: the scratch already holds live counts (built as the
/// chain ran), so pass 1 reserves each column by its live count, pass 2
/// copies the scratch prefix in. Returns the handle.
fn assemble_into_store<CS: ColumnStorage>(store: &mut CS, s: &FlatScratch) -> PlanHandle {
    // pass 1: reserve by live count.
    store.reserve::<FlatPhase>(plan_col::PHASES, s.phase_count);
    store.reserve::<FlatTrunk>(plan_col::TRUNKS, s.trunk_count);
    store.reserve::<FlatFiber>(plan_col::FIBERS, s.fiber_count);
    store.reserve::<UnitIdW>(plan_col::FIBER_UNITS, s.fiber_unit_count);
    store.reserve::<StoreIdW>(plan_col::FIBER_COLUMNS, s.fiber_column_count);

    // pass 2: copy the scratch prefixes into the reserved columns.
    // SAFETY: each column was reserved for the matching T with the live count;
    // we copy exactly that many records from the scratch prefix.
    unsafe {
        copy_prefix(store.column_ptr_mut::<FlatPhase>(plan_col::PHASES), &s.phases, s.phase_count);
        copy_prefix(store.column_ptr_mut::<FlatTrunk>(plan_col::TRUNKS), &s.trunks, s.trunk_count);
        copy_prefix(store.column_ptr_mut::<FlatFiber>(plan_col::FIBERS), &s.fibers, s.fiber_count);
        copy_prefix(store.column_ptr_mut::<UnitIdW>(plan_col::FIBER_UNITS), &s.fiber_units, s.fiber_unit_count);
        copy_prefix(store.column_ptr_mut::<StoreIdW>(plan_col::FIBER_COLUMNS), &s.fiber_columns, s.fiber_column_count);
    }

    PlanHandle {
        phase_count: s.phase_count as u32,
        trunk_count: s.trunk_count as u32,
        fiber_count: s.fiber_count as u32,
        fiber_unit_count: s.fiber_unit_count as u32,
        fiber_column_count: s.fiber_column_count as u32,
    }
}

/// # Safety
/// `dst` must point to at least `n` writable `T`; `src` must hold at least `n`.
unsafe fn copy_prefix<T: Copy>(dst: *mut T, src: &[T], n: usize) {
    for i in 0..n {
        dst.add(i).write(src[i]);
    }
}

// ---------------------------------------------------------------------------
// PART 4: Scheduler-shaped owner. Holds the !Send/!Sync store + the handle.
// The unsafe Send/Sync impl carries the frozen-between-commit-and-replan
// provenance argument.
// ---------------------------------------------------------------------------

struct Scheduler<CS: ColumnStorage> {
    store: CS,
    plan: PlanHandle,
}

// SAFETY: `Scheduler` owns its `ColumnStorage`. The plan columns are written
// once during `commit`/replan (which take `&mut self`, hence exclusive) and
// are frozen for the duration of a frame: dispatch reads them through shared
// `&self` and never mutates. A `&Scheduler` handed to per-core dispatch
// closures therefore only ever observes immutable, fully-initialised plan
// columns. No interior mutability, no aliasing write. The raw provider
// pointers inside the store are never exposed for mutation across threads;
// the only cross-thread access is the read path over frozen columns.
unsafe impl<CS: ColumnStorage> Send for Scheduler<CS> {}
unsafe impl<CS: ColumnStorage> Sync for Scheduler<CS> {}

impl<CS: ColumnStorage> Scheduler<CS> {
    fn new(store: CS) -> Self {
        Self { store, plan: PlanHandle::default() }
    }

    /// Replan: &mut self (exclusive). Builds flat scratch, reserves + copies
    /// into the owned store, updates the handle.
    fn commit_plan(&mut self, unit_count: usize) {
        let scratch = build_flat_scratch(unit_count);
        self.plan = assemble_into_store(&mut self.store, &scratch);
    }

    /// Read path: &self (shared). Walk the store-backed plan by StoreId.
    fn total_fiber_units(&self) -> usize {
        let n = self.plan.fiber_count as usize;
        // SAFETY: FIBERS column reserved for FlatFiber with fiber_count records.
        let fibers = unsafe { self.store.column_ptr::<FlatFiber>(plan_col::FIBERS) };
        let mut total = 0usize;
        for f in 0..n {
            // SAFETY: f < fiber_count, the reserved count.
            let fib = unsafe { *fibers.add(f) };
            total += fib.unit_count as usize;
        }
        total
    }
}

fn assert_send_sync<T: Send + Sync>() {}

fn main() {
    // PART 1: the dissolve.
    let nested = core::mem::size_of::<ExecutionPlanNested>();
    let flat_scratch = core::mem::size_of::<FlatScratch>();
    let handle = core::mem::size_of::<PlanHandle>();
    println!("=== PART 1: dissolve ===");
    println!("nested ExecutionPlan (dominant field): {} bytes ({:.2} MB)", nested, nested as f64 / 1_048_576.0);
    println!("flat CSR scratch (stack, all plan-wide caps): {} bytes ({:.1} KB)", flat_scratch, flat_scratch as f64 / 1024.0);
    println!("store-backed PlanHandle: {} bytes", handle);
    assert!(nested > 8 * 1_048_576, "nested should be multi-MB");
    assert!(flat_scratch < 64 * 1024, "flat scratch should fit comfortably on the stack");
    assert!(handle <= 32, "handle should be a few words");

    // PART 2+3: build flat scratch, store-back, read through the handle.
    println!("\n=== PART 2+3: store-backed round trip ===");
    let store = ArenaStore::new(plan_col::COUNT);
    let mut sched = Scheduler::new(store);
    let unit_count = 20;
    sched.commit_plan(unit_count);
    println!("plan handle: phases={} trunks={} fibers={} fiber_units={}",
        sched.plan.phase_count, sched.plan.trunk_count, sched.plan.fiber_count, sched.plan.fiber_unit_count);
    // every unit lands in exactly one fiber: total fiber units == unit_count.
    let total = sched.total_fiber_units();
    println!("total fiber units read back from store: {}", total);
    assert_eq!(total, unit_count, "store-backed read must recover every unit");
    // ceil(20/8) = 3 fibers.
    assert_eq!(sched.plan.fiber_count, 3);

    // empty plan is valid (zero counts, zero reserves).
    let store2 = ArenaStore::new(plan_col::COUNT);
    let mut sched2 = Scheduler::new(store2);
    sched2.commit_plan(0);
    assert_eq!(sched2.plan.fiber_count, 0);
    assert_eq!(sched2.total_fiber_units(), 0);
    println!("empty plan: fibers=0 ok");

    // replan reuses the same store (reserve frees prior columns first).
    sched.commit_plan(40);
    assert_eq!(sched.total_fiber_units(), 40);
    assert_eq!(sched.plan.fiber_count, 5); // ceil(40/8)
    println!("replan on same store: 40 units -> {} fibers, recovered {}", sched.plan.fiber_count, sched.total_fiber_units());

    // PART 4: the Scheduler owning a !Send/!Sync store is Send + Sync.
    println!("\n=== PART 4: Send/Sync over a !Send store ===");
    assert_send_sync::<Scheduler<ArenaStore>>();
    // prove the store ITSELF is !Send (negative check via a helper that only
    // compiles for !Send types would need autotraits; instead we just note
    // ArenaStore holds *mut u8, which is !Send/!Sync by construction).
    println!("Scheduler<ArenaStore>: Send + Sync (asserted at compile time)");
    println!("  (ArenaStore holds *mut u8 => !Send/!Sync; Scheduler lifts it via the frozen-plan invariant)");

    println!("\nWORKS: flat-CSR store-backed plan threads through, dissolves the monolith, Send/Sync holds.");
}
