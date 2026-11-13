//! Tests for `frozen-no-recursion-limit`: each refused shape, and each
//! look-alike that has to pass.

use super::*;

fn lines(text: &str) -> Vec<usize> {
    findings(text).into_iter().map(|f| f.line).collect()
}

#[test]
fn a_recursion_limit_is_refused() {
    assert_eq!(lines("#![no_std]\n#![recursion_limit = \"512\"]\n"), vec![
        2
    ]);
}

#[test]
fn a_type_length_limit_is_refused() {
    assert_eq!(lines("#![type_length_limit = \"1048576\"]\n"), vec![1]);
}

#[test]
fn spacing_does_not_hide_it() {
    assert_eq!(lines("# ! [ recursion_limit = \"256\" ]\n"), vec![1]);
}

#[test]
fn one_under_cfg_attr_is_refused() {
    assert_eq!(
        lines("#![cfg_attr(test, recursion_limit = \"256\")]\n"),
        vec![1]
    );
}

#[test]
fn the_shipped_attributes_pass() {
    assert!(lines("#![no_std]\n#![deny(unsafe_op_in_unsafe_fn)]\n").is_empty());
}

#[test]
fn a_name_that_only_contains_it_passes() {
    assert!(lines("#[doc(alias = \"x\")]\nconst MY_RECURSION_LIMITS: () = ();\nfn recursion_limit_of() {}\n").is_empty());
}

#[test]
fn comments_and_strings_are_not_read() {
    assert!(lines("// #![recursion_limit = \"512\"]\n//! recursion_limit\nconst S: &str = \"#![recursion_limit]\";\n").is_empty());
}

#[test]
fn no_escape_lets_a_line_pass() {
    assert_eq!(
        lines("#![recursion_limit = \"512\"] // lint:allow(frozen-no-recursion-limit) probing\n")
            .len(),
        1,
        "a lint:allow comment is not read"
    );
}
