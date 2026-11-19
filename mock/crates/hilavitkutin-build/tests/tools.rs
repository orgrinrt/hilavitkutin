//! The tools table, where the tools home is, and what a script does about
//! a tool for everything it can find.

use hilavitkutin_build::tools::{CLANG, CRANELIFT, GCC, LLVM_BOLT, MOLD, WILD};
use hilavitkutin_build::{
    Line,
    Os,
    Provision,
    Source,
    TOOLS,
    Tool,
    older,
    provision,
    tools_home,
    write_refusal,
    write_tool_dir,
};
use notko::Maybe;

/// What a script removes beside a tool it built: the directories whose
/// version is below the pin, and nothing else. A newer version is another
/// clone's pin, and a name that is not a version, a lock file's among
/// them, is nobody's to remove.
#[test]
fn only_a_version_below_the_pin_is_older() {
    for below in ["0.9.0", "0.9.12", "0.9", "0", "0.09.99"] {
        assert!(older(below, "0.10.0"), "{below}");
    }
    assert!(older("0.10.0", "0.10.1"));
    assert!(older("0.10", "0.10.1"), "a number left out is a zero");
    assert!(!older("0.10", "0.10.0"), "a number left out is a zero");
    assert!(older("0.10.9", "1.0.0"));
    assert!(older("9", "10"), "numbers, not text");
    for not_below in [
        "0.10.0",
        "0.10.0.0",
        "0.10.1",
        "0.11.0",
        "1.0.0",
        "10",
        "",
        "0.9.0.lock",
        "0.10.0.lock",
        "0.9.0-rc1",
        "0.9.",
        ".9",
        "0..9",
        "linker",
        "0.9.0 ",
        "-1.0.0",
        "+0.9.0",
    ] {
        assert!(!older(not_below, "0.10.0"), "{not_below:?}");
    }
    // A pin that is not dotted numbers has nothing below it.
    assert!(!older("0.9.0", "nightly"));
    assert!(!older("0.9.0", ""));
}

#[test]
fn the_table_names_every_tool_once_with_the_source_the_design_gives_it() {
    let rows: Vec<_> = TOOLS
        .iter()
        .map(|tool| (tool.name, tool.version, tool.source))
        .collect();
    assert_eq!(rows, [
        ("mold", Maybe::Isnt, Source::Package("mold")),
        (
            "wild",
            Maybe::Is("0.10.0"),
            Source::LocalBuild("wild-linker")
        ),
        ("clang", Maybe::Isnt, Source::Package("clang")),
        ("gcc", Maybe::Isnt, Source::Package("gcc")),
        (
            "cranelift",
            Maybe::Isnt,
            Source::Component("rustc-codegen-cranelift-preview")
        ),
        ("polka-passes", Maybe::Isnt, Source::Awaited),
        ("math-peephole", Maybe::Isnt, Source::Awaited),
        ("polly", Maybe::Isnt, Source::Awaited),
        ("llvm-profdata", Maybe::Isnt, Source::Awaited),
        ("llvm-bolt", Maybe::Isnt, Source::Awaited),
        ("mimalloc", Maybe::Isnt, Source::Awaited),
    ]);
    assert_eq!(MOLD.proof.mark, Maybe::Is("mold "));
    assert_eq!(WILD.proof.mark, Maybe::Is("Linker: Wild"));
    assert_eq!(CLANG.proof.mark, Maybe::Isnt);
    assert_eq!(CRANELIFT.proof.line, Line::Sysroot);
    assert_eq!(LLVM_BOLT.proof.line, Line::Nothing);
}

