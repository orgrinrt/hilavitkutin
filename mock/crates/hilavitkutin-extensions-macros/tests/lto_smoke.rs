//! Verifies that `#[export_extension]`-emitted trampolines survive
//! aggressive `lto = "fat"` cdylib builds.
//!
//! The fixture under `tests/lto_smoke_fixture/` is a standalone cdylib
//! that uses `#[export_extension(init = Init, shutdown = Shutdown)]`. We
//! build it with `cargo build --release`, read the resulting library with the
//! `object` crate, and assert that:
//!
//! 1. `__hilavitkutin_extension_descriptor` is among the library's exports,
//!    read from the dynamic symbols of an ELF file, the exports of a Mach-O
//!    file and the export table of a PE image.
//! 2. Where the file carries a symbol table, as ELF and Mach-O do, the init
//!    and shutdown trampoline names appear in it. Rust mangling keeps the
//!    original ident as a substring of the mangled form, so
//!    `ext_init_trampoline` and `ext_shutdown_trampoline` are findable even
//!    though the full mangled symbol differs. A PE image has no such table,
//!    so on Windows this step does not apply and step 3 stands in for it.
//! 3. Loading the library, the descriptor's `init_fn` and `shutdown_fn` are
//!    present and resolve to different addresses.
//!
//! The reader refuses a file it cannot parse rather than finding nothing in
//! it, and `a_list_without_the_shutdown_trampoline_is_reported` shows the
//! check reports a missing symbol instead of passing over it.

use std::path::{Path, PathBuf};
use std::process::Command;

use object::{BinaryFormat, Object, ObjectSymbol};

const FIXTURE_LIB: &str = "lto_smoke_fixture";
const DESCRIPTOR: &str = "__hilavitkutin_extension_descriptor";
const TRAMPOLINES: [&str; 2] = ["ext_init_trampoline", "ext_shutdown_trampoline"];

/// What a built library offers to the outside, and what its file records.
struct Symbols {
    /// Names the loader can resolve.
    exports: Vec<Vec<u8>>,
    /// Every name the file's symbol table records, or none where the format
    /// has no such table.
    table:   Option<Vec<Vec<u8>>>,
}

/// A symbol's name, or why the file would not give it. A name that cannot be
/// read refuses the file, since skipping it would read as the symbol absent.
fn name_of<'data>(symbol: &impl ObjectSymbol<'data>) -> Result<Vec<u8>, String> {
    symbol
        .name_bytes()
        .map(<[u8]>::to_vec)
        .map_err(|e| format!("unreadable symbol name: {e}"))
}

/// Read a library's symbols, or say why it cannot.
fn read_symbols(bytes: &[u8]) -> Result<Symbols, String> {
    let file = object::File::parse(bytes).map_err(|e| format!("not an object file: {e}"))?;
    match file.format() {
        BinaryFormat::Elf => {
            // The dynamic symbols are the exports of an ELF library; the
            // static table lists the rest, trampolines included.
            let exports = file
                .dynamic_symbols()
                .filter(|s| s.is_definition())
                .map(|s| name_of(&s))
                .collect::<Result<_, _>>()?;
            let table = file
                .symbols()
                .chain(file.dynamic_symbols())
                .map(|s| name_of(&s))
                .collect::<Result<_, _>>()?;
            Ok(Symbols {
                exports,
                table: Some(table),
            })
        },
        BinaryFormat::MachO => {
            let exports = file
                .exports()
                .map_err(|e| format!("unreadable export trie: {e}"))?
                .iter()
                .map(|e| e.name().to_vec())
                .collect();
            let table = file
                .symbols()
                .map(|s| name_of(&s))
                .collect::<Result<_, _>>()?;
            Ok(Symbols {
                exports,
                table: Some(table),
            })
        },
        BinaryFormat::Pe => {
            let exports = file
                .exports()
                .map_err(|e| format!("unreadable export table: {e}"))?
                .iter()
                .map(|e| e.name().to_vec())
                .collect();
            Ok(Symbols {
                exports,
                table: None,
            })
        },
        other => Err(format!("not a library format this test reads: {other:?}")),
    }
}

