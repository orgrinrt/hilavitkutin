//! Sketch (C6 / #340, Phase C): trunk formation, spectral split + threshold.
//!
//! C6 (roadmap section 9, consolidation Step 7 `:1374-1396`): trunk formation is a
//! BINARY split. For >5 fibers, partition the column-sharing Laplacian with
//! spectral bisection / k_way_partition (Fiedler power iteration count pinned, the
//! spec pins 30, for determinism). For <=5 fibers, a single trunk (spectral is
//! overkill; the waist detection already bounded the phase). Premise: the spectral
//! split wires + is deterministic at a pinned iteration count, and the threshold
//! is a clean policy. Leeway (section 9): SOME-SHAPE; the threshold is bench-
//! tunable later.
//!
//! Hypothesis: a >5-node two-cluster column-sharing Laplacian, run through
//! `k_way_partition` (k=2) with a pinned iteration count, splits the two clusters
//! into distinct trunks AND gives bit-identical results across two runs
//! (deterministic: power iteration from a fixed seed, no RNG, the `iterations`
//! count is an explicit argument). The <=5 case is the single-trunk policy branch.
//! Build + asserts are the test; outcome at the bottom.
//!
//! `TF` is a test-only f32 newtype carrying the arvo numeric traits the spectral
//! surface needs (orphan rule blocks impl-on-f32); copied verbatim from
//! arvo-spectral/tests/common, which is the canonical usage these primitives ship
//! and are tested against.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![allow(dead_code)]

use core::cmp::Ordering;
use core::ops::{Add, Mul, Sub};

use arvo::traits::{FromConstant, Recip, Sqrt, TotalOrd};
use arvo::USize;
use arvo_spectral::{dense_laplacian_lambda_max_bound, k_way_partition, laplacian, Matrix};
use arvo_tensor::Dim;

// The pinned Fiedler power-iteration count (spec Step 7 pins 30 for determinism).
const FIEDLER_ITERS: USize = USize(30);

