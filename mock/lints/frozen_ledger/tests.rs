//! Tests for `frozen-ledger`: a ledger that holds, and each way one does not.

use super::*;

const MANIFEST_OK: &str =
    "[package]\nname = \"x\"\n\n[dependencies]\nhilavitkutin-contracts.workspace = true\n";

fn ledger_of(rows: &[(&str, &str, usize)]) -> String {
    let mut out = String::from("# a comment\n");
    for (path, hash, breaks) in rows {
        out.push_str(&format!(
            "\n[[file]]\npath = \"{path}\"\nform = \"value\"\nhash = \"{hash}\"\nbreaks = {breaks}\nlocked = false\n"
        ));
    }
    out
}

fn files() -> Vec<String> {
    vec!["Cargo.toml".to_string(), "src/lib.rs".to_string()]
}

fn read(path: &str) -> Option<Vec<u8>> {
    match path {
        "Cargo.toml" => Some(MANIFEST_OK.as_bytes().to_vec()),
        "src/lib.rs" => Some(b"#![no_std]\n".to_vec()),
        _ => None,
    }
}

fn good() -> String {
    ledger_of(&[
        ("Cargo.toml", &hash_text(MANIFEST_OK.as_bytes()), 0),
        ("src/lib.rs", &hash_text(b"#![no_std]\n"), 0),
    ])
}

fn messages(found: Vec<Finding>) -> Vec<String> {
    found.into_iter().map(|f| f.message).collect()
}

#[test]
fn the_hash_is_fnv1a_64() {
    assert_eq!(hash_text(b""), "fnv1a64:cbf29ce484222325");
    assert_eq!(hash_text(b"a"), "fnv1a64:af63dc4c8601ec8c");
}

#[test]
fn a_ledger_over_every_file_holds() {
    assert_eq!(findings(&good(), "", MANIFEST_OK, &files(), read), vec![]);
}

#[test]
fn an_edited_file_is_refused() {
    let edited = |path: &str| {
        match path {
            "src/lib.rs" => Some(b"#![no_std]\npub fn f() {}\n".to_vec()),
            p => read(p),
        }
    };
    let found = findings(&good(), "", MANIFEST_OK, &files(), edited);
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0].line, 10);
    assert!(found[0].message.contains("src/lib.rs` hashes to"));
}

#[test]
fn the_manifest_and_a_build_script_need_lines() {
    let only_lib = ledger_of(&[("src/lib.rs", &hash_text(b"#![no_std]\n"), 0)]);
    let mut all = files();
    all.push("build.rs".to_string());
    assert_eq!(
        messages(findings(&only_lib, "", MANIFEST_OK, &all, read)),
        vec!["`Cargo.toml` has no line in the ledger", "`build.rs` has no line in the ledger",]
    );
}

#[test]
fn a_listed_file_that_is_gone_is_refused() {
    let mut rows = good();
    rows.push_str("\n[[file]]\npath = \"src/gone.rs\"\nform = \"value\"\nhash = \"fnv1a64:0000000000000000\"\nbreaks = 0\nlocked = false\n");
    assert_eq!(
        messages(findings(&rows, "", MANIFEST_OK, &files(), read)),
        vec!["`src/gone.rs` is listed and gone"]
    );
}

#[test]
fn a_break_count_must_match_the_recorded_edits() {
    let breaks = "[[break]]\nslice = \"HL0a\"\nfile = \"src/lib.rs\"\nform = \"value\"\nhash = \"fnv1a64:0\"\nreason = \"r\"\n";
    assert_eq!(
        messages(findings(&good(), breaks, MANIFEST_OK, &files(), read)),
        vec!["`src/lib.rs` counts 0 breaks and 1 are recorded"]
    );
    let counted = ledger_of(&[
        ("Cargo.toml", &hash_text(MANIFEST_OK.as_bytes()), 0),
        ("src/lib.rs", &hash_text(b"#![no_std]\n"), 1),
    ]);
    assert_eq!(
        findings(&counted, breaks, MANIFEST_OK, &files(), read),
        vec![]
    );
}

