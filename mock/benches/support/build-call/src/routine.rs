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

/// The bound on one call, in microseconds: three times the first timed call
/// measured at that cell, rounded up to whole seconds. `None` leaves the
/// harness's fallback of 300 s per worker.
pub const fn bound_us(scenario: u8, profile: u8, n: usize) -> Option<u64> {
    let _ = (scenario, profile, n);
    None
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
