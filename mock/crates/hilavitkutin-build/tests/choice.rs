//! The seven build choices: what each pragma writes on each of its three
//! routes, as literal text, and which targets each admits.

mod common;

use common::pragmas;
use hilavitkutin_build::{
    Bfd,
    Bolt,
    ClangCc,
    ClangDriver,
    Cranelift,
    DylibDomain,
    ExpandedLto,
    FastMath,
    GccCc,
    GccDriver,
    Incremental,
    Mold,
    NoIncremental,
    ObjectFormat,
    Pragma,
    Shared,
    Target,
    ThinLto,
    Wild,
    Written,
};
use notko::Maybe;

/// What a pragma writes on its three routes: flags, cfgs and cargo
/// configuration.
fn routes<P: Pragma>(with: &Written<'_>) -> [String; 3] {
    let mut out = [String::new(), String::new(), String::new()];
    P::write_rustflags(with, &mut out[0]).expect("a String takes every write");
    P::write_cfgs(&mut out[1]).expect("a String takes every write");
    P::write_cargo_config(with, &mut out[2]).expect("a String takes every write");
    out
}

const TRIPLE: &str = "x86_64-unknown-linux-gnu";

fn owed(profile: &'static str) -> Written<'static> {
    Written::new(profile, TRIPLE, Maybe::Isnt, Shared::Owed)
}

fn written(profile: &'static str) -> Written<'static> {
    Written::new(profile, TRIPLE, Maybe::Isnt, Shared::Written)
}

/// A linker's argument is a rustc flag, which the wrapper appends: the
/// row of the canon names no route of a script's own for it.
#[test]
fn a_linker_writes_its_argument_as_a_rustc_flag() {
    assert_eq!(routes::<Mold>(&owed("dev")), [
        "-Clink-arg=-fuse-ld=mold",
        "",
        ""
    ]);
    assert_eq!(routes::<Bfd>(&owed("dev")), [
        "-Clink-arg=-fuse-ld=bfd",
        "",
        ""
    ]);
}

/// Wild is named to clang by the path it is handed, the driver and the
/// path as one pair of flags on one route, so neither arrives alone.
/// Without a path it writes nothing.
#[test]
fn wild_is_named_to_clang_by_its_path() {
    let found = Written::new(
        "release",
        TRIPLE,
        Maybe::Is("/home/u/.cache/tools/wild-0.10.0/bin/wild"),
        Shared::Owed,
    );
    assert_eq!(routes::<Wild>(&found), [
        "-Clinker=clang -Clink-arg=--ld-path=/home/u/.cache/tools/wild-0.10.0/bin/wild",
        "",
        ""
    ]);
    assert_eq!(routes::<Wild>(&owed("release")), ["", "", ""]);
}

#[test]
fn the_dylib_edge_writes_its_flag() {
    assert_eq!(routes::<DylibDomain>(&owed("dev")), [
        "-Cprefer-dynamic",
        "",
        ""
    ]);
}

#[test]
fn a_profile_key_is_written_for_the_profile_it_is_handed() {
    assert_eq!(routes::<ThinLto>(&owed("release")), [
        "",
        "",
        "profile.release.lto = \"thin\"\n"
    ]);
    assert_eq!(routes::<ThinLto>(&owed("dev")), [
        "",
        "",
        "profile.dev.lto = \"thin\"\n"
    ]);
    assert_eq!(routes::<ExpandedLto>(&owed("release")), [
        "",
        "",
        "profile.release.lto = \"fat\"\nprofile.release.codegen-units = 1\n"
    ]);
    assert_eq!(routes::<Incremental>(&owed("release")), [
        "",
        "",
        "profile.release.incremental = true\n"
    ]);
    assert_eq!(routes::<NoIncremental>(&owed("dev")), [
        "",
        "",
        "profile.dev.incremental = false\n"
    ]);
    // A key of a profile is written whatever was written before it.
    assert_eq!(
        routes::<ThinLto>(&written("dev")),
        routes::<ThinLto>(&owed("dev"))
    );
    assert_eq!(
        routes::<NoIncremental>(&written("dev")),
        routes::<NoIncremental>(&owed("dev"))
    );
}

/// Cargo's unstable flag belongs to no profile and is written once; the
/// profile's own key is written each time.
#[test]
fn cranelift_writes_the_unstable_flag_once_and_its_profile_key_each_time() {
    assert_eq!(routes::<Cranelift>(&owed("dev")), [
        "",
        "",
        "unstable.codegen-backend = true\nprofile.dev.codegen-backend = \"cranelift\"\n"
    ]);
    assert_eq!(routes::<Cranelift>(&written("release")), [
        "",
        "",
        "profile.release.codegen-backend = \"cranelift\"\n"
    ]);
}

/// The keys of a driver and of a C compiler belong to no profile: they
/// are written where owed and left where a profile before wrote them.
#[test]
fn a_shared_key_is_written_where_owed_and_left_where_written() {
    assert_eq!(routes::<GccDriver>(&owed("dev")), [
        "",
        "",
        "target.\"x86_64-unknown-linux-gnu\".linker = \"gcc\"\n"
    ]);
    assert_eq!(routes::<ClangDriver>(&owed("release")), [
        "",
        "",
        "target.\"x86_64-unknown-linux-gnu\".linker = \"clang\"\n"
    ]);
    let dotted = Written::new("dev", "thumbv8m.main-none-eabi", Maybe::Isnt, Shared::Owed);
    assert_eq!(
        routes::<ClangDriver>(&dotted)[2],
        "target.\"thumbv8m.main-none-eabi\".linker = \"clang\"\n"
    );
    assert_eq!(routes::<GccCc>(&owed("dev")), [
        "",
        "",
        "env.CC = \"gcc\"\nenv.CXX = \"g++\"\nenv.AR = \"ar\"\n"
    ]);
    assert_eq!(routes::<ClangCc>(&owed("dev")), [
        "",
        "",
        "env.CC = \"clang\"\nenv.CXX = \"clang++\"\nenv.AR = \"llvm-ar\"\n"
    ]);
    for left in [
        routes::<GccDriver>(&written("release")),
        routes::<ClangDriver>(&written("release")),
        routes::<GccCc>(&written("release")),
        routes::<ClangCc>(&written("release")),
    ] {
        assert_eq!(left, ["", "", ""]);
    }
}

#[test]
fn fast_math_writes_its_cfg_and_nothing_else() {
    assert_eq!(routes::<FastMath>(&owed("release")), [
        "",
        "arvo_fast_math\n",
        ""
    ]);
}

/// One target of each object format, and one the crate cannot place.
fn targets() -> [(&'static str, Target); 5] {
    let of =
        |os, arch| Target::from_cargo(Maybe::Is(os), Maybe::Isnt, Maybe::Is(arch), Maybe::Isnt);
    [
        ("elf", of("linux", "x86_64")),
        ("mach-o", of("macos", "aarch64")),
        ("coff", of("windows", "x86_64")),
        ("wasm", of("unknown", "wasm32")),
        ("unplaced", Target::default()),
    ]
}

const ELF_ONLY: &str = "links ELF only, and this target's objects are not ELF";
const NOT_WASM: &str = "has no meaning on wasm, which this target is";

/// Over every pragma and every kind of target: the three linkers admit
/// ELF alone, the backend and the dylib edge everything but wasm, and
/// every other pragma every target. `Bolt` is among the others, though it
/// is ELF only: it is in every release set and writes nothing yet.
#[test]
fn every_pragma_admits_the_targets_the_design_says() {
    let targets = targets();
    assert_eq!(targets.map(|(_, target)| target.format), [
        ObjectFormat::Elf,
        ObjectFormat::MachO,
        ObjectFormat::Coff,
        ObjectFormat::Wasm,
        ObjectFormat::Other
    ]);
    for row in pragmas() {
        for (kind, target) in &targets {
            let expected = match (row.name, *kind) {
                ("Mold" | "Wild" | "Bfd", "elf") => Maybe::Isnt,
                ("Mold" | "Wild" | "Bfd", _) => Maybe::Is(ELF_ONLY),
                ("Cranelift" | "DylibDomain", "wasm") => Maybe::Is(NOT_WASM),
                _ => Maybe::Isnt,
            };
            assert_eq!((row.admitted)(target), expected, "{} on {kind}", row.name);
        }
    }
    assert_eq!(Bolt::admitted(&targets[1].1), Maybe::Isnt);
}
