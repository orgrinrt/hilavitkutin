// The two tests every frozen crate's `tests/ledger.rs` includes: its ledger
// holds over its files, and every file under `src/` has a line.
//
// Kept beside the laws' tests rather than in `src/`, because it reads files
// and `src/` is `no_std`. A frozen crate's `tests/ledger.rs` is the one line
// `include!("../../hilavitkutin-laws/tests/support/ledger.rs");`, so the two
// tests are written once. The check itself is `hilavitkutin_laws::ledger`.

const LEDGER: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/LEDGER.toml"));
const BREAKS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/BREAKS.toml"));

fn read(path: &str) -> ::hilavitkutin_contracts::Maybe<&'static [u8]> {
    let full = ::std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    match ::std::fs::read(&full) {
        Ok(bytes) => {
            ::hilavitkutin_contracts::Maybe::Is(::std::boxed::Box::leak(bytes.into_boxed_slice()))
        },
        Err(_) => ::hilavitkutin_contracts::Maybe::Isnt,
    }
}

fn sources(dir: &::std::path::Path, into: &mut ::std::vec::Vec<::std::string::String>) {
    let root = ::std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for item in ::std::fs::read_dir(dir).expect("src reads") {
        let path = item.expect("a directory item reads").path();
        if path.is_dir() {
            sources(&path, into);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let rel = path.strip_prefix(root).expect("under the crate");
            into.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
}

#[test]
fn the_ledger_holds() {
    let finding = ::hilavitkutin_laws::ledger::check(LEDGER, BREAKS, read);
    if let ::hilavitkutin_laws::ledger::Finding::Mismatch {
        line,
    } = finding
    {
        let mut path = "";
        for (n, text) in LEDGER.lines().enumerate() {
            if n + 1 > line.host() {
                if let Some(p) = text.trim().strip_prefix("path = \"") {
                    path = p.trim_end_matches('"');
                    break;
                }
            }
        }
        let bytes = match read(path) {
            ::hilavitkutin_contracts::Maybe::Is(b) => b,
            ::hilavitkutin_contracts::Maybe::Isnt => &[],
        };
        let now = ::hilavitkutin_laws::ledger::hash_text(bytes);
        panic!(
            "{path}: the ledger's hash is stale, the file hashes to {}",
            ::std::str::from_utf8(&now).unwrap_or("?")
        );
    }
    assert_eq!(finding, ::hilavitkutin_laws::ledger::Finding::Holds);
}

#[test]
fn every_source_file_has_a_line() {
    let mut found = ::std::vec::Vec::new();
    sources(
        &::std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut found,
    );
    assert!(!found.is_empty());
    for path in found {
        assert!(
            ::hilavitkutin_laws::ledger::lists(LEDGER, &path).get(),
            "{path} has no line in LEDGER.toml"
        );
    }
}