/// The wanted names that no name in `names` contains.
///
/// Substring, because mangling wraps an ident (`_ZN..17h<hash>E`) and Mach-O
/// prefixes an underscore, and the ident survives both.
fn missing<'a>(names: &[Vec<u8>], wanted: &[&'a str]) -> Vec<&'a str> {
    wanted
        .iter()
        .copied()
        .filter(|w| !names.iter().any(|n| contains(n, w.as_bytes())))
        .collect()
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

#[test]
fn trampolines_survive_lto() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_manifest = manifest_dir
        .join("tests")
        .join("lto_smoke_fixture")
        .join("Cargo.toml");
    assert!(
        fixture_manifest.exists(),
        "lto_smoke_fixture/Cargo.toml missing at {:?}",
        fixture_manifest,
    );

    let build = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(&fixture_manifest)
        .output()
        .expect("invoke cargo build");
    if !build.status.success() {
        panic!(
            "cargo build --release failed for the LTO fixture\n--- stdout ---\n{}\n--- stderr ---\n{}",
            String::from_utf8_lossy(&build.stdout),
            String::from_utf8_lossy(&build.stderr),
        );
    }

    let target_dir = fixture_target_dir(&fixture_manifest).join("release");
    let candidates = [
        target_dir.join(format!("lib{FIXTURE_LIB}.dylib")),
        target_dir.join(format!("lib{FIXTURE_LIB}.so")),
        target_dir.join(format!("{FIXTURE_LIB}.dll")),
    ];
    let cdylib = candidates
        .iter()
        .find(|p| p.exists())
        .unwrap_or_else(|| panic!("no cdylib output found among {:?}", candidates));

    let bytes = std::fs::read(cdylib).expect("read the LTO fixture");
    let symbols = read_symbols(&bytes)
        .unwrap_or_else(|why| panic!("cannot read the LTO fixture {cdylib:?}: {why}"));

    assert!(
        missing(&symbols.exports, &[DESCRIPTOR]).is_empty(),
        "exported descriptor function missing from the exports of {cdylib:?}",
    );
    if let Some(table) = &symbols.table {
        let gone = missing(table, &TRAMPOLINES);
        assert!(
            gone.is_empty(),
            "trampolines did not survive LTO in {cdylib:?}, missing: {gone:?}",
        );
    }

    assert_distinct_lifecycle_addresses(cdylib);
}

/// Load the library and assert that the descriptor's `init_fn` and
/// `shutdown_fn` slots are present and resolve to distinct addresses.
///
/// Symbol survival is necessary; distinct fn-pointer addresses is what the
/// framework's contract actually requires, and what ICF address aliasing would
/// silently break for non-trivial consumer impls. On a PE image, which has no
/// symbol table to read, this is the check that stands in for the second.
fn assert_distinct_lifecycle_addresses(cdylib: &Path) {
    use core::ffi::c_void;

    use hilavitkutin_extensions::{ExtensionAbiStatus, ExtensionDescriptor};

    type DescriptorFn = unsafe extern "C" fn() -> *const ExtensionDescriptor;
    type LifecycleFn = unsafe extern "C" fn(*mut c_void) -> ExtensionAbiStatus;

    let lib = unsafe { libloading::Library::new(cdylib).expect("load the LTO fixture") };
    let descriptor_fn = unsafe {
        lib.get::<DescriptorFn>(DESCRIPTOR.as_bytes())
            .expect("resolve descriptor symbol")
    };
    let descriptor_ptr = unsafe { descriptor_fn() };
    assert!(
        !descriptor_ptr.is_null(),
        "descriptor function returned null pointer",
    );
    // SAFETY: descriptor function returned a non-null pointer to the
    // extension's static descriptor; valid for the lifetime of the
    // loaded library, which is the rest of this function.
    let descriptor = unsafe { &*descriptor_ptr };
    let init_fn: Option<LifecycleFn> = descriptor.init_fn;
    let shutdown_fn: Option<LifecycleFn> = descriptor.shutdown_fn;
    assert!(init_fn.is_some(), "fixture should declare init_fn");
    assert!(shutdown_fn.is_some(), "fixture should declare shutdown_fn");

    let init_addr = init_fn.unwrap() as usize;
    let shutdown_addr = shutdown_fn.unwrap() as usize;
    assert_ne!(
        init_addr, shutdown_addr,
        "init_fn and shutdown_fn must resolve to distinct addresses; ICF \
         address-aliasing would silently break consumers whose impls compile \
         to non-byte-identical bodies. The per-trampoline byte-string \
         discriminator (b\"init\" vs b\"shutdown\" via core::hint::black_box) \
         is what keeps the addresses distinct.",
    );
}

