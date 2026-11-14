//! Tests for `frozen-no-feature-gate`: each refused shape, and each
//! look-alike that has to pass.

use super::*;

fn lines(text: &str) -> Vec<usize> {
    findings(text).into_iter().map(|f| f.line).collect()
}

#[test]
fn a_feature_attribute_is_refused() {
    assert_eq!(lines("#![no_std]\n#![feature(const_trait_impl)]\n"), vec![
        2
    ]);
}

#[test]
fn spacing_does_not_hide_it() {
    assert_eq!(lines("# ! [ feature ( adt_const_params ) ]\n"), vec![1]);
}

#[test]
fn a_feature_under_cfg_attr_is_refused() {
    assert_eq!(lines("#![cfg_attr(test, feature(test))]\n"), vec![1]);
}

// The compiler takes an attribute broken between any two of its tokens.
#[test]
fn an_attribute_broken_over_lines_is_refused() {
    assert_eq!(lines("#![\nfeature(never_type)]\n"), vec![1]);
    assert_eq!(
        lines("#![no_std]\n#!\n[cfg_attr(\n    test,\n    feature(test)\n)]\n"),
        vec![2]
    );
}

#[test]
fn an_attribute_after_a_broken_one_is_read_on_its_own() {
    assert!(lines("#![cfg_attr(\n    test,\n    allow(dead_code)\n)]\n#[cfg(feature = \"x\")]\nfn f() {}\n").is_empty());
}

#[test]
fn the_shipped_attributes_pass() {
    assert!(lines("#![no_std]\n#![deny(unsafe_op_in_unsafe_fn)]\n").is_empty());
}

#[test]
fn a_cargo_feature_cfg_passes() {
    assert!(
        lines(
            "#![cfg_attr(feature = \"x\", allow(dead_code))]\n#[cfg(feature = \"x\")]\nfn f() {}\n"
        )
        .is_empty()
    );
}

#[test]
fn an_identifier_ending_in_feature_passes() {
    assert!(lines("#![cfg_attr(test, no_feature(x))]\n").is_empty());
}

#[test]
fn comments_and_strings_are_not_read() {
    assert!(
        lines("// #![feature(x)]\n/// #![feature(x)]\nconst S: &str = \"#![feature(x)]\";\n")
            .is_empty()
    );
}

#[test]
fn no_escape_lets_a_line_pass() {
    assert_eq!(
        lines("#![feature(x)] // lint:allow(frozen-no-feature-gate) probing\n").len(),
        1,
        "a lint:allow comment is not read"
    );
}

#[test]
fn the_runner_keeps_a_finding_on_a_line_with_a_marker() {
    let bare = scan::runner::lines_reported(lint(), "#![feature(x)]\n");
    assert_eq!(bare, vec![1], "the runner reports the unmarked line");
    let marked = scan::runner::lines_reported(
        lint(),
        "#![feature(x)] // lint:allow(frozen-no-feature-gate) probing the gate\n",
    );
    assert_eq!(marked, vec![1], "a marker on the line drops nothing");
}

#[test]
fn only_a_frozen_crate_is_in_reach() {
    assert!(scan::applies_to("hilavitkutin-frozen-authors"));
    assert!(!scan::applies_to("hilavitkutin-api"));
    assert!(!scan::applies_to("hilavitkutin"));
}
