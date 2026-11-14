//! Integration test for `plan_inputs_from_bundle`: the per-work-unit
//! bundle walk projected against the REAL `WorkUnit` / `AccessSet` /
//! `PlanInputs` contract (not the standalone scratch's stand-in types).
//!
//! Two work units with overlapping access sets exercise the per-unit
//! read/write/access mask fill, the `unit_count` accumulation, the
//! commutativity copy, and the nested per-unit witness inference. The
//! dependency edge (W0 writes a store W1 reads) is asserted via the
//! shared bit position.

// `PlanInputs<CU, CS>` and the projection are now sized by the `Capacity`
// TYPE (`Dim<N>`), so no `generic_const_exprs` gate is needed: the unit and
// store capacities are types, not `Cap` const generics.

use arvo::{Bool, USize};
use arvo_tensor::Dim;
use hilavitkutin::plan::{AccessMask, MorselBudget, PlanInputs, plan_inputs_from_bundle};
use hilavitkutin_api::meta::{RANK_CONSUMER, RANK_PASS_START, RANK_SCHEDULE_END};
use hilavitkutin_api::run_cfg::{PassStart, ScheduleEnd};
use hilavitkutin_api::work_unit::{HasSchedule, OnMeta};
use hilavitkutin_api::{
    AccessSet,
    Always,
    Atomic,
    BatchApi,
    Buffered,
    BuilderInput,
    Column,
    ColumnReaderApi,
    ColumnValue,
    ColumnWriterApi,
    Cons,
    EachApi,
    Empty,
    HasBatch,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
    HasReduce,
    HasResourceProvider,
    HasVirtualFirer,
    Holds,
    Immediate,
    Normal,
    ReduceApi,
    ResolveColumnRead,
    ResolveColumnWrite,
    ResolveResource,
    ResolveVirtualFire,
    Resource,
    ResourceProviderApi,
    UnitDispatch,
    Virtual,
    VirtualFirerApi,
    WorkUnit,
    read,
    write,
};

type MaxUnits = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test unit capacity; Dim<N> array-length root; tracked: #649
type MaxStores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test store capacity; Dim<N> array-length root; tracked: #649

// ---------------------------------------------------------------------
// Permissive Ctx + provider shim (same shape as scheduler_builder.rs):
// the projection never runs `execute`, but the `WorkUnit` contract
// requires a `Ctx` GAT bound by the full `Has*` family, so the fixture
// satisfies them generically.
// ---------------------------------------------------------------------

struct Stub;

impl<R: AccessSet> ColumnReaderApi<R> for Stub {
    unsafe fn read<T: ColumnValue, I>(&self, _i: USize) -> T
    where
        R: Holds<Column<T>>,
        Self: ResolveColumnRead<T, I>,
    {
        unimplemented!()
    }
}
impl<T: ColumnValue, I> ResolveColumnRead<T, I> for Stub {
    unsafe fn resolve_read(&self, _i: USize) -> T {
        unimplemented!()
    }
}
impl<W: AccessSet> ColumnWriterApi<W> for Stub {}
impl<T: ColumnValue, I> ResolveColumnWrite<T, I> for Stub {
    unsafe fn resolve_write(&self, _i: USize, _v: T) {}

    unsafe fn resolve_held(&self, _i: USize) -> T {
        unimplemented!()
    }
}
impl<R: AccessSet> ResourceProviderApi<R> for Stub {
    fn resource<T: 'static, I>(&self) -> &T
    where
        R: Holds<Resource<T>>,
        Self: ResolveResource<T, I>,
    {
        unimplemented!()
    }
}
impl<T: 'static, I> ResolveResource<T, I> for Stub {
    fn resolve_resource(&self) -> &T {
        unimplemented!()
    }
}
impl<W: AccessSet> VirtualFirerApi<W> for Stub {
    fn fire<V: 'static, I>(&self)
    where
        W: Holds<Virtual<V>>,
        Self: ResolveVirtualFire<V, I>,
    {
    }
}
impl<V, I> ResolveVirtualFire<V, I> for Stub {
    fn resolve_fire(&self) {}
}
impl<R: AccessSet, W: AccessSet> EachApi<R, W> for Stub {
    fn run<F>(&self, _f: F)
    where
        F: FnMut(USize),
    {
    }
}
impl<R: AccessSet, W: AccessSet> BatchApi<R, W> for Stub {
    fn run<F>(&self, _f: F)
    where
        F: FnMut(USize, USize),
    {
    }
}
impl<R: AccessSet, W: AccessSet> ReduceApi<R, W> for Stub {
    fn run<A, F>(&self, init: A, _f: F) -> A
    where
        A: 'static,
        F: FnMut(A, USize) -> A,
    {
        init
    }
}