// The canonical Step-7 threshold: <=5 fibers -> single trunk; >5 -> spectral.
const SPECTRAL_THRESHOLD: usize = 5;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
struct TF(pub f32);
impl Add for TF {
    type Output = TF;
    fn add(self, r: Self) -> Self {
        TF(self.0 + r.0)
    }
}
impl Sub for TF {
    type Output = TF;
    fn sub(self, r: Self) -> Self {
        TF(self.0 - r.0)
    }
}
impl Mul for TF {
    type Output = TF;
    fn mul(self, r: Self) -> Self {
        TF(self.0 * r.0)
    }
}
impl Sqrt for TF {
    type Output = Self;
    fn sqrt(self) -> Self {
        if self.0 < 0.0 || self.0.is_nan() {
            return TF(f32::NAN);
        }
        if self.0 == 0.0 {
            return self;
        }
        let bits = self.0.to_bits();
        let mut g = f32::from_bits((bits >> 1) + (0x3f80_0000u32 >> 1));
        let x = self.0;
        for _ in 0..5 {
            g = 0.5 * (g + x / g);
        }
        TF(g)
    }
}
impl Recip for TF {
    type Output = Self;
    fn recip(self) -> Self {
        TF(1.0_f32 / self.0)
    }
}
impl TotalOrd for TF {
    fn total_cmp(self, other: Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}
impl FromConstant for TF {
    fn from_constant<const C: USize>() -> Self {
        TF(C.0 as f32)
    }
}
impl From<u32> for TF {
    fn from(v: u32) -> Self {
        TF(v as f32)
    }
}

// The Step-7 trunk policy: number of trunks for a given fiber count + the
// (lazy) spectral split. Below threshold -> 1 trunk; above -> spectral k=2 (the
// recursive k_way bisection the spec uses; here demonstrated at k=2, the base).
fn wants_spectral(fiber_count: usize) -> bool {
    fiber_count > SPECTRAL_THRESHOLD
}

fn main() {
    // ---- >5 fibers: a 6-node column-sharing weight graph, two clusters
    // {0,1,2} and {3,4,5}, heavy intra-cluster edges, one weak bridge. ----
    assert!(wants_spectral(6), "6 fibers -> spectral");
    let mut w: Matrix<u32, Dim<6>> = Matrix::from_fn(|_, _| 0u32);
    for (a, b) in [(0, 1), (1, 2), (0, 2), (3, 4), (4, 5), (3, 5)] {
        w.set(USize(a), USize(b), 10);
        w.set(USize(b), USize(a), 10);
    }
    // weak bridge cluster A <-> B
    w.set(USize(2), USize(3), 1);
    w.set(USize(3), USize(2), 1);

    let lap: Matrix<TF, Dim<6>> = laplacian(&w);
    let sigma = dense_laplacian_lambda_max_bound(&lap);

    let (count_a, ids_a) = k_way_partition::<_, Dim<6>, Dim<2>, TF>(&lap, sigma, FIEDLER_ITERS);
    let (count_b, ids_b) = k_way_partition::<_, Dim<6>, Dim<2>, TF>(&lap, sigma, FIEDLER_ITERS);

    assert_eq!(count_a, USize(2), "two trunks on a 2-cluster phase");
    // Determinism: two runs at the pinned iteration count are bit-identical.
    let a = ids_a.as_ref();
    let b = ids_b.as_ref();
    for i in 0..6 {
        assert_eq!(a[i], b[i], "trunk assignment deterministic at node {i}");
    }
    // The two clusters land in distinct trunks; each cluster is internally cohesive.
    assert_eq!(a[0], a[1], "cluster A co-trunk (0,1)");
    assert_eq!(a[1], a[2], "cluster A co-trunk (1,2)");
    assert_eq!(a[3], a[4], "cluster B co-trunk (3,4)");
    assert_eq!(a[4], a[5], "cluster B co-trunk (4,5)");
    assert_ne!(a[0], a[3], "clusters separated into distinct trunks");

    // ---- <=5 fibers: single-trunk policy, no spectral call. ----
    for fc in 0..=5 {
        assert!(!wants_spectral(fc), "{fc} fibers -> single trunk (no spectral)");
    }

    println!(
        "WORKS: C6 trunk formation. >5 fibers (6, two clusters) -> k_way_partition k=2 splits \
         {{0,1,2}} | {{3,4,5}} into 2 distinct trunks, DETERMINISTIC across two runs at the pinned \
         {} Fiedler iterations. <=5 fibers -> single-trunk policy (no spectral). Binary threshold \
         + spectral split both wire and are deterministic.",
        FIEDLER_ITERS.0
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28).
//
// `k_way_partition` (k=2) over the column-sharing Laplacian of a 6-node
// two-cluster graph splits {0,1,2} | {3,4,5} into two distinct trunks, and two
// runs at the pinned 30 Fiedler iterations are BIT-IDENTICAL (deterministic:
// power iteration from a fixed seed, no RNG; `iterations` is an explicit arg).
// The <=5-fiber single-trunk policy is a clean threshold branch.
//
// WHAT THIS SETTLES (C6): the canonical Step-7 binary trunk split wires and is
// deterministic. >5 fibers -> spectral k-way on the Laplacian (the engine already
// calls k_way_partition at steps.rs:402; this confirms determinism at a pinned
// iteration count, the property the schedule needs to be statically analysable);
// <=5 fibers -> single trunk. The shipped arvo-spectral primitives carry it.
//
// WHAT THIS DOES NOT SETTLE: the column-sharing edge-WEIGHT = shared-byte-volume
// computation (a plan-side weight fill, mechanical) and the recursive k>2 split
// for many-cluster phases (k_way_partition handles it; threshold/k tuning is
// bench-decided later, Kind-2). Neither is an open feasibility question.
// ---------------------------------------------------------------------
