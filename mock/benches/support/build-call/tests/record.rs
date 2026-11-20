//! A replay runs no cargo, so its line carries no crate count; the line reads
//! back as written, and a replay line that carries a count is refused.

use build_call::record::{self, Line};
use build_call::{Profile, Scenario, toolchain};

fn replay_line(compiled: Option<usize>) -> Line {
    Line {
        bench: "test-record".into(),
        arm: "default_replay".into(),
        scenario: "link".into(),
        profile: "dev".into(),
        n: 25,
        pid: 1,
        call: 1,
        setup_compiled: 0,
        compiled,
        expected: None,
        absent_before: true,
        present_after: true,
        exe_bytes: 1,
        target_bytes: 1,
        incremental_bytes: 0,
        mark: toolchain::get()
            .own_mark()
            .expect("this host's own linker has a mark"),
        check: "fixture check 0".into(),
        exit: "0".into(),
        waited: false,
        linker: "toolchain".into(),
        quantity: "replay".into(),
    }
}

#[test]
fn a_replay_line_writes_dashes_for_its_counts_and_reads_back() {
    let line = replay_line(None);
    let tsv = line.to_tsv();
    let fields: Vec<&str> = tsv.trim_end().split('\t').collect();
    assert_eq!((fields[8], fields[9]), ("-", "-"), "{tsv}");
    assert_eq!(
        (fields[19], fields[20]),
        ("toolchain", "replay"),
        "the linker and the quantity are the last two columns: {tsv}"
    );
    assert_eq!(
        record::HEADER.trim_end().split('\t').count(),
        fields.len(),
        "the header names every column"
    );
    assert_eq!(Line::from_tsv(tsv.trim_end()), Some(line));
}

/// The quantity is read from its column and from nowhere else: an arm named
/// like a replay that says it relinked is held to a count, and a quantity the
/// bench does not measure is refused.
#[test]
fn the_quantity_is_the_lines_own_and_an_unknown_one_is_refused() {
    let mut named_like_a_relink = replay_line(None);
    named_like_a_relink.arm = "default_relink".into();
    let ok = record::check(
        &[named_like_a_relink],
        "test-record",
        Scenario::Link,
        Profile::Dev,
        25,
    );
    assert!(ok.is_ok(), "the arm's name decides nothing: {ok:?}");
    for unknown in ["", "replays", "Replay", "link"] {
        let mut line = replay_line(None);
        line.quantity = unknown.into();
        let why = record::check(&[line], "test-record", Scenario::Link, Profile::Dev, 25)
            .expect_err("a quantity the bench does not measure must be refused");
        assert!(why.contains("which this bench does not measure"), "{why}");
    }
}

#[test]
fn a_replay_line_that_carries_a_count_is_refused() {
    let ok = record::check(
        &[replay_line(None)],
        "test-record",
        Scenario::Link,
        Profile::Dev,
        25,
    );
    assert!(ok.is_ok(), "{ok:?}");
    let why = record::check(
        &[replay_line(Some(0))],
        "test-record",
        Scenario::Link,
        Profile::Dev,
        25,
    )
    .expect_err("a replay that says it compiled anything ran cargo");
    assert!(why.contains("runs no cargo"), "{why}");
}

#[test]
fn a_cargo_line_with_no_count_is_refused() {
    let mut line = replay_line(None);
    line.quantity = "relink".into();
    let why = record::check(&[line], "test-record", Scenario::Link, Profile::Dev, 25)
        .expect_err("a relink must carry the count of what it compiled");
    assert!(
        why.contains("compiled - crates, the generator predicts 1"),
        "{why}"
    );
}