struct TestCtx {
    p: Stub,
}
impl<R: AccessSet> HasColumnReader<R> for TestCtx {
    type Provider = Stub;

    fn reader(&self) -> &Stub {
        &self.p
    }
}
impl<W: AccessSet> HasColumnWriter<W> for TestCtx {
    type Provider = Stub;

    fn writer(&self) -> &Stub {
        &self.p
    }
}
impl<R: AccessSet> HasResourceProvider<R> for TestCtx {
    type Provider = Stub;

    fn resources(&self) -> &Stub {
        &self.p
    }
}
impl<W: AccessSet> HasVirtualFirer<W> for TestCtx {
    type Provider = Stub;

    fn virtuals(&self) -> &Stub {
        &self.p
    }
}
impl<R: AccessSet, W: AccessSet> HasEach<R, W> for TestCtx {
    type Provider = Stub;

    fn each(&self) -> &Stub {
        &self.p
    }
}
impl<R: AccessSet, W: AccessSet> HasBatch<R, W> for TestCtx {
    type Provider = Stub;

    fn batch(&self) -> &Stub {
        &self.p
    }
}
impl<R: AccessSet, W: AccessSet> HasReduce<R, W> for TestCtx {
    type Provider = Stub;

    fn reduce(&self) -> &Stub {
        &self.p
    }
}

// ---------------------------------------------------------------------
// Store markers + global store list. Position in `Stores` is the bit
// index: Resource<RA>=0, Column<CX>=1, Column<CY>=2.
// ---------------------------------------------------------------------

#[derive(Copy, Clone)]
struct RA;
#[derive(Copy, Clone)]
struct CX;
#[derive(Copy, Clone)]
struct CY;

type Stores = Cons<Resource<RA>, Cons<Column<CX>, Cons<Column<CY>, Empty>>>;

// W0 reads RA, writes CX. Not commutative (default).
struct W0;
impl BuilderInput for W0 {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for W0 {
    type Ctx<'frame> = TestCtx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = read![Resource<RA>];
    type Write = write![Column<CX>];

    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

// W1 reads CX (the store W0 writes => dep edge), writes CY. Commutative.
struct W1;
impl BuilderInput for W1 {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for W1 {
    type Ctx<'frame> = TestCtx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = read![Column<CX>];
    type Write = write![Column<CY>];

