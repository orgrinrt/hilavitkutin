//! E4 slice-2 de-risk: const-time per-WU lifecycle classification for the
//! self-hosting meta pipeline (plan-stage / consumer / epilogue ranks).
//!
//! Shape-A needs the plan/grouping path to order meta WUs vs consumer WUs by
//! lifecycle (PlanStage before consumers before ScheduleEnd). That needs a
//! per-WU const lifecycle rank computed from the WU's schedule type, at the same
//! const-eval point the grouping computes masks (mirrors BundleProject /
//! BundleMasks over a carrier).
//!
//! THE WALL (reasoned, demonstrated in `wall` mod below): if meta WUs use the
//! SAME `On<V>` marker as consumers (canonical surface `On<meta::PlanStage>`),
//! a per-WU rule "On<meta::X> -> meta rank, On<consumerV> -> consumer rank"
//! cannot be written without specialization: a blanket `impl<V> Lifecycle for
//! On<V>` (consumer default) plus a specific `impl Lifecycle for On<meta::X>`
//! (meta) OVERLAP, and a negative bound `V: !MetaVirtual` is not expressible.
//! Full specialization is forbidden (unstable-features.md).
//!
//! THE ESCAPE (tested here, hypothesis SOME-SHAPE-WORKS): meta WUs declare a
//! DISTINCT schedule marker `OnMeta<V>` (vs consumer `On<V>`). Then three
//! DISJOINT impls (`Always`, `On<V>`, `OnMeta<V> where V: MetaVirtual`) classify
//! every WU's schedule at const time with no overlap and no specialization, and
//! a const fold over a mixed carrier yields the correct per-unit rank array.
//! `OnMeta<V>` is a minor surface adaptation of the canonical `On<meta::V>`
//! (forced by the specialization ban); behaviour (meta WUs gated on lifecycle
//! virtuals, self-hosting) is unchanged.
//!
//! Outcome at the bottom.

#![allow(dead_code)]

use core::marker::PhantomData;

// ---- schedule markers (mirror hilavitkutin-api work_unit) ----
struct Always;
struct On<V>(PhantomData<V>); // consumer: runs when consumer virtual V fires
struct OnMeta<V>(PhantomData<V>); // meta: runs at lifecycle virtual V

// ---- the 4 meta lifecycle markers + the MetaVirtual classifier ----
struct PlanStage;
struct ScheduleReady;
struct PassStart;
struct ScheduleEnd;

/// Lifecycle rank: 0 = plan-stage (before ScheduleReady), 1 = consumer (after
/// ScheduleReady, before ScheduleEnd), 2 = epilogue (after consumers).
type Rank = u8;
const RANK_PLAN: Rank = 0;
const RANK_CONSUMER: Rank = 1;
const RANK_EPILOGUE: Rank = 2;

/// Classifies a meta virtual marker by its lifecycle rank. Impl'd ONLY on the
/// four meta markers (closed set the engine owns). Consumer virtuals never impl
/// it, which is fine: only `OnMeta<V>` reads it.
trait MetaVirtual {
    const RANK: Rank;
}
impl MetaVirtual for PlanStage {
    const RANK: Rank = RANK_PLAN;
}
impl MetaVirtual for ScheduleReady {
    const RANK: Rank = RANK_PLAN; // the ScheduleReady fire boundary sits at plan end
}
impl MetaVirtual for PassStart {
    const RANK: Rank = RANK_CONSUMER;
}
impl MetaVirtual for ScheduleEnd {
    const RANK: Rank = RANK_EPILOGUE;
}

/// Per-schedule lifecycle rank, the const the grouping reads. THREE DISJOINT
/// impls: `Always` and `On<V>` are consumer-rank; `OnMeta<V>` takes V's meta
/// rank. No overlap (distinct head types), no specialization.
trait Lifecycle {
    const RANK: Rank;
}
impl Lifecycle for Always {
    const RANK: Rank = RANK_CONSUMER;
}
impl<V> Lifecycle for On<V> {
    const RANK: Rank = RANK_CONSUMER;
}
impl<V: MetaVirtual> Lifecycle for OnMeta<V> {
    const RANK: Rank = <V as MetaVirtual>::RANK;
}

