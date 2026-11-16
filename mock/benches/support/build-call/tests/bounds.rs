//! The bound on one call: three times the measured first call, scaled up to
//! points no run has measured yet, and absent where nothing was measured.

use build_call::routine::bound_us;
use build_call::{Profile, Scenario};

const S: u64 = 1_000_000;

#[test]
fn a_measured_cell_is_bounded_at_three_times_its_first_call() {
    assert_eq!(
        bound_us(Scenario::Clean.code(), Profile::Dev.code(), 25),
        Some(3 * 17 * S)
    );
    assert_eq!(
        bound_us(Scenario::Leaf.code(), Profile::Dev.code(), 25),
        Some(3 * 6 * S)
    );
}

#[test]
fn a_larger_point_scales_the_nearest_measured_one_below_it_rounding_up() {
    // 17 s at 25 crates is 65.96 s at 97, rounded up to 66.
    assert_eq!(
        bound_us(Scenario::Clean.code(), Profile::Dev.code(), 97),
        Some(3 * 66 * S)
    );
}

#[test]
fn a_point_below_every_measured_one_and_an_unmeasured_cell_have_no_bound() {
    assert_eq!(
        bound_us(Scenario::Clean.code(), Profile::Dev.code(), 10),
        None
    );
    assert_eq!(
        bound_us(Scenario::Link.code(), Profile::Dev.code(), 25),
        None
    );
    assert_eq!(
        bound_us(Scenario::Clean.code(), Profile::Release.code(), 25),
        None
    );
}
