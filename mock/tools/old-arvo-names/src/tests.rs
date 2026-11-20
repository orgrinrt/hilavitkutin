//! What the count takes for an old path and what it leaves, and the whole path
//! once: a repository with a tracked file, the count of its crate before and
//! after a path is planted in it.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use mockspace::tool::contract_faults;

use super::{Counts, OLD_PATHS, OldArvoNames, count, group_of, render, scan, tally};

const NONE: Counts = [0; OLD_PATHS.len()];

fn slot(path: &str) -> usize {
    OLD_PATHS
        .iter()
        .position(|p| *p == path)
        .unwrap_or_else(|| panic!("{path} is not one of the old paths"))
}

fn with(path: &str, n: usize) -> Counts {
    let mut counts = NONE;
    counts[slot(path)] = n;
    counts
}

#[test]
fn each_old_path_counts_in_its_own_slot() {
    for path in OLD_PATHS {
        assert_eq!(
            count(&format!("use {path}::Thing;")),
            with(path, 1),
            "{path}"
        );
    }
}

#[test]
fn a_path_counts_each_time_it_is_written() {
    let text = "use arvo::{A, B};\nlet x: ::arvo::USize = arvo::USize(0);\n";
    assert_eq!(count(text), with("arvo", 3));
}

#[test]
fn a_name_that_is_no_old_path_counts_for_nothing() {
    // current arvo's crates, a name ending in an old one, a name that only
    // starts with one, and an old name with no path after it
    let text = "use arvo_numeral::USize; use arvo_capacity::Dim; use my_arvo::x; \
                use arvo_bitsy::y; let arvo = 1; arvo_bits; arvo:";
    assert_eq!(count(text), NONE);
}

#[test]
fn a_file_belongs_to_its_crate_or_bench() {
    assert_eq!(
        group_of("crates/hilavitkutin/src/lib.rs").as_deref(),
        Some("hilavitkutin")
    );
    assert_eq!(
        group_of("benches/frame_wide/arms/a/src/lib.rs").as_deref(),
        Some("benches/frame_wide")
    );
    assert_eq!(group_of("benches/lib.rs").as_deref(), Some("benches"));
    assert_eq!(group_of("tools/old-arvo-names/src/lib.rs"), None);
    assert_eq!(group_of("crates/hilavitkutin"), None);
}

#[test]
fn only_rust_files_with_a_group_are_counted() {
    let groups = tally([
        ("crates/a/src/lib.rs", "use arvo::X;"),
        ("crates/a/Cargo.toml", "arvo::X"),
        ("tools/t/src/lib.rs", "use arvo::X;"),
        ("crates/b/src/lib.rs", "pub fn f() {}"),
    ]);
    assert_eq!(groups["a"], with("arvo", 1));
    assert_eq!(groups["b"], NONE);
    assert_eq!(groups.len(), 2, "{groups:?}");
}

/// A git repository in a directory of its own, removed when it is dropped.
struct Repo(PathBuf);

fn git(dir: &Path, args: &[&str]) {
    let done = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("git runs");
    assert!(done.status.success(), "git {args:?}: {done:?}");
}

impl Repo {
    fn new(name: &str) -> Self {
        let dir =
            std::env::temp_dir().join(format!("old-arvo-names-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a temporary directory");
        git(&dir, &["init", "-q"]);
        Self(dir)
    }

    fn write(&self, rel: &str, text: &str) {
        let path = self.0.join(rel);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("a directory");
        std::fs::write(path, text).expect("a file");
    }

    fn track(&self, rel: &str) {
        git(&self.0, &["add", "--", rel]);
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn a_planted_old_path_moves_the_count_of_its_crate() {
    let repo = Repo::new("planted");
    repo.write("mock/crates/linking/src/lib.rs", "pub fn f() {}\n");
    repo.write("mock/crates/engine/src/lib.rs", "use arvo::USize;\n");
    repo.track("mock/crates/linking/src/lib.rs");
    repo.track("mock/crates/engine/src/lib.rs");
    let (before, read) = scan(&repo.0, "mock").expect("a scan");
    assert_eq!(read, 2);
    assert_eq!(before["linking"], NONE);
    assert_eq!(before["engine"], with("arvo", 1));

    repo.write(
        "mock/crates/linking/src/lib.rs",
        "use arvo::USize;\npub fn f() {}\n",
    );
    let (after, _) = scan(&repo.0, "mock").expect("a scan");
    assert_eq!(
        after["linking"],
        with("arvo", 1),
        "the planted path is counted"
    );
    assert_eq!(after["engine"], before["engine"], "and nothing else moves");
}

#[test]
fn an_untracked_file_is_not_read() {
    let repo = Repo::new("untracked");
    repo.write("mock/crates/linking/src/lib.rs", "pub fn f() {}\n");
    repo.track("mock/crates/linking/src/lib.rs");
    repo.write("mock/crates/linking/src/extra.rs", "use arvo::USize;\n");
    let (groups, read) = scan(&repo.0, "mock").expect("a scan");
    assert_eq!(read, 1);
    assert_eq!(groups["linking"], NONE);
}

#[test]
fn the_report_lists_a_crate_with_none_and_the_whole() {
    let mut groups = BTreeMap::new();
    let mut engine = with("arvo", 2);
    engine[slot("arvo_tensor")] = 1;
    groups.insert("engine".to_string(), engine);
    let crates: BTreeSet<String> = ["engine", "linking"].map(String::from).into();
    let said = render(&groups, &crates);
    let lines: Vec<&str> = said.lines().collect();
    assert_eq!(lines.len(), 3, "{said}");
    assert!(
        lines[0].starts_with("engine") && lines[0].ends_with("3  arvo:: 2, arvo_tensor:: 1"),
        "{said}"
    );
    assert!(
        lines[1].starts_with("linking") && lines[1].trim_end().ends_with(" 0"),
        "{said}"
    );
    assert!(
        lines[2].starts_with("all") && lines[2].ends_with("3  arvo:: 2, arvo_tensor:: 1"),
        "{said}"
    );
}

#[test]
fn the_tool_keeps_what_it_declares() {
    assert!(contract_faults(&OldArvoNames, None).is_empty());
}