// ---- minimal WorkUnit + HasSchedule (mirror slice-1 recovery) ----
trait WorkUnit<Schedule = Always> {}
trait HasSchedule {
    type Sched: Lifecycle;
}
impl<W: WorkUnit<Always>> HasSchedule for W {
    type Sched = Always;
}

// ---- carrier (mirror WuCons / WuNil) ----
struct WuCons<W, Tail>(PhantomData<(W, Tail)>);
struct WuNil;

/// Const fold over the carrier writing each unit's lifecycle rank into `out` at
/// its carrier index (mirrors BundleProject's per-unit mask write). Reads
/// `<W as HasSchedule>::Sched` then that schedule's `Lifecycle::RANK`.
trait RankFold {
    fn fold(out: &mut [Rank], idx: usize);
}
impl RankFold for WuNil {
    #[inline]
    fn fold(_out: &mut [Rank], _idx: usize) {}
}
impl<W, Tail> RankFold for WuCons<W, Tail>
where
    W: HasSchedule,
    Tail: RankFold,
{
    #[inline]
    fn fold(out: &mut [Rank], idx: usize) {
        out[idx] = <<W as HasSchedule>::Sched as Lifecycle>::RANK;
        <Tail as RankFold>::fold(out, idx + 1);
    }
}

// ---- test WUs: mixed carrier ----
struct ConsumerAlways;
impl WorkUnit<Always> for ConsumerAlways {}

struct Tick; // a consumer virtual
struct ConsumerOnTick;
impl WorkUnit<On<Tick>> for ConsumerOnTick {}
impl HasSchedule for ConsumerOnTick {
    type Sched = On<Tick>;
}

struct PlanWu;
impl WorkUnit<OnMeta<PlanStage>> for PlanWu {}
impl HasSchedule for PlanWu {
    type Sched = OnMeta<PlanStage>;
}

struct EndWu;
impl WorkUnit<OnMeta<ScheduleEnd>> for EndWu {}
impl HasSchedule for EndWu {
    type Sched = OnMeta<ScheduleEnd>;
}

// ---- the wall, demonstrated: this does NOT compile (kept commented). ----
mod wall {
    // use super::*;
    // // Goal: classify On<meta::X> vs On<consumerV> WITHOUT a distinct OnMeta.
    // trait Lc2 { const RANK: u8; }
    // impl<V> Lc2 for super::On<V> { const RANK: u8 = 1; }      // consumer default
    // impl Lc2 for super::On<super::PlanStage> { const RANK: u8 = 0; } // meta override
    // // ^ error[E0119]: conflicting implementations of trait `Lc2` for `On<PlanStage>`
    // //   (the blanket and the specific overlap; needs specialization, forbidden).
}

fn main() {
    // Carrier order: PlanWu, ConsumerAlways, ConsumerOnTick, EndWu.
    type Carrier =
        WuCons<PlanWu, WuCons<ConsumerAlways, WuCons<ConsumerOnTick, WuCons<EndWu, WuNil>>>>;
    let mut ranks = [255u8; 4];
    <Carrier as RankFold>::fold(&mut ranks, 0);

    assert_eq!(ranks[0], RANK_PLAN, "PlanWu (OnMeta<PlanStage>) -> plan rank");
    assert_eq!(ranks[1], RANK_CONSUMER, "ConsumerAlways -> consumer rank");
    assert_eq!(ranks[2], RANK_CONSUMER, "ConsumerOnTick (On<Tick>) -> consumer rank");
    assert_eq!(ranks[3], RANK_EPILOGUE, "EndWu (OnMeta<ScheduleEnd>) -> epilogue rank");

    // Const-context proof: the rank is a usable associated const (no runtime).
    const PLAN_R: Rank = <OnMeta<PlanStage> as Lifecycle>::RANK;
    const TICK_R: Rank = <On<Tick> as Lifecycle>::RANK;
    const END_R: Rank = <OnMeta<ScheduleEnd> as Lifecycle>::RANK;
    let _: [(); RANK_PLAN as usize] = [(); PLAN_R as usize];
    let _: [(); RANK_CONSUMER as usize] = [(); TICK_R as usize];
    let _: [(); RANK_EPILOGUE as usize] = [(); END_R as usize];

    println!(
        "WORKS: OnMeta<V> gives disjoint const lifecycle classification (plan/consumer/epilogue) over a mixed carrier; no specialization. ranks={:?}",
        ranks
    );
}
