//! The comparison, each way it holds and each way it refuses.

use super::*;

const A: &str = "c0 f1 00000000000000aa\nc0 end written=1 refused=none\n";
const B: &str = "c0 f1 00000000000000ab\nc0 end written=1 refused=none\n";

#[test]
fn equal_streams_that_match_the_golden_hold() {
    let said = verdict(A, A, A, false).expect("holds");
    assert!(said.starts_with("SAME: 2 lines"));
}

#[test]
fn streams_that_part_are_refused_at_the_first_line() {
    let why = verdict(A, B, A, false).expect_err("refused");
    assert!(why.contains("part at line 1"), "{why}");
}

#[test]
fn a_shorter_stream_parts_where_it_ends() {
    let why = verdict(A, "c0 f1 00000000000000aa\n", A, false).expect_err("refused");
    assert!(why.contains("line 2") && why.contains("<none>"), "{why}");
}

#[test]
fn hosts_that_drifted_together_are_refused_by_the_golden() {
    let why = verdict(B, B, A, false).expect_err("refused");
    assert!(why.contains("front/digests.txt"), "{why}");
}

#[test]
fn a_missing_golden_is_refused() {
    assert!(verdict(A, A, "", false).is_err());
}

#[test]
fn an_empty_host_is_refused_even_against_an_empty_golden() {
    let why = verdict("", "", "", false).expect_err("refused");
    assert!(why.contains("printed nothing"), "{why}");
}

#[test]
fn the_control_holds_only_when_the_streams_part() {
    assert!(
        verdict(A, B, A, true)
            .expect("holds")
            .starts_with("CONTROL HOLDS")
    );
    assert!(verdict(A, A, A, true).is_err());
}
