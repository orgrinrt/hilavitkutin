//! A call whose disk cannot hold its cell's budget and the margin is refused
//! before anything is built; the cell's own target directory counts as free,
//! since the call reuses or removes it.

mod common;

use build_call::space::{MARGIN, budget, free_bytes, free_of_df, refusal};
use build_call::{Call, Control, Profile, Scenario};
use common::Removed;

#[test]
fn a_disk_under_the_budget_and_the_margin_is_refused_and_one_that_holds_them_is_not() {
    let b = 500;
    assert!(refusal(b + MARGIN, 0, b).is_none());
    assert!(refusal(b + MARGIN - 1, 0, b).is_some());
    assert!(
        refusal(MARGIN, b, b).is_none(),
        "what the cell already holds counts as free"
    );
    let why = refusal(0, 0, b).expect("an empty disk is refused");
    assert!(why.contains("refused before anything was built"), "{why}");
}

#[test]
fn incremental_state_needs_the_larger_budget() {
    for n in build_fixture::POINTS {
        assert!(budget(n, true) > budget(n, false), "n={n}");
    }
}

#[test]
fn the_free_space_is_read_from_df_and_a_missing_path_is_an_error() {
    let text = "Filesystem     1024-blocks      Used Available Capacity Mounted on\n\
                /dev/vda          30787492  25000000   5787492      82% /\n";
    assert_eq!(free_of_df(text), Some(5_787_492 * 1024));
    assert_eq!(free_of_df("Filesystem\n"), None);
    let here =
        free_bytes(std::path::Path::new(".")).expect("df reads the working directory's disk");
    assert!(here > 0);
    assert!(free_bytes(std::path::Path::new("/no/such/path/build-call")).is_err());
}

#[test]
#[should_panic(expected = "refused before anything was built")]
fn a_call_on_a_disk_with_no_room_is_refused_in_its_setup() {
    let _gone = Removed::bench("test-space");
    let input = [Scenario::Clean.code(), Profile::Dev.code(), 0, 0, 0, 0, 0, 0];
    let _ = Call::ready_with(
        "test-space",
        "control",
        common::off(),
        &input,
        25,
        Some(Control::NoSpace),
    );
}
