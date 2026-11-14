//! Tests for `frozen-no-outside-source`: each refused shape, and each
//! look-alike that has to pass.

use super::*;

fn lines(text: &str) -> Vec<usize> {
    findings(text).into_iter().map(|f| f.line).collect()
}

#[test]
fn a_path_attribute_is_refused() {
    assert_eq!(
        lines("#![no_std]\n#[path = \"../tests/common/units.rs\"]\nmod units;\n"),
        vec![2]
    );
}

#[test]
fn a_path_under_cfg_attr_is_refused() {
    assert_eq!(
        lines("#[cfg_attr(test, path = \"other.rs\")]\nmod units;\n"),
        vec![1]
    );
}

// The compiler takes an attribute broken between any two of its tokens.
#[test]
fn a_path_attribute_broken_over_lines_is_refused() {
    assert_eq!(
        lines("#[\n    path\n    = \"x.rs\"\n]\nmod units;\n"),
        vec![1]
    );
}

#[test]
fn each_include_macro_is_refused() {
    assert_eq!(lines("include!(\"../tests/common/units.rs\");\n"), vec![1]);
    assert_eq!(
        lines("const TEXT: &str = include_str!(\"../LEDGER.toml\");\n"),
        vec![1]
    );
    assert_eq!(
        lines("fn f() {}\nconst BYTES: &[u8] = include_bytes!(\"x.bin\");\n"),
        vec![2]
    );
}

#[test]
fn spacing_and_a_path_before_the_name_do_not_hide_it() {
    assert_eq!(lines("core::include ! (\"x.rs\");\n"), vec![1]);
    assert_eq!(
        lines("fn f() -> &'static str { return include_str!(\"x\") }\n"),
        vec![1]
    );
}

// A macro can be imported under another name, and the call then carries
// neither the name nor a `!` beside it.
#[test]
fn an_import_under_another_name_is_refused_at_the_import() {
    assert_eq!(
        lines("use core::include as bring;\nbring!(\"../tests/x.rs\");\n"),
        vec![1]
    );
}

#[test]
fn both_in_one_file_come_back_in_line_sequence() {
    assert_eq!(
        lines("include!(\"a.rs\");\n#[path = \"b.rs\"]\nmod b;\n"),
        vec![1, 2]
    );
}

#[test]
fn a_plain_module_and_the_shipped_attributes_pass() {
    assert!(
        lines("#![no_std]\n#![deny(unsafe_op_in_unsafe_fn)]\npub mod units;\nmod record;\n")
            .is_empty()
    );
}

#[test]
fn a_name_that_only_contains_one_passes() {
    assert!(lines("fn includes(x: u32) -> bool { x > 0 }\nconst INCLUDED: () = ();\nfn include_all() {}\nfn path_of() {}\n").is_empty());
}

// `path` as a word of code, and as an attribute's argument that is no key.
#[test]
fn the_word_path_outside_a_path_attribute_passes() {
    assert!(
        lines("#[doc(alias = \"path\")]\nfn path() {}\nconst PATH: () = ();\nlet xpath = 3;\n")
            .is_empty()
    );
    assert!(lines("#[cfg(feature = \"x\")]\nstruct S { path: u32 }\n").is_empty());
}

#[test]
fn comments_and_strings_are_not_read() {
    assert!(lines("// #[path = \"x.rs\"]\n//! include!(\"x.rs\")\nconst S: &str = \"include_str!(x) #[path = y]\";\n").is_empty());
}

#[test]
fn no_escape_lets_a_line_pass() {
    assert_eq!(
        lines("include!(\"x.rs\"); // lint:allow(frozen-no-outside-source) probing\n").len(),
        1,
        "a lint:allow comment is not read"
    );
}

#[test]
fn the_runner_keeps_a_finding_on_a_line_with_a_marker() {
    let bare = scan::runner::lines_reported(lint(), "include!(\"x.rs\");\n");
    assert_eq!(bare, vec![1], "the runner reports the unmarked line");
    let marked = scan::runner::lines_reported(
        lint(),
        "#[path = \"x.rs\"] // lint:allow(frozen-no-outside-source) probing the gate\nmod x;\n",
    );
    assert_eq!(marked, vec![1], "a marker on the line drops nothing");
}
