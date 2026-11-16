//! An arm is held to the mark its named linker leaves: the default arm to the
//! version line of the toolchain's own `rust-lld`, read at run time. A missing
//! mark, another linker's mark, another build of the same lld, a second
//! linker's string joined after the right one, and an arm naming a linker the
//! bench has no mark for are each refused. Whether that mark is the one the
//! default linker really writes is shown by a real link in `controls.rs`.

use build_call::record::{self, Line};
use build_call::{Profile, Scenario, toolchain};

const N: usize = 25;

/// An honest line of a relink or a replay by `arm`, carrying `mark`.
fn link_line(arm: &str, mark: &str) -> Line {
    let replay = record::is_replay(arm);
    Line {
        bench:             "test-marks".into(),
        arm:               arm.into(),
        scenario:          "link".into(),
        profile:           "dev".into(),
        n:                 N,
        pid:               1,
        call:              1,
        setup_compiled:    0,
        compiled:          (!replay).then_some(1),
        expected:          (!replay).then_some(1),
        absent_before:     true,
        present_after:     true,
        exe_bytes:         1,
        target_bytes:      1,
        incremental_bytes: 0,
        mark:              mark.into(),
        check:             "fixture check 0".into(),
        exit:              "0".into(),
        waited:            false,
    }
}

fn checked(line: Line) -> Result<String, String> {
    record::check(&[line], "test-marks", Scenario::Link, Profile::Dev, N)
}

fn default_mark() -> String {
    toolchain::get()
        .default_mark()
        .expect("this host's default linker has a mark")
}

#[test]
fn the_default_arms_pass_on_the_toolchains_own_lld_and_on_nothing_more_or_less() {
    let want = default_mark();
    for arm in ["default_relink", "default_replay"] {
        assert!(checked(link_line(arm, &want)).is_ok(), "{arm}");
        let joined = format!("{want} | mold 2.30.0 (compatible with GNU ld)");
        assert!(
            checked(link_line(arm, &joined)).is_err(),
            "{arm}: a second linker's string joined after the right one: {joined}"
        );
        let without_build = want
            .split_once(" (")
            .map_or_else(|| format!("{want}x"), |(v, _)| v.to_string());
        assert!(
            checked(link_line(arm, &without_build)).is_err(),
            "{arm}: the same version with no build of the toolchain's own: {without_build}"
        );
    }
}

#[test]
fn a_missing_mark_is_refused() {
    let why = checked(link_line(
        "default_relink",
        "none among [\"rustc version 1.98.0-nightly\", \"GCC: (Ubuntu 13.3.0) 13.3.0\"]",
    ))
    .expect_err("an executable with no linker mark must be refused");
    assert!(why.contains("must carry `Linker: LLD"), "{why}");
}

#[test]
fn another_linkers_mark_on_the_default_arm_is_refused() {
    for other in [
        "mold 2.30.0 (compatible with GNU ld)",
        "Wild 0.10.0",
        "Linker: LLD 18.1.3",
        "Linker: LLD 22.1.6 (/usr/src/llvm 0000000)",
    ] {
        let why = checked(link_line("default_relink", other))
            .expect_err("the default arm linked by another linker must be refused");
        assert!(why.contains(other), "{why}");
    }
}

#[test]
fn the_same_linker_at_a_version_that_only_starts_alike_is_refused() {
    let longer = format!("{}0", default_mark());
    assert!(
        checked(link_line("default_relink", &longer)).is_err(),
        "{longer}"
    );
}

#[test]
fn an_arm_named_for_a_linker_with_no_known_mark_is_refused_whatever_it_carries() {
    let why = checked(link_line("mold_relink", &default_mark()))
        .expect_err("an arm named for mold whose output carries lld's mark must be refused");
    assert!(why.contains("names the linker `mold`"), "{why}");
}

#[test]
fn the_default_mark_is_the_own_lld_version_line_and_nothing_else_has_one() {
    let own = r#"{ "link-self-contained": { "components": [ "linker" ] }, "linker-flavor": "gnu-lld-cc" }"#;
    let said =
        "LLD 22.1.6 (/checkout/src/llvm-project/llvm 08c84e69) (compatible with GNU linkers)";
    assert_eq!(
        toolchain::default_mark(own, Some(said), "h"),
        Ok("Linker: LLD 22.1.6 (/checkout/src/llvm-project/llvm 08c84e69)".to_string())
    );
    assert!(
        toolchain::default_mark(own, None, "h").is_err(),
        "a toolchain whose rust-lld said nothing"
    );
    assert!(
        toolchain::default_mark(own, Some("LLD 22.1.6"), "h").is_err(),
        "a version line in a form this bench does not know"
    );
    assert!(
        toolchain::default_mark(own, Some("mold 2.30.0 (compatible with GNU linkers)"), "h")
            .is_err(),
        "a rust-lld that is not lld"
    );
    let cc = r#"{ "linker-flavor": "gnu-cc" }"#;
    assert!(toolchain::default_mark(cc, Some(said), "h").is_err());
    let system_lld = r#"{ "linker-flavor": "gnu-lld-cc" }"#;
    assert!(
        toolchain::default_mark(system_lld, Some(said), "h").is_err(),
        "an lld the toolchain does not carry is not the one rust-lld describes"
    );
}
