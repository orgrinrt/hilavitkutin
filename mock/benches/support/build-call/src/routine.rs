//! The routine of each build and link cell: eight input bytes naming the
//! scenario and the profile, one `u64` out, and the bound on one call.
//!
//! The harness passes an arm only the point, so the scenario and the profile
//! reach it in its input, which the routine the hooks library picks for the
//! cell's sweep builds. Arms of one bench answer the same number for the same
//! scenario (the crates compiled, or the executable's size), so the harness
//! holds them to agreeing.

use mockspace_bench_core::{Routine, RoutineBridge, routine_bridge};

use crate::{Profile, Scenario};

/// The cell `SCENARIO` in `PROFILE`, by their codes.
pub struct Cell<const SCENARIO: u8, const PROFILE: u8>;

impl<const SCENARIO: u8, const PROFILE: u8> Routine for Cell<SCENARIO, PROFILE> {
    type Input = [u8; 8];
    type Output = u64;

    fn build_input(_seed: u64) -> [u8; 8] {
        [SCENARIO, PROFILE, 0, 0, 0, 0, 0, 0]
    }

    fn max_call_us(n: usize) -> Option<u64> {
        bound_us(SCENARIO, PROFILE, n)
    }
}

/// The first timed call measured at a cell, in whole seconds rounded up, the
/// slower of the bench's two arms: scenario code, profile code, point, seconds.
/// Each row names the run it was read from.
const FIRST_CALL_S: [(u8, u8, usize, u64); 4] = [
    // build-static, run 20261116T063249Z-f04daa0a-dirty, first warm pass
    (0, 0, 25, 17),
    (1, 0, 25, 6),
    (2, 0, 25, 6),
    (3, 0, 25, 15),
];

/// The bound on one call, in microseconds: three times the first timed call
/// measured at that cell. A point with no row of its own takes the row of the
/// largest measured point below it, scaled by the ratio of the crate counts,
/// until a run measures it. `None` leaves the harness's fallback of 300 s per
/// worker.
pub const fn bound_us(scenario: u8, profile: u8, n: usize) -> Option<u64> {
    let mut best: Option<(usize, u64)> = None;
    let mut i = 0;
    while i < FIRST_CALL_S.len() {
        let (s, p, at, secs) = FIRST_CALL_S[i];
        if s == scenario && p == profile && at <= n {
            best = match best {
                Some((b, _)) if b >= at => best,
                _ => Some((at, secs)),
            };
        }
        i += 1;
    }
    match best {
        Some((at, secs)) => {
            let scaled = (secs * n as u64).div_ceil(at as u64);
            Some(3 * scaled * 1_000_000)
        },
        None => None,
    }
}

/// The bridge of the cell `scenario` in `profile`.
pub fn bridge(scenario: Scenario, profile: Profile) -> RoutineBridge {
    macro_rules! cells {
        ($($s:literal),*) => {
            match (scenario.code(), profile.code()) {
                $(
                    ($s, 0) => routine_bridge!(Cell<$s, 0>),
                    ($s, 1) => routine_bridge!(Cell<$s, 1>),
                )*
                _ => unreachable!("every scenario and profile has a code below"),
            }
        };
    }
    cells!(0, 1, 2, 3, 4)
}