/// The directory cargo builds the fixture into.
///
/// Asked of cargo rather than assumed beside the manifest, because a
/// `build.target-dir` in any `.cargo/config.toml` above the fixture, or a
/// `CARGO_TARGET_DIR` in the environment, moves it, and a fixed path then
/// finds no library and fails for a reason unrelated to LTO.
fn fixture_target_dir(fixture_manifest: &Path) -> PathBuf {
    let metadata = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--no-deps")
        .arg("--manifest-path")
        .arg(fixture_manifest)
        .output()
        .expect("invoke cargo metadata");
    if !metadata.status.success() {
        panic!(
            "cargo metadata failed for the LTO fixture\n--- stderr ---\n{}",
            String::from_utf8_lossy(&metadata.stderr),
        );
    }
    parse_target_directory(&String::from_utf8_lossy(&metadata.stdout))
        .unwrap_or_else(|| panic!("no target_directory in cargo metadata output"))
}

/// Pull `target_directory` out of `cargo metadata` JSON.
///
/// The value is a path, so the only escapes it can carry are `\\` and
/// `\"`; the parse handles those and nothing else.
fn parse_target_directory(json: &str) -> Option<PathBuf> {
    let key = "\"target_directory\":\"";
    let start = json.find(key)? + key.len();
    let mut out = String::new();
    let mut chars = json[start ..].chars();
    loop {
        match chars.next()? {
            '"' => return Some(PathBuf::from(out)),
            '\\' => out.push(chars.next()?),
            c => out.push(c),
        }
    }
}

#[test]
fn target_directory_is_read_from_metadata() {
    let json = r#"{"packages":[],"target_directory":"/work/target","version":1}"#;
    assert_eq!(
        parse_target_directory(json),
        Some(PathBuf::from("/work/target"))
    );
}

#[test]
fn target_directory_unescapes_a_quote_and_a_backslash() {
    let json = r#"{"target_directory":"/a \"b\"\\c","version":1}"#;
    assert_eq!(
        parse_target_directory(json),
        Some(PathBuf::from("/a \"b\"\\c"))
    );
}

#[test]
fn target_directory_is_absent_without_the_key() {
    assert_eq!(parse_target_directory(r#"{"packages":[]}"#), None);
}

#[test]
fn target_directory_is_absent_when_the_value_never_closes() {
    assert_eq!(
        parse_target_directory(r#"{"target_directory":"/unterminated"#),
        None
    );
}

fn names(list: &[&str]) -> Vec<Vec<u8>> {
    list.iter().map(|s| s.as_bytes().to_vec()).collect()
}

#[test]
fn a_list_without_the_shutdown_trampoline_is_reported() {
    // The control for the survival check: a table that kept the init
    // trampoline and lost the shutdown one is reported as missing exactly it,
    // so a pass above means both names were there.
    let table = names(&[
        "_ZN15lto_smoke_fixture19ext_init_trampoline17h0123456789abcdefE",
        "unrelated_symbol",
    ]);
    assert_eq!(missing(&table, &TRAMPOLINES), vec![
        "ext_shutdown_trampoline"
    ]);
    assert!(missing(&table, &["ext_init_trampoline"]).is_empty());
}

#[test]
fn a_mach_o_underscore_prefix_does_not_hide_the_descriptor() {
    let exports = names(&["___hilavitkutin_extension_descriptor"]);
    assert!(missing(&exports, &[DESCRIPTOR]).is_empty());
}

#[test]
fn a_file_that_is_not_an_object_is_refused() {
    // Nothing found in an unreadable file must not pass for nothing missing.
    assert!(read_symbols(b"this is not a shared library").is_err());
    assert!(read_symbols(&[]).is_err());
}
