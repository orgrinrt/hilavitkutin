//! Tests for what `frozen-ledger` holds a tree to on the trunk: an edit made
//! since the branch left it, and where the trunk's ledger is read from.

use std::path::{Path, PathBuf};
use std::process::Command;

use super::*;

const EDITED: &[u8] = b"#![no_std]\npub fn f() {}\n";

/// The ledger of `good` with `src/lib.rs` at `EDITED` and counting `breaks`.
fn edited(breaks: usize) -> String {
    ledger_of(&[
        ("Cargo.toml", &hash_text(MANIFEST_OK.as_bytes()), 0),
        ("src/lib.rs", &hash_text(EDITED), breaks),
    ])
}

fn read_edited(path: &str) -> Option<Vec<u8>> {
    match path {
        "src/lib.rs" => Some(EDITED.to_vec()),
        p => read(p),
    }
}

/// `ledger` with every line locked at the count it carries.
fn locked(ledger: &str, at: usize) -> String {
    ledger
        .replace("locked = false", "locked = true")
        .replace("form = ", &format!("locked_breaks = {at}\nform = "))
}

#[test]
fn a_ledger_the_trunk_carries_unchanged_owes_nothing() {
    assert_eq!(since(&good(), &good()), vec![]);
}

// The tree alone cannot tell this from a file that was never edited: the
// hash in the ledger is the file's, and no count says otherwise.
#[test]
fn an_edit_with_its_hash_rewritten_and_no_break_is_refused() {
    assert_eq!(findings(&edited(0), "", &files(), read_edited), vec![]);
    let found = since(&edited(0), &good());
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!((found[0].file, found[0].line), ("LEDGER.toml", 10));
    assert!(
        found[0].message.contains("an edit needs a break"),
        "{found:?}"
    );
}

#[test]
fn an_edit_with_its_break_recorded_is_taken() {
    let breaks = break_of("src/lib.rs", &hash_text(EDITED));
    assert_eq!(findings(&edited(1), &breaks, &files(), read_edited), vec![]);
    assert_eq!(since(&edited(1), &good()), vec![]);
}

// A count that only matches the trunk's is no new break: the one recorded
// there paid for the edit before this one.
#[test]
fn a_second_edit_needs_a_break_of_its_own() {
    let found = since(&edited(1), &broken_once());
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].message.contains("still counts 1 breaks"),
        "{found:?}"
    );
    assert_eq!(since(&edited(2), &broken_once()), vec![]);
}

#[test]
fn a_file_the_trunk_has_locked_does_not_change() {
    let found = since(&locked(&edited(0), 0), &locked(&good(), 0));
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].message.contains("locked on the trunk"),
        "{found:?}"
    );

    // A break recorded for it does not open the lock, and neither does
    // dropping the lock in the same change.
    let found = since(&edited(1), &locked(&good(), 0));
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].message.contains("locked on the trunk"),
        "{found:?}"
    );
}

#[test]
fn a_file_only_one_ledger_lists_is_not_compared() {
    let only_manifest = ledger_of(&[("Cargo.toml", &hash_text(MANIFEST_OK.as_bytes()), 0)]);
    assert_eq!(since(&edited(0), &only_manifest), vec![]);
    assert_eq!(since(&only_manifest, &good()), vec![]);
}

// ---- where the trunk's ledger is read from ------------------------------------

/// A git that answers each command by its first word, and no other.
fn answering(
    answers: &'static [(&'static str, &'static str)],
) -> impl Fn(&[&str]) -> Option<String> {
    move |args| {
        answers
            .iter()
            .find(|(verb, _)| *verb == args[0])
            .map(|(_, out)| (*out).to_string())
    }
}

#[test]
fn a_crate_in_no_repository_is_held_to_the_tree_alone() {
    assert_eq!(trunk_ledger(answering(&[])), Ok(None));
}

#[test]
fn a_clone_with_no_commit_on_the_trunk_is_refused() {
    assert_eq!(
        trunk_ledger(answering(&[("rev-parse", "true\n")])),
        Err(NoTrunk)
    );
}

#[test]
fn a_crate_the_branch_adds_is_held_to_the_tree_alone() {
    let git = answering(&[("rev-parse", "true\n"), ("merge-base", "abc\n")]);
    assert_eq!(trunk_ledger(git), Ok(None));
}

#[test]
fn the_ledger_is_read_at_the_commit_the_branch_left_the_trunk_at() {
    let git = |args: &[&str]| {
        match args {
            ["rev-parse", "--is-inside-work-tree"] => Some("true\n".to_string()),
            ["merge-base", "HEAD", "origin/dev"] => Some("abc\n".to_string()),
            ["show", "abc:./LEDGER.toml"] => Some("then".to_string()),
            _ => None,
        }
    };
    assert_eq!(trunk_ledger(git), Ok(Some("then".to_string())));
}

/// A repository on disk, removed when dropped.
struct Repo(PathBuf);

impl Repo {
    fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("frozen-ledger-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("authors")).unwrap();
        let repo = Self(dir);
        repo.git(&["init", "-q"]);
        repo
    }

    fn krate(&self) -> PathBuf {
        self.0.join("authors")
    }

    fn git(&self, args: &[&str]) -> Option<String> {
        run_git(&self.0, args)
    }

    /// Write the crate's ledger and commit it.
    fn commit(&self, ledger: &str) {
        std::fs::write(self.krate().join("LEDGER.toml"), ledger).unwrap();
        self.git(&["add", "-A"]).unwrap();
        let identity = ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false"];
        let commit = ["commit", "-q", "--no-verify", "--allow-empty", "-m", "c"];
        self.git(&[&identity[..], &commit[..]].concat()).unwrap();
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn run_git(dir: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

// The commands as git itself takes them: the ledger comes from the commit the
// branch left the trunk at, by a path beside the crate, and neither from a
// later commit of the trunk nor from the branch's own.
#[test]
fn a_real_clone_gives_the_ledger_the_branch_left_the_trunk_with() {
    let repo = Repo::new("left");
    repo.commit("base");
    repo.git(&["branch", "work"]).unwrap();
    repo.commit("later on the trunk");
    repo.git(&["update-ref", "refs/remotes/origin/dev", "HEAD"])
        .unwrap();
    repo.git(&["checkout", "-q", "work"]).unwrap();
    repo.commit("on the branch");

    let krate = repo.krate();
    assert_eq!(
        trunk_ledger(|args: &[&str]| run_git(&krate, args)),
        Ok(Some("base".to_string()))
    );
}

#[test]
fn a_real_clone_with_no_trunk_is_refused() {
    let repo = Repo::new("none");
    repo.commit("base");
    let krate = repo.krate();
    assert_eq!(
        trunk_ledger(|args: &[&str]| run_git(&krate, args)),
        Err(NoTrunk)
    );
}
