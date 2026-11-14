//! Tests for `no-process-words-in-comments`: each refused shape, and each
//! look-alike that has to pass.

use super::*;

fn lines(text: &str) -> Vec<usize> {
    findings(text).into_iter().map(|(line, _)| line).collect()
}

#[test]
fn a_round_by_stamp_in_a_doc_comment_is_refused() {
    assert_eq!(
        lines("/// Lands with round 202605101036.\nfn f() {}\n"),
        vec![1]
    );
}

#[test]
fn each_always_word_is_refused() {
    for text in [
        "//! Topic 5 axis F.\n",
        "// the next changelist adds it\n",
        "/// the runtime megaround\n",
        "/// per the topics of the arc\n",
    ] {
        assert_eq!(lines(text), vec![1], "{text}");
    }
}

#[test]
fn an_audit_finding_is_refused() {
    assert_eq!(lines("/// Per audit-2 M4.\n"), vec![1]);
}

#[test]
fn a_src_or_doc_changelist_by_abbreviation_is_refused() {
    assert_eq!(lines("// see the src CL\n// and the doc CL\n"), vec![1, 2]);
}

#[test]
fn a_trailing_comment_after_code_is_read() {
    assert_eq!(lines("let x = 1; // added in the last round\n"), vec![1]);
}

#[test]
fn a_block_doc_line_is_read() {
    assert_eq!(lines("/**\n * fixed in a later round\n */\n"), vec![2]);
}

#[test]
fn the_word_reported_is_the_first_on_the_line() {
    assert_eq!(findings("/// the topic of round 3\n")[0].1, "topic");
}

#[test]
fn arithmetic_round_passes() {
    for text in [
        "/// round up to the next multiple of 4\n",
        "/// round to nearest even\n",
        "/// a round trip through the codec\n",
        "/// the round-trip holds\n",
        "/// rounding error stays under one ulp\n",
        "/// handed out round robin\n",
    ] {
        assert!(lines(text).is_empty(), "{text}");
    }
}

#[test]
fn code_and_strings_are_not_read() {
    assert!(lines("let round = 3;\nlet s = \"a // round 4\";\nfn topic() {}\n").is_empty());
}

#[test]
fn a_word_inside_a_longer_word_passes() {
    assert!(lines("/// the background thread and the surrounding fiber\n").is_empty());
}
