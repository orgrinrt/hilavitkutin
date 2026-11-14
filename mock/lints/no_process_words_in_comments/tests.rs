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
        "/// Round a bit count up to whole bytes.\n",
        "/// this rounds the value to the nearest step\n",
        "/// round each lane separately\n",
    ] {
        assert!(lines(text).is_empty(), "{text}");
    }
}

#[test]
fn a_round_led_by_a_noun_word_is_refused() {
    for text in [
        "/// written in this round\n",
        "/// the earlier rounds kept it\n",
        "// see the design round\n",
    ] {
        assert_eq!(lines(text), vec![1], "{text}");
    }
}

#[test]
fn a_round_with_any_leader_is_refused() {
    for text in [
        "/// lands in a following round\n",
        "/// the downstream rounds read it\n",
        "// split along its seams in a later\n// round.\n",
        "/// Round (file-size lint): split into modules.\n",
        "/// fixed in round-2b\n",
    ] {
        assert!(!lines(text).is_empty(), "{text}");
    }
}

#[test]
fn a_block_opened_after_code_is_read_on_later_lines() {
    assert_eq!(
        lines("let x = 1; /* opens here\n  written in round 4\n*/\n"),
        vec![2]
    );
}

#[test]
fn code_after_a_block_close_is_not_read() {
    assert!(lines("/* a note\n*/ let round = 4;\n").is_empty());
}

#[test]
fn nested_blocks_close_at_the_outer_close() {
    assert_eq!(
        lines("/* outer /* inner */\n  still in round 4\n*/\nlet round = 1;\n"),
        vec![2]
    );
}

#[test]
fn a_wrapped_multiplication_is_code() {
    assert!(lines("let y = width\n    * x.round()\n    * last round;\n").is_empty());
}

#[test]
fn a_char_literal_quote_does_not_hide_a_trailing_comment() {
    assert_eq!(lines("let q = '\"'; // added in the last round\n"), vec![1]);
    assert_eq!(lines("let q = '\\''; // added in the last round\n"), vec![
        1
    ]);
}

#[test]
fn a_lifetime_is_not_a_char_literal() {
    assert_eq!(lines("fn f<'a>(x: &'a str) {} // the last round\n"), vec![
        1
    ]);
}

#[test]
fn a_block_comment_spans_lines_without_a_leading_star() {
    assert_eq!(lines("/*\n  written in round 4\n*/\nlet x = 1;\n"), vec![2]);
    assert!(lines("/* closed */\n  * the last round\n").is_empty());
}

#[test]
fn code_and_strings_are_not_read() {
    assert!(lines("let round = 3;\nlet s = \"a // round 4\";\nfn topic() {}\n").is_empty());
}

#[test]
fn a_word_inside_a_longer_word_passes() {
    assert!(lines("/// the background thread and the surrounding fiber\n").is_empty());
}