#[test]
fn a_locked_line_keeps_its_count() {
    let locked = good().replacen(
        "path = \"src/lib.rs\"\nform = \"value\"\nhash",
        "path = \"src/lib.rs\"\nform = \"value\"\nlocked_breaks = 1\nhash",
        1,
    );
    let locked = locked.replace(
        "breaks = 0\nlocked = false\n",
        "breaks = 0\nlocked = true\n",
    );
    let found = messages(findings(&locked, "", MANIFEST_OK, &files(), read));
    assert!(
        found.contains(&"`src/lib.rs` is locked at 1 breaks and counts 0".to_string()),
        "{found:?}"
    );
    assert!(
        found.contains(&"`Cargo.toml` is locked and records no `locked_breaks`".to_string()),
        "{found:?}"
    );
}

#[test]
fn a_dependency_beside_the_contracts_is_refused() {
    let manifest = format!(
        "{MANIFEST_OK}notko.workspace = true\n\n[dev-dependencies.extra]\npath = \"x\"\n\n[build-dependencies]\ncc = \"1\"\n"
    );
    let rows = ledger_of(&[
        ("Cargo.toml", &hash_text(manifest.as_bytes()), 0),
        ("src/lib.rs", &hash_text(b"#![no_std]\n"), 0),
    ]);
    let read = |path: &str| {
        match path {
            "Cargo.toml" => Some(manifest.as_bytes().to_vec()),
            p => read(p),
        }
    };
    let found = findings(&rows, "", &manifest, &files(), read);
    assert_eq!(
        found.iter().map(|f| f.line).collect::<Vec<_>>(),
        vec![6, 8, 12],
        "{found:?}"
    );
}

#[test]
fn a_malformed_line_is_refused_and_named() {
    let bad = good().replace("breaks = 0", "breaks = none");
    let found = findings(&bad, "", MANIFEST_OK, &files(), read);
    assert_eq!(found.iter().map(|f| f.line).collect::<Vec<_>>(), vec![
        7, 14
    ]);
    assert!(found.iter().all(|f| f.file == "LEDGER.toml"));
}

#[test]
fn the_documents_and_the_ledgers_need_no_line() {
    assert!(!listed("LEDGER.toml"));
    assert!(!listed("BREAKS.toml"));
    assert!(!listed("DESIGN.md.tmpl"));
    assert!(listed("Cargo.toml"));
    assert!(listed("build.rs"));
    assert!(listed("tests/ledger.rs"));
}

// ---- through the runner, which reads the crate through the lint's view -------

/// The frozen authors crate with a manifest and the given ledger.
fn crate_with(ledger: &str) -> mockspace::testkit::LintFixture {
    scan::runner::authors("#![no_std]\n")
        .with_doc("Cargo.toml", MANIFEST_OK)
        .with_doc("LEDGER.toml", ledger)
}

fn reported(fixture: mockspace::testkit::LintFixture) -> Vec<(usize, String)> {
    scan::runner::reported(lint(), fixture)
        .into_iter()
        .map(|e| (e.line, e.message))
        .collect()
}

#[test]
fn a_crate_whose_ledger_holds_is_clean_through_the_runner() {
    assert_eq!(reported(crate_with(&good())), vec![]);
}

#[test]
fn a_file_the_crate_holds_and_the_ledger_lacks_is_refused_through_the_runner() {
    let fixture = crate_with(&good()).with_module("src/extra.rs", "pub fn f() {}\n");
    assert_eq!(reported(fixture), vec![(
        1,
        "`src/extra.rs` has no line in the ledger".to_string()
    )]);
}

#[test]
fn a_crate_with_no_ledger_is_refused_through_the_runner() {
    let fixture = scan::runner::authors("#![no_std]\n").with_doc("Cargo.toml", MANIFEST_OK);
    assert_eq!(reported(fixture), vec![(
        1,
        "a frozen crate carries no `LEDGER.toml`".to_string()
    )]);
}

#[test]
fn a_marker_in_the_ledger_drops_nothing_through_the_runner() {
    let edited = good().replacen("hash = \"fnv1a64:", "hash = \"fnv1a64:0", 1);
    let bare = reported(crate_with(&edited));
    assert_eq!(bare.len(), 1, "{bare:?}");
    let marked = edited.replacen(
        "[[file]]",
        "[[file]] # lint:allow(frozen-ledger) probing the ledger",
        2,
    );
    assert_eq!(
        reported(crate_with(&marked)),
        bare,
        "a marker drops nothing"
    );
}

#[test]
fn a_crate_that_is_not_frozen_needs_no_ledger() {
    let plain = mockspace::testkit::LintFixture::new("#![no_std]\n")
        .with_crate_name("hilavitkutin-contracts", "contracts");
    assert_eq!(reported(plain), vec![]);
}