    const COMMUTATIVE: Bool = Bool::TRUE;

    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

type Wus = Cons<W0, Cons<W1, Empty>>;

const RA_BIT: USize = USize(0); // lint:allow(no-bare-numeric) reason: store bit index; tracked: #121
const CX_BIT: USize = USize(1); // lint:allow(no-bare-numeric) reason: store bit index; tracked: #121
const CY_BIT: USize = USize(2); // lint:allow(no-bare-numeric) reason: store bit index; tracked: #121
const RECORDS: USize = USize(1000); // lint:allow(no-bare-numeric) reason: test record count; tracked: #121

#[test]
fn projects_two_unit_bundle_to_plan_inputs() {
    let inputs: PlanInputs<MaxUnits, MaxStores> =
        plan_inputs_from_bundle::<Wus, Stores, _, MaxUnits, MaxStores>(RECORDS, MorselBudget::ZERO);

    assert_eq!(inputs.unit_count, USize(2), "two units populated"); // lint:allow(no-bare-numeric) reason: expected count; tracked: #121
    assert_eq!(
        inputs.record_count, RECORDS,
        "record_count threaded through"
    );

    let reads = inputs.reads.as_ref();
    let writes = inputs.writes.as_ref();
    let access = inputs.access.as_ref();
    let commutative = inputs.commutative.as_ref();

    // W0 at index 0: reads {RA}, writes {CX}, access {RA, CX}, not commutative.
    let r0 = &reads[0];
    let w0 = &writes[0];
    let a0 = &access[0];
    assert_eq!(r0.contains(RA_BIT), Bool::TRUE, "W0 reads RA");
    assert_eq!(r0.contains(CX_BIT), Bool::FALSE, "W0 does not read CX");
    assert_eq!(w0.contains(CX_BIT), Bool::TRUE, "W0 writes CX");
    assert_eq!(a0.contains(RA_BIT), Bool::TRUE, "W0 access union has RA");
    assert_eq!(a0.contains(CX_BIT), Bool::TRUE, "W0 access union has CX");
    assert_eq!(commutative[0], Bool::FALSE, "W0 not commutative");

    // W1 at index 1: reads {CX}, writes {CY}, commutative.
    let r1 = &reads[1];
    let w1 = &writes[1];
    let a1 = &access[1];
    assert_eq!(r1.contains(CX_BIT), Bool::TRUE, "W1 reads CX");
    assert_eq!(w1.contains(CY_BIT), Bool::TRUE, "W1 writes CY");
    assert_eq!(a1.contains(CX_BIT), Bool::TRUE, "W1 access union has CX");
    assert_eq!(a1.contains(CY_BIT), Bool::TRUE, "W1 access union has CY");
    assert_eq!(
        commutative[1],
        Bool::TRUE,
        "W1 commutative override applied"
    );

    // Dependency edge: W0 writes CX (bit 1), W1 reads CX (bit 1) => same bit.
    assert_eq!(
        w0.overlaps(r1),
        Bool::TRUE,
        "W0 write overlaps W1 read on shared store CX (dep edge)"
    );
}

// ---------------------------------------------------------------------
// The frozen reads and the ranks. One unit per band: a setup that writes
// CX, a consumer that reads CX and reads and writes the buffered CZ, and a
// close that reads CX. Buffered<CZ>=0, Column<CX>=1, Column<CY>=2.
// ---------------------------------------------------------------------

#[derive(Copy, Clone)]
struct CZ;

type Banded = Cons<Buffered<CZ>, Cons<Column<CX>, Cons<Column<CY>, Empty>>>;

struct Lead;
impl BuilderInput for Lead {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl HasSchedule for Lead {
    type Sched = OnMeta<PassStart>;
}
impl WorkUnit<OnMeta<PassStart>> for Lead {
    type Ctx<'frame> = TestCtx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Empty;
    type Write = write![Column<CX>];

    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

struct Mid;
impl BuilderInput for Mid {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Mid {
    type Ctx<'frame> = TestCtx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Buffered<CZ>, Cons<Column<CX>, Empty>>;
    type Write = Cons<Buffered<CZ>, Empty>;

    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

struct Close;
impl BuilderInput for Close {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl HasSchedule for Close {
    type Sched = OnMeta<ScheduleEnd>;
}
impl WorkUnit<OnMeta<ScheduleEnd>> for Close {
    type Ctx<'frame> = TestCtx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = read![Column<CX>];
    type Write = write![Column<CY>];

    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

type Bands = Cons<Lead, Cons<Mid, Cons<Close, Empty>>>;

fn banded() -> PlanInputs<MaxUnits, MaxStores> {
    plan_inputs_from_bundle::<Bands, Banded, _, MaxUnits, MaxStores>(RECORDS, MorselBudget::ZERO)
}

fn held(mask: &AccessMask<MaxStores>) -> Vec<usize> {
    (0 .. 3).filter(|p| mask.contains(USize(*p)).0).collect()
}

#[test]
fn a_read_of_a_buffered_column_is_projected_as_frozen() {
    let inputs = banded();
    let reads = inputs.reads.as_ref();
    let frozen = inputs.frozen.as_ref();
    assert_eq!(held(&reads[1]), [0, 1], "the consumer reads both stores");
    assert_eq!(
        held(&frozen[1]),
        [0],
        "only the buffered one at the frozen side"
    );
    assert!(held(&frozen[0]).is_empty(), "the setup reads nothing");
    assert!(
        held(&frozen[2]).is_empty(),
        "the close reads a plain column"
    );
    assert_eq!(
        held(&inputs.writes.as_ref()[1]),
        [0],
        "the write of it is still a write"
    );
}

#[test]
fn each_unit_is_projected_with_the_rank_of_its_band() {
    let inputs = banded();
    let ranks = inputs.ranks.as_ref();
    assert_eq!([ranks[0], ranks[1], ranks[2]], [
        RANK_PASS_START,
        RANK_CONSUMER,
        RANK_SCHEDULE_END
    ]);
}

// A3b: test-local resource values are bare scalars/markers with no Seq/Map
// collection members, so their L1 morsel footprint is zero.
impl hilavitkutin_api::footprint::ResourceFootprint for RA {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
