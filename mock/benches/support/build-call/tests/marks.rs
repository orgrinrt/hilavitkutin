//! A line is held to the mark its linker leaves, as hilavitkutin-build says
//! it: the toolchain's own to the version line of its `rust-lld`, read at run
//! time, a named linker to a text that starts as its tool's row says, and a
//! linker with no mark to none. A missing mark, another linker's mark, another
//! build of the same lld, a second linker's string joined after the right
//! one, and a linker name the crate does not know are each refused. Whether
//! the toolchain's mark is the one its linker really writes is shown by a real
//! link in `controls.rs`, and how that mark is worked out is held by
//! hilavitkutin-build's own `tests/linker.rs`.

use build_call::linker::NO_MARK;
use build_call::record::{self, Line};
use build_call::{Kind, Profile, Scenario, toolchain};

const N: usize = 25;

/// An honest line of a relink or a replay linked by `linker`, carrying `mark`.
fn link_line(linker: &str, kind: Kind, mark: &str) -> Line {
    let replay = kind == Kind::Replay;
    Line {
        bench:             "test-marks".into(),
        arm:               "an-arm".into(),
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
        linker:            linker.into(),
        quantity:          kind.label().into(),
    }
}

fn checked(line: Line) -> Result<String, String> {
    record::check(&[line], "test-marks", Scenario::Link, Profile::Dev, N)
}

fn own_mark() -> String {
    toolchain::get()
        .own_mark()
        .expect("this host's own linker has a mark")
}

/// What a line carries where the linker left nothing.
fn no_mark() -> String {
    format!("{NO_MARK}[\"rustc version 1.98.0-nightly\", \"GCC: (Ubuntu 13.3.0) 13.3.0\"]")
}

const MOLD_MARK: &str = "mold 2.30.0 (compatible with GNU ld)";

#[test]
fn the_toolchains_own_passes_on_its_own_lld_and_on_nothing_more_or_less() {
    let want = own_mark();
    for kind in [Kind::Relink, Kind::Replay] {
        assert!(
            checked(link_line("toolchain", kind, &want)).is_ok(),
            "{kind:?}"
        );
        let joined = format!("{want} | {MOLD_MARK}");
        assert!(
            checked(link_line("toolchain", kind, &joined)).is_err(),
            "{kind:?}: a second linker's string joined after the right one: {joined}"
        );
        let without_build = want
            .split_once(" (")
            .map_or_else(|| format!("{want}x"), |(v, _)| v.to_string());
        assert!(
            checked(link_line("toolchain", kind, &without_build)).is_err(),
            "{kind:?}: the same version with no build of the toolchain's own: {without_build}"
        );
    }
}

#[test]
fn a_missing_mark_is_refused() {
    let why = checked(link_line("toolchain", Kind::Relink, &no_mark()))
        .expect_err("an executable with no linker mark must be refused");
    assert!(why.contains("must carry `Linker: LLD"), "{why}");
}

#[test]
fn another_linkers_mark_on_the_toolchains_own_is_refused() {
    for other in [
        MOLD_MARK,
        "Wild 0.10.0",
        "Linker: LLD 18.1.3",
        "Linker: LLD 22.1.6 (/usr/src/llvm 0000000)",
    ] {
        let why = checked(link_line("toolchain", Kind::Relink, other))
            .expect_err("the toolchain's own linked by another linker must be refused");
        assert!(why.contains(other), "{why}");
    }
}

#[test]
fn the_same_linker_at_a_version_that_only_starts_alike_is_refused() {
    let longer = format!("{}0", own_mark());
    assert!(
        checked(link_line("toolchain", Kind::Relink, &longer)).is_err(),
        "{longer}"
    );
}

#[test]
fn a_named_linker_is_held_to_the_mark_its_tool_leaves() {
    assert!(checked(link_line("Mold", Kind::Relink, MOLD_MARK)).is_ok());
    assert!(checked(link_line("Mold", Kind::Replay, MOLD_MARK)).is_ok());
    let why = checked(link_line("Mold", Kind::Relink, &own_mark()))
        .expect_err("a line of mold whose output carries lld's mark must be refused");
    assert!(why.contains("Mold leaves one that starts `mold `"), "{why}");
    let why = checked(link_line("Mold", Kind::Relink, &no_mark()))
        .expect_err("a line of mold whose output carries no mark must be refused");
    assert!(why.contains("Mold leaves one that starts"), "{why}");
    assert!(
        checked(link_line(
            "Wild",
            Kind::Relink,
            "Linker: Wild version 0.10.0"
        ))
        .is_ok()
    );
    assert!(checked(link_line("Wild", Kind::Relink, MOLD_MARK)).is_err());
}

#[test]
fn a_linker_that_leaves_no_mark_is_held_to_none() {
    assert!(checked(link_line("Bfd", Kind::Relink, &no_mark())).is_ok());
    for other in [own_mark(), MOLD_MARK.to_string()] {
        let why = checked(link_line("Bfd", Kind::Relink, &other))
            .expect_err("a line of GNU ld whose output carries a linker's mark must be refused");
        assert!(why.contains("Bfd leaves none"), "{why}");
    }
    // An executable whose comment section could not be read carries the
    // error as its mark, which is no proof of GNU ld.
    assert!(
        checked(link_line(
            "Bfd",
            Kind::Relink,
            "/x/fx_facade: not a 64-bit little-endian ELF file"
        ))
        .is_err()
    );
}

#[test]
fn a_linker_name_the_crate_does_not_know_is_refused_whatever_the_output_carries() {
    for unknown in ["default", "mold", "gold", ""] {
        let why = checked(link_line(unknown, Kind::Relink, &own_mark()))
            .expect_err("a line of an unknown linker must be refused");
        assert!(
            why.contains(&format!("names the linker `{unknown}`")),
            "{why}"
        );
    }
}