/// The version lines the four programs print on the machine this was
/// written on, and lines that only look like them.
#[test]
fn a_proof_holds_of_the_tools_own_version_line_and_of_no_other() {
    let table: [(&Tool, &str, bool); 16] = [
        (&MOLD, "mold 2.30.0 (compatible with GNU ld)", true),
        (&MOLD, "moldy 1.0", false),
        (&MOLD, "GNU ld (GNU Binutils for Ubuntu) 2.42", false),
        (&MOLD, "", false),
        (
            &WILD,
            "Wild 0.10.0 non-git-build (compatible with GNU linkers)",
            true,
        ),
        (&WILD, "Wild 0.10.0", true),
        (&WILD, "Wild 0.10.01 non-git-build", false),
        (&WILD, "Wild 0.10.0-rc1", false),
        (&WILD, "Wild 0.11.0 (compatible with GNU linkers)", false),
        (&WILD, "wild 0.10.0", false),
        (&CLANG, "Ubuntu clang version 18.1.3 (1ubuntu1)", true),
        (&CLANG, "clang version 19.1.0", true),
        (&CLANG, "gcc (Ubuntu 13.3.0-6ubuntu2~24.04.1) 13.3.0", false),
        (&GCC, "gcc (Ubuntu 13.3.0-6ubuntu2~24.04.1) 13.3.0", true),
        (&GCC, "gcc-13 (Ubuntu) 13.3.0", false),
        (&GCC, "Ubuntu clang version 18.1.3 (1ubuntu1)", false),
    ];
    for (tool, line, expected) in table {
        assert_eq!(tool.proof.holds(line), expected, "{}: {line:?}", tool.name);
    }
    // A tool proved by the sysroot has no line, and one with no source no proof.
    assert!(CRANELIFT.proof.holds(""));
    assert!(!LLVM_BOLT.proof.holds("BOLT revision 1"));
}

const WILD_LINE: &str = "Wild 0.10.0 non-git-build (compatible with GNU linkers)";
const OLD_WILD: &str = "Wild 0.9.0 (compatible with GNU linkers)";
const MOLD_LINE: &str = "mold 2.30.0 (compatible with GNU ld)";

/// Over every kind of source, and for each what is on the path and what
/// is in the home: nothing, the tool, or a program that is not it.
#[test]
fn a_script_does_about_a_tool_what_its_source_and_the_machine_allow() {
    let none = Maybe::Isnt;
    let table: [(&Tool, Maybe<&str>, Maybe<&str>, Provision); 20] = [
        // A local build: the path first, then the home, then the build.
        (&WILD, Maybe::Is(WILD_LINE), none, Provision::Path),
        (
            &WILD,
            Maybe::Is(WILD_LINE),
            Maybe::Is(WILD_LINE),
            Provision::Path,
        ),
        (
            &WILD,
            Maybe::Is(WILD_LINE),
            Maybe::Is(OLD_WILD),
            Provision::Path,
        ),
        (&WILD, none, Maybe::Is(WILD_LINE), Provision::Home),
        (
            &WILD,
            Maybe::Is(OLD_WILD),
            Maybe::Is(WILD_LINE),
            Provision::Home,
        ),
        (&WILD, none, none, Provision::Build),
        (&WILD, Maybe::Is(OLD_WILD), none, Provision::Build),
        (&WILD, none, Maybe::Is(OLD_WILD), Provision::Build),
        (&WILD, Maybe::Is(MOLD_LINE), Maybe::Is(""), Provision::Build),
        // A package is used from the path or refused, whatever a home holds.
        (&MOLD, Maybe::Is(MOLD_LINE), none, Provision::Path),
        (&MOLD, none, none, Provision::Refuse),
        (&MOLD, none, Maybe::Is(MOLD_LINE), Provision::Refuse),
        (&MOLD, Maybe::Is(WILD_LINE), none, Provision::Refuse),
        (
            &GCC,
            Maybe::Is("gcc (Ubuntu 13.3.0-6ubuntu2~24.04.1) 13.3.0"),
            none,
            Provision::Path,
        ),
        (&CLANG, none, none, Provision::Refuse),
        // A component is proved by being found, and refused where it is not.
        (&CRANELIFT, Maybe::Is(""), none, Provision::Path),
        (&CRANELIFT, none, none, Provision::Refuse),
        (&CRANELIFT, none, Maybe::Is(""), Provision::Refuse),
        // A tool with no source is gone on without, whatever is found.
        (&LLVM_BOLT, none, none, Provision::Awaited),
        (
            &LLVM_BOLT,
            Maybe::Is("BOLT revision 1"),
            Maybe::Is("BOLT revision 1"),
            Provision::Awaited,
        ),
    ];
    for (tool, on_path, in_home, expected) in table {
        assert_eq!(
            provision(tool, on_path, in_home),
            expected,
            "{}: {on_path:?} {in_home:?}",
            tool.name
        );
    }
}

