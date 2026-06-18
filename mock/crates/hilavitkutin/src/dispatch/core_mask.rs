//! GATE-2 R4a: per-(core, phase) unit masks for the N-core runtime-mask dispatch.
//!
//! op's chosen mechanism (2026-06-07): each worker walks the flat carrier gated
//! by a runtime mask of its owned trunks (the shipped `run_gated<M: BitAccess>`).
//! This module produces that mask. `grouping_arrays` lifts the R2 const grouping
//! (`phase_of` / `trunk_of`) into per-unit phase / trunk arrays; `core_phase_mask`
//! selects, for one core and one phase, the units that core runs.
//!
//! Trunk-to-core ownership is per-phase round-robin: trunks are within-phase, so
//! a core's owned set is decided per phase and redistributed at each waist
//! barrier. Core `c` owns trunk `t` in phase `p` iff the within-phase rank of `t`
//! modulo `ncores` equals `c`. The rank is the count of trunk roots below `t` in
//! phase `p`: trunk ids are component-min positions (`compute_trunks`
//! canonicalises to the smallest member), so trunk `s` exists in phase `p` iff
//! `trunk[s] == s` and `phase[s] == p`. Round-robin is the simplest correct
//! distribution; a load-balanced policy is a later perf fork. Single-core
//! (`ncores == 1`): every rank `% 1 == 0`, so core 0 owns every trunk = the full
//! per-phase walk (the 1-core degenerate, no special path).

use arvo::strategy::Identity;
use arvo::USize;
use arvo_bitmask::BitAccess;
use arvo_tensor::{Capacity, ConstCapacity};

use crate::plan::grouping::{group_n, phase_of, trunk_of, BundleMasks};

/// Fill per-unit phase and trunk arrays from the R2 const grouping, returning
/// the unit count. Loops the shipped `phase_of` / `trunk_of` over carrier
/// positions; `Adj` is the grouping row word threaded to them. The caller sizes
/// `phase_out` / `trunk_out` to at least the unit count.
pub fn grouping_arrays<Wus, Stores, Witnesses, CU, CS, Adj>(
    phase_out: &mut [USize],
    trunk_out: &mut [USize],
) -> USize
where
    CU: Capacity + ConstCapacity,
    CS: Capacity,
    Wus: BundleMasks<Stores, Witnesses, CS>,
    Adj: BitAccess + Identity,
{
    let n = group_n::<Wus, Stores, Witnesses, CU, CS>();
    let mut u = 0; // lint:allow(no-bare-numeric) reason: carrier-position loop index; tracked: #121
    while u < n.0 {
        phase_out[u] = phase_of::<Wus, Stores, Witnesses, CU, CS, Adj>(USize(u));
        trunk_out[u] = trunk_of::<Wus, Stores, Witnesses, CU, CS, Adj>(USize(u));
        u += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    n
}

/// The unit mask core `core` runs in phase `target_phase`: bit `u` set iff
/// `phase[u] == target_phase` and the within-phase rank of `trunk[u]` modulo
/// `ncores` equals `core`. `Adj` is the plan's `D::AdjRow` (the `run_gated` mask
/// word).
pub fn core_phase_mask<Adj: BitAccess + Identity>(
    phase: &[USize],
    trunk: &[USize],
    n: USize,
    core: USize,
    target_phase: USize,
    ncores: USize,
) -> Adj {
    let mut m = <Adj as Identity>::ZERO;
    if ncores.0 == 0 {
        return m;
    }
    let count = n.0; // lint:allow(no-bare-numeric) reason: loop bound; tracked: #121
    let mut u = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
    while u < count {
        if phase[u].0 == target_phase.0 {
            // Within-phase rank of trunk[u]: count of trunk roots below it in
            // this phase (trunk id == component-min position, so trunk `s`
            // exists in this phase iff `trunk[s] == s && phase[s] == p`).
            let t = trunk[u].0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: trunk id as index; tracked: #121
            let mut rank = 0; // lint:allow(no-bare-numeric) reason: within-phase trunk rank; tracked: #121
            let mut s = 0; // lint:allow(no-bare-numeric) reason: trunk-root scan index; tracked: #121
            while s < t {
                if phase[s].0 == target_phase.0 && trunk[s].0 == s {
                    rank += 1; // lint:allow(no-bare-numeric) reason: rank increment; tracked: #121
                }
                s += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
            }
            if rank % ncores.0 == core.0 {
                m = m.with_bit_set(USize(u));
            }
        }
        u += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    m
}

/// Number of trunk roots in `target_phase`. A trunk root is a unit `u` with
/// `trunk[u] == u` (component-min canonicalisation, `compute_trunks`) and
/// `phase[u] == target_phase`. This is the within-phase trunk count the
/// convergence path uses to decide whether a phase is single-trunk (the serial
/// bottleneck the spec splits head+tail).
pub fn phase_trunk_count(phase: &[USize], trunk: &[USize], n: USize, target_phase: USize) -> USize {
    let count = n.0; // lint:allow(no-bare-numeric) reason: loop bound; tracked: #121
    let mut t = 0; // lint:allow(no-bare-numeric) reason: trunk-root tally; tracked: #121
    let mut u = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
    while u < count {
        if phase[u].0 == target_phase.0 && trunk[u].0 == u {
            t += 1; // lint:allow(no-bare-numeric) reason: tally increment; tracked: #121
        }
        u += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    USize(t)
}

/// The mask of every unit in `target_phase` (bit `u` iff `phase[u] ==
/// target_phase`), regardless of trunk or core. The shared mask all cores walk
/// for a single-trunk phase under head+tail convergence: ownership there is by
/// record range, not by trunk, so every core runs the same (single) trunk's
/// units over a disjoint record slice.
pub fn phase_mask<Adj: BitAccess + Identity>(
    phase: &[USize],
    n: USize,
    target_phase: USize,
) -> Adj {
    let mut m = <Adj as Identity>::ZERO;
    let count = n.0; // lint:allow(no-bare-numeric) reason: loop bound; tracked: #121
    let mut u = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
    while u < count {
        if phase[u].0 == target_phase.0 {
            m = m.with_bit_set(USize(u));
        }
        u += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    m
}
