//! When two units that share a store take an edge, and what the leading bands
//! write.
//!
//! One definition of the edge rule, read by step 1 of the plan and by the
//! const grouping, so the graph the plan sorts and the phases the dispatch
//! gates on cannot come apart.

use arvo::{Bool, USize};
use arvo_tensor::Capacity;
use hilavitkutin_api::meta::{RANK_CONSUMER, RANK_PLAN_STAGE};

use super::access::AccessMask;
use super::inputs::PlanInputs;

/// Whether an edge from a unit of rank `from` to a unit of rank `to` agrees
/// with the lifecycle bands. The bands run in rank sequence whatever the
/// graph says, so an edge against them is never built.
#[inline]
pub const fn with_the_ranks(from: USize, to: USize) -> Bool {
    Bool(from.0 <= to.0) // lint:allow(no-bare-numeric) reason: lifecycle rank compare; tracked: #121
}

/// Whether the reader waits for the writer: a read-after-write edge from the
/// writer to the reader.
///
/// `waits` is the reader's read mask without the stores it reads at the
/// frozen side, so a buffered column gives no edge. `writes` is the writer's
/// write mask.
#[inline]
pub const fn raw_edge<CS: Capacity>(
    writes: &AccessMask<CS>,
    writer_rank: USize,
    waits: &AccessMask<CS>,
    reader_rank: USize,
) -> Bool {
    Bool(with_the_ranks(writer_rank, reader_rank).0 && writes.overlaps(waits).0)
}

/// Whether the unit at position `p` reads a plain column with a reach behind a
/// writer of it at an earlier position.
///
/// Such a read depends on records other than its own, so the writer's whole
/// range has to land before it starts: the grouping opens a phase at `p`.
/// `reaches` holds, per unit, the plain columns it reads with a reach.
pub const fn reach_behind_a_writer<CS: Capacity>(
    reaches: &[AccessMask<CS>],
    writes: &[AccessMask<CS>],
    ranks: &[USize],
    p: USize,
) -> Bool {
    let mut i = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
    while i < p.0 {
        if raw_edge(&writes[i], ranks[i], &reaches[p.0], ranks[p.0]).0 {
            return Bool::TRUE;
        }
        i += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    Bool::FALSE
}

/// The stores the maintenance units of the leading bands write, apart for the
/// plan band, which runs only in a pass that recomputes the plan.
///
/// A store one of these units writes is taken as changed in every pass the
/// unit runs, so the pass seeds its dirty units from them before its first
/// band.
pub struct BandWrites<CS: Capacity> {
    plan: AccessMask<CS>,
    lead: AccessMask<CS>,
}

impl<CS: Capacity> Copy for BandWrites<CS> {}
impl<CS: Capacity> Clone for BandWrites<CS> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<CS: Capacity> BandWrites<CS> {
    /// No leading band writes anything.
    pub const fn none() -> Self {
        Self {
            plan: AccessMask::empty(),
            lead: AccessMask::empty(),
        }
    }

    /// Gather the write masks of the units ranked below the consumer band.
    ///
    /// The buffered columns are left out: a write of one is seen by nobody in
    /// its own pass, whatever the writer's rank, and reaches its readers by the
    /// boundary that brings it in.
    pub fn of<CU: Capacity>(inputs: &PlanInputs<CU, CS>) -> Self {
        let writes = inputs.writes.as_ref();
        let frozen = inputs.frozen.as_ref();
        let ranks = inputs.ranks.as_ref();
        let n = inputs
            .unit_count
            .0
            .min(writes.len())
            .min(ranks.len())
            .min(frozen.len()); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: loop bound over the live units; tracked: #121
        let mut out = Self::none();
        let mut buffered = AccessMask::empty();
        let mut u = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
        while u < n {
            buffered = buffered.union(&frozen[u]);
            if ranks[u].0 == RANK_PLAN_STAGE.0 {
                out.plan = out.plan.union(&writes[u]);
            } else if ranks[u].0 < RANK_CONSUMER.0 {
                out.lead = out.lead.union(&writes[u]);
            }
            u += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
        }
        out.plan = out.plan.without(&buffered);
        out.lead = out.lead.without(&buffered);
        out
    }

    /// The stores the leading bands of one pass write. `plan_band` says
    /// whether the pass runs its plan band.
    #[inline]
    pub const fn seed(&self, plan_band: Bool) -> AccessMask<CS> {
        if plan_band.0 { self.lead.union(&self.plan) } else { self.lead }
    }
}
