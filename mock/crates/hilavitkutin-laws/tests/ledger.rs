//! The ledger check refuses each thing it exists to refuse.

use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::num::{Bool, Count, Index};
use hilavitkutin_laws::ledger::{Finding, check, hash_text, lists};

const FILE: &[u8] = b"pub struct Frozen;\n";

fn read(path: &str) -> Maybe<&'static [u8]> {
    if path == "src/lib.rs" { Maybe::Is(FILE) } else { Maybe::Isnt }
}

fn ledger(hash: &str, breaks: usize, locked: &str) -> String {
    format!(
        "# frozen\n\n[[file]]\npath = \"src/lib.rs\"\nform = \"store\"\nhash = \"{hash}\"\nbreaks = {breaks}\n{locked}\n"
    )
}

fn good_hash() -> String {
    String::from_utf8(hash_text(FILE).to_vec()).unwrap()
}

const ONE_EDIT: &str = "[[break]]\nslice = \"HL0b\"\nfile = \"src/lib.rs\"\nform = \"store\"\nhash = \"x\"\nreason = \"why\"\n";

#[test]
fn holds() {
    assert_eq!(
        check(&ledger(&good_hash(), 0, "locked = false"), "", read),
        Finding::Holds
    );
    assert_eq!(
        check(&ledger(&good_hash(), 1, "locked = false"), ONE_EDIT, read),
        Finding::Holds
    );
    assert_eq!(
        check(
            &ledger(&good_hash(), 1, "locked = true\nlocked_breaks = 1"),
            ONE_EDIT,
            read
        ),
        Finding::Holds
    );
    assert!(lists(&ledger(&good_hash(), 0, "locked = false"), "src/lib.rs").get());
    assert!(!lists(&ledger(&good_hash(), 0, "locked = false"), "src/other.rs").get());
}

#[test]
fn a_stale_hash_is_a_mismatch() {
    let stale = "fnv1a64:0000000000000000";
    assert_eq!(
        check(&ledger(stale, 0, "locked = false"), "", read),
        Finding::Mismatch {
            line: Index::from_host(3),
        }
    );
}

#[test]
fn a_breaks_count_off_the_edits() {
    assert_eq!(
        check(&ledger(&good_hash(), 0, "locked = false"), ONE_EDIT, read),
        Finding::Breaks {
            line:  Index::from_host(3),
            edits: Count::ONE,
        }
    );
    assert_eq!(
        check(&ledger(&good_hash(), 1, "locked = false"), "", read),
        Finding::Breaks {
            line:  Index::from_host(3),
            edits: Count::ZERO,
        }
    );
}

#[test]
fn a_locked_line_edited_after_its_lock() {
    let two_edits = format!("{ONE_EDIT}\n{ONE_EDIT}");
    assert_eq!(
        check(
            &ledger(&good_hash(), 2, "locked = true\nlocked_breaks = 1"),
            &two_edits,
            read
        ),
        Finding::Locked {
            line: Index::from_host(3),
        }
    );
}

#[test]
fn a_missing_file() {
    let text = ledger(&good_hash(), 0, "locked = false").replace("src/lib.rs", "src/gone.rs");
    assert_eq!(check(&text, "", read), Finding::Missing {
        line: Index::from_host(3),
    });
}

#[test]
fn malformed_lines() {
    let no_form = ledger(&good_hash(), 0, "locked = false").replace("form = \"store\"\n", "");
    assert_eq!(check(&no_form, "", read), Finding::Malformed {
        line:      Index::from_host(3),
        in_breaks: Bool::FALSE,
    });
    let bad_count =
        ledger(&good_hash(), 0, "locked = false").replace("breaks = 0", "breaks = none");
    assert_eq!(check(&bad_count, "", read), Finding::Malformed {
        line:      Index::from_host(7),
        in_breaks: Bool::FALSE,
    });
    let locked_without_count = ledger(&good_hash(), 0, "locked = true");
    assert_eq!(check(&locked_without_count, "", read), Finding::Malformed {
        line:      Index::from_host(3),
        in_breaks: Bool::FALSE,
    });
    assert_eq!(
        check(
            &ledger(&good_hash(), 0, "locked = false"),
            "file = \"src/lib.rs\"\n",
            read
        ),
        Finding::Malformed {
            line:      Index::from_host(1),
            in_breaks: Bool::TRUE,
        }
    );
}
