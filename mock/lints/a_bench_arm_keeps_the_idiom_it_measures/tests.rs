//! Tests for `a-bench-arm-keeps-the-idiom-it-measures`: each rewrite clippy
//! makes is refused, and each spelling of the idiom passes.

use super::*;

fn rot() -> &'static Idiom {
    &IDIOMS[0]
}

fn sat() -> &'static Idiom {
    &IDIOMS[1]
}

const ROT_KEPT: &str = "\
#[allow(clippy::manual_rotate)]
#[inline(always)]
fn rotl_manual(v: u64, k: u32) -> u64 {
    (v << k) | (v >> (64 - k))
}
";

const SAT_KEPT: &str = "\
acc = match acc.checked_add(term) {
    Some(v) => v,
    None => u64::MAX,
};
";

#[test]
fn the_shift_or_passes() {
    assert_eq!(finding(ROT_KEPT, rot()), None);
}

#[test]
fn the_checked_match_passes() {
    assert_eq!(finding(SAT_KEPT, sat()), None);
}

#[test]
fn the_rotate_intrinsic_is_refused() {
    let drifted = "fn rotl_manual(v: u64, k: u32) -> u64 {\n    v.rotate_left(k)\n}\n";
    assert!(finding(drifted, rot()).is_some());
}

#[test]
fn the_saturating_add_is_refused() {
    assert!(finding("acc = acc.saturating_add(term);\n", sat()).is_some());
}

#[test]
fn the_unwrap_or_rewrite_is_refused() {
    assert!(finding("acc = acc.checked_add(term).unwrap_or(u64::MAX);\n", sat()).is_some());
}

#[test]
fn the_intrinsic_beside_the_idiom_is_refused() {
    let both = format!("{ROT_KEPT}fn other(v: u64) -> u64 {{ v.rotate_left(3) }}\n");
    assert!(finding(&both, rot()).is_some());
}

#[test]
fn a_comment_naming_the_intrinsic_is_not_a_call() {
    let commented = format!("// clippy would rewrite this to `v.rotate_left(k)`.\n{ROT_KEPT}");
    assert_eq!(finding(&commented, rot()), None);
}

#[test]
fn the_idiom_only_in_a_comment_is_refused() {
    let commented =
        "// (v << k) | (v >> (64 - k))\nfn rotl_manual(v: u64, k: u32) -> u64 { v ^ k as u64 }\n";
    assert!(finding(commented, rot()).is_some());
}

#[test]
fn whitespace_does_not_decide() {
    let packed = "acc=match acc.checked_add(term){Some(v)=>v,None=>u64::MAX};";
    assert_eq!(finding(packed, sat()), None);
    let wide = "(v  <<  k)\n    | (v >> (64 - k))";
    assert_eq!(finding(wide, rot()), None);
}
