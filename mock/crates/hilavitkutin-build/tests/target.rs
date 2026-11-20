//! The target's facts, read from the four values cargo hands a script.

use hilavitkutin_build::target::Env;
use hilavitkutin_build::{Arch, Isa, ObjectFormat, Os, Target};
use notko::Maybe;

fn target(os: &str, env: &str, arch: &str) -> Target {
    Target::from_cargo(Maybe::Is(os), Maybe::Is(env), Maybe::Is(arch), Maybe::Isnt)
}

#[test]
fn each_value_cargo_passes_reads_as_its_fact() {
    let table: [(&str, &str, &str, Os, Env, Arch, ObjectFormat); 12] = [
        (
            "linux",
            "gnu",
            "x86_64",
            Os::Linux,
            Env::Gnu,
            Arch::X86_64,
            ObjectFormat::Elf,
        ),
        (
            "linux",
            "musl",
            "aarch64",
            Os::Linux,
            Env::Musl,
            Arch::Aarch64,
            ObjectFormat::Elf,
        ),
        (
            "android",
            "",
            "aarch64",
            Os::Android,
            Env::Other,
            Arch::Aarch64,
            ObjectFormat::Elf,
        ),
        (
            "freebsd",
            "",
            "x86_64",
            Os::FreeBsd,
            Env::Other,
            Arch::X86_64,
            ObjectFormat::Elf,
        ),
        (
            "netbsd",
            "",
            "riscv64",
            Os::NetBsd,
            Env::Other,
            Arch::Riscv64,
            ObjectFormat::Elf,
        ),
        (
            "openbsd",
            "",
            "x86_64",
            Os::OpenBsd,
            Env::Other,
            Arch::X86_64,
            ObjectFormat::Elf,
        ),
        (
            "macos",
            "",
            "aarch64",
            Os::MacOs,
            Env::Other,
            Arch::Aarch64,
            ObjectFormat::MachO,
        ),
        (
            "ios",
            "",
            "aarch64",
            Os::Ios,
            Env::Other,
            Arch::Aarch64,
            ObjectFormat::MachO,
        ),
        (
            "windows",
            "msvc",
            "x86_64",
            Os::Windows,
            Env::Msvc,
            Arch::X86_64,
            ObjectFormat::Coff,
        ),
        (
            "windows",
            "gnu",
            "x86_64",
            Os::Windows,
            Env::Gnu,
            Arch::X86_64,
            ObjectFormat::Coff,
        ),
        (
            "unknown",
            "",
            "wasm32",
            Os::Other,
            Env::Other,
            Arch::Wasm32,
            ObjectFormat::Wasm,
        ),
        (
            "wasi",
            "p1",
            "wasm64",
            Os::Wasi,
            Env::Other,
            Arch::Wasm64,
            ObjectFormat::Wasm,
        ),
    ];
    for (os, env, arch, expected_os, expected_env, expected_arch, format) in table {
        let read = target(os, env, arch);
        assert_eq!(
            (read.os, read.env, read.arch, read.format),
            (expected_os, expected_env, expected_arch, format),
            "{os} {env} {arch}"
        );
    }
}

/// A value the crate has no name for, a name in another case, and no
/// value at all read as `Other`, and place the target nowhere.
#[test]
fn an_unknown_or_absent_value_is_other() {
    for os in ["plan9", "Linux", "linux-gnu", ""] {
        let read = target(os, "gnu", "x86_64");
        assert_eq!(
            (read.os, read.format),
            (Os::Other, ObjectFormat::Other),
            "{os:?}"
        );
    }
    assert_eq!(target("linux", "uclibc", "x86_64").env, Env::Other);
    assert_eq!(target("linux", "gnu", "sparc64").arch, Arch::Other);
    // An architecture the crate has no name for is still placed by its system.
    assert_eq!(target("linux", "gnu", "sparc64").format, ObjectFormat::Elf);
    // wasi is placed by its architecture alone.
    assert_eq!(target("wasi", "", "x86_64").format, ObjectFormat::Other);

    let absent = Target::from_cargo(Maybe::Isnt, Maybe::Isnt, Maybe::Isnt, Maybe::Isnt);
    assert_eq!(absent, Target::default());
    assert_eq!(
        (
            absent.os,
            absent.env,
            absent.arch,
            absent.format,
            absent.isa
        ),
        (
            Os::Other,
            Env::Other,
            Arch::Other,
            ObjectFormat::Other,
            Isa::Iss64
        )
    );
}

fn isa(features: &str) -> Isa {
    Target::from_cargo(Maybe::Isnt, Maybe::Isnt, Maybe::Isnt, Maybe::Is(features)).isa
}

#[test]
fn the_richest_listed_feature_is_the_instruction_set() {
    for (features, expected) in [
        ("avx512f,avx2,sse2", Isa::Avx512),
        ("avx512", Isa::Avx512),
        ("sse2,avx2", Isa::Avx2),
        ("neon,sve", Isa::Sve),
        ("neon", Isa::Neon),
        ("sse2, avx2 ", Isa::Avx2),
        ("sse2,sse4.1", Isa::Iss64),
        ("", Isa::Iss64),
    ] {
        assert_eq!(isa(features), expected, "feature list {features:?}");
    }
}

#[test]
fn a_feature_is_matched_whole_never_as_a_prefix() {
    for features in ["avx2x", "avx512vl", "neonx,sve2"] {
        assert_eq!(isa(features), Isa::Iss64, "feature list {features:?}");
    }
}