fn home(named: Maybe<&str>, xdg: Maybe<&str>, user: Maybe<&str>, host: Os) -> Option<String> {
    Option::from(tools_home(named, xdg, user, host))
        .map(|home: hilavitkutin_build::Home<'_>| home.to_string())
}

/// Each of the three variables set, unset and empty, on a macOS host and
/// on any other.
#[test]
fn the_tools_home_is_the_builders_setting_or_the_per_user_cache() {
    let (set, none, empty) = (Maybe::Is("/set"), Maybe::Isnt, Maybe::Is(""));
    let (xdg, user) = (Maybe::Is("/xdg"), Maybe::Is("/home/u"));
    let table: [(Maybe<&str>, Maybe<&str>, Maybe<&str>, Os, Option<&str>); 12] = [
        (set, xdg, user, Os::Linux, Some("/set")),
        (set, xdg, user, Os::MacOs, Some("/set")),
        (set, none, none, Os::Other, Some("/set")),
        (
            none,
            xdg,
            user,
            Os::Linux,
            Some("/xdg/hilavitkutin-build/tools"),
        ),
        (
            empty,
            xdg,
            user,
            Os::Linux,
            Some("/xdg/hilavitkutin-build/tools"),
        ),
        (
            none,
            none,
            user,
            Os::Linux,
            Some("/home/u/.cache/hilavitkutin-build/tools"),
        ),
        (
            none,
            empty,
            user,
            Os::Other,
            Some("/home/u/.cache/hilavitkutin-build/tools"),
        ),
        (
            none,
            xdg,
            user,
            Os::MacOs,
            Some("/home/u/Library/Caches/hilavitkutin-build/tools"),
        ),
        (
            none,
            none,
            user,
            Os::MacOs,
            Some("/home/u/Library/Caches/hilavitkutin-build/tools"),
        ),
        (none, xdg, none, Os::MacOs, None),
        (none, none, none, Os::Linux, None),
        (empty, empty, empty, Os::Linux, None),
    ];
    for (named, xdg, user, host, expected) in table {
        assert_eq!(
            home(named, xdg, user, host).as_deref(),
            expected,
            "{named:?} {xdg:?} {user:?} {host:?}"
        );
    }
}

#[test]
fn a_tools_directory_is_the_home_the_host_and_the_pinned_name() {
    let mut pinned = String::new();
    write_tool_dir(
        "/home/u/.cache/hilavitkutin-build/tools",
        "x86_64-unknown-linux-gnu",
        &WILD,
        &mut pinned,
    )
    .expect("a String takes every write");
    assert_eq!(
        pinned,
        "/home/u/.cache/hilavitkutin-build/tools/x86_64-unknown-linux-gnu/wild-0.10.0"
    );

    let mut unpinned = String::new();
    write_tool_dir("/t", "aarch64-apple-darwin", &MOLD, &mut unpinned)
        .expect("a String takes every write");
    assert_eq!(unpinned, "/t/aarch64-apple-darwin/mold");
}

fn refusal(tool: &Tool) -> String {
    let mut out = String::new();
    write_refusal(tool, &mut out).expect("a String takes every write");
    out
}

#[test]
fn a_refusal_is_an_error_line_with_the_command_that_brings_the_tool() {
    assert_eq!(
        refusal(&MOLD),
        "cargo::error=hilavitkutin-build: a pragma of this build needs mold, which this machine lacks: install \
         the package `mold` (`apt-get install mold` on Debian and Ubuntu)\n"
    );
    assert_eq!(
        refusal(&CRANELIFT),
        "cargo::error=hilavitkutin-build: a pragma of this build needs cranelift, which this toolchain lacks: \
         run `rustup component add rustc-codegen-cranelift-preview`\n"
    );
    // Nothing is said of a tool the script builds, or of one with no source.
    assert_eq!(refusal(&WILD), "");
    assert_eq!(refusal(&LLVM_BOLT), "");
}

/// A sink that refuses every write.
struct Refusing;

impl core::fmt::Write for Refusing {
    fn write_str(&mut self, _: &str) -> core::fmt::Result {
        Err(core::fmt::Error)
    }
}

#[test]
fn a_refusing_writer_is_reported() {
    assert!(write_refusal(&MOLD, &mut Refusing).is_err());
    assert!(write_tool_dir("/t", "host", &WILD, &mut Refusing).is_err());
}
