//! The generated cargo file: its text for a prototype, whose it is by its
//! first line, and whether a `config.toml` takes it.

use hilavitkutin_build::{
    Claim,
    ClangCc,
    ClangDriver,
    Cranelift,
    Default,
    ExpandedLto,
    FastMath,
    GENERATED_NAME,
    GccDriver,
    INCLUDE_LINE,
    Incremental,
    Mold,
    NoIncremental,
    Pgo,
    Prototype,
    Target,
    ThinLto,
    Workspace,
    claim,
    claimed,
    includes_generated,
    write_cargo_config,
    write_generated,
    write_uncarried,
};
use notko::Maybe;

fn generated<P: Prototype>(krate: &str) -> String {
    let mut out = String::new();
    write_generated::<P, _>(krate, "x86_64-unknown-linux-gnu", &mut out)
        .expect("a String takes every write");
    // One key twice is a file cargo cannot read, whatever the prototype.
    let mut keys: Vec<_> = out
        .lines()
        .skip(1)
        .map(|line| line.split(" = ").next().unwrap_or(line))
        .collect();
    let written = keys.len();
    keys.sort_unstable();
    keys.dedup();
    assert_eq!(keys.len(), written, "a key is written twice:\n{out}");
    out
}

/// Every pragma of workspace reach but the second driver and C compiler,
/// the shared ones in every profile, as a claiming script has to have
/// them.
struct Everything;

impl Prototype for Everything {
    type Ci = (ClangDriver, ClangCc);
    type Dev = (Cranelift, Incremental, ClangDriver, ClangCc, FastMath);
    type DevOpt = (ClangDriver, ClangCc);
    type Profiling = (ClangDriver, ClangCc);
    type Release = (ExpandedLto, NoIncremental, ClangDriver, ClangCc, Pgo);
}

/// One backend in both profiles: its unstable flag belongs to neither.
struct BothCranelift;

impl Prototype for BothCranelift {
    type Ci = ();
    type Dev = (Cranelift,);
    type DevOpt = ();
    type Profiling = ();
    type Release = (Cranelift, NoIncremental);
}

/// The other driver, in every profile, beside a key of each profile.
struct GccDriven;

impl Prototype for GccDriven {
    type Ci = (GccDriver,);
    type Dev = (ThinLto, GccDriver);
    type DevOpt = (GccDriver,);
    type Profiling = (GccDriver,);
    type Release = (GccDriver, ThinLto);
}

// The text below is asserted of files a claiming script can write: each
// of the three is a prototype `claimed` accepts.
const _: () = claimed::<Everything>();
const _: () = claimed::<BothCranelift>();
const _: () = claimed::<GccDriven>();

#[test]
fn the_file_is_the_claim_then_the_keys_of_dev_and_of_release() {
    assert_eq!(
        generated::<Everything>("kaski-app"),
        "# written by hilavitkutin-build for kaski-app: edits are lost\n\
         unstable.codegen-backend = true\n\
         profile.dev.codegen-backend = \"cranelift\"\n\
         profile.dev.incremental = true\n\
         target.\"x86_64-unknown-linux-gnu\".linker = \"clang\"\n\
         env.CC = \"clang\"\n\
         env.CXX = \"clang++\"\n\
         env.AR = \"llvm-ar\"\n\
         profile.release.lto = \"fat\"\n\
         profile.release.codegen-units = 1\n\
         profile.release.incremental = false\n"
    );
}

#[test]
fn a_key_of_no_profile_is_in_the_file_once() {
    assert_eq!(
        generated::<BothCranelift>("a"),
        "# written by hilavitkutin-build for a: edits are lost\n\
         unstable.codegen-backend = true\n\
         profile.dev.codegen-backend = \"cranelift\"\n\
         profile.release.codegen-backend = \"cranelift\"\n\
         profile.release.incremental = false\n"
    );
    // The driver's key is written by the dev set and left by the release
    // set, wherever in its set the pragma stands.
    assert_eq!(
        generated::<GccDriven>("a"),
        "# written by hilavitkutin-build for a: edits are lost\n\
         profile.dev.lto = \"thin\"\n\
         target.\"x86_64-unknown-linux-gnu\".linker = \"gcc\"\n\
         profile.release.lto = \"thin\"\n"
    );
}

fn target(os: &'static str, arch: &'static str) -> Target {
    Target::from_cargo(Maybe::Is(os), Maybe::Isnt, Maybe::Is(arch), Maybe::Isnt)
}

fn uncarried<P: Prototype>(target: Target, lacked: &[&str]) -> String {
    let mut out = String::new();
    write_uncarried::<P, _>(&target, |tool| lacked.contains(&tool.name), &mut out)
        .expect("a String takes every write");
    out
}

/// A pragma whose key would be in the file and which the build cannot
/// carry is named once for each set that holds it, with the set's
/// profile. The same prototype on a build that carries it writes nothing.
#[test]
fn a_pragma_the_build_cannot_carry_is_named_with_its_profile() {
    let elf = || target("linux", "x86_64");
    assert_eq!(uncarried::<Everything>(elf(), &[]), "");
    assert_eq!(
        uncarried::<Workspace>(elf(), &["cranelift", "clang", "gcc"]),
        ""
    );

    // In one set alone, and in both.
    assert_eq!(
        uncarried::<Everything>(elf(), &["cranelift"]),
        "cargo::error=hilavitkutin-build: Cranelift, in the dev set of the prototype that claims the workspace, \
         waits on cranelift, which this build has not got: the generated cargo file is left as it was\n"
    );
    assert_eq!(
        uncarried::<BothCranelift>(elf(), &["cranelift"]),
        "cargo::error=hilavitkutin-build: Cranelift, in the dev set of the prototype that claims the workspace, \
         waits on cranelift, which this build has not got: the generated cargo file is left as it was\n\
         cargo::error=hilavitkutin-build: Cranelift, in the release set of the prototype that claims the workspace, \
         waits on cranelift, which this build has not got: the generated cargo file is left as it was\n"
    );
    // One tool two pragmas of a set wait on is said of each.
    let no_clang = uncarried::<Everything>(elf(), &["clang"]);
    assert_eq!(no_clang.lines().count(), 4, "{no_clang}");
    assert!(
        no_clang.starts_with(
            "cargo::error=hilavitkutin-build: ClangDriver, in the dev set of the prototype that claims the \
             workspace, waits on clang,"
        ),
        "{no_clang}"
    );

    // A target that does not admit the pragma, with its tool there.
    assert_eq!(
        uncarried::<BothCranelift>(target("unknown", "wasm32"), &[]),
        "cargo::error=hilavitkutin-build: Cranelift, in the dev set of the prototype that claims the workspace, has \
         no meaning on wasm, which this target is: the generated cargo file is left as it was\n\
         cargo::error=hilavitkutin-build: Cranelift, in the release set of the prototype that claims the workspace, \
         has no meaning on wasm, which this target is: the generated cargo file is left as it was\n"
    );
}

/// A pragma of crate reach has no key in the file, and a tool with no
/// source yet is lacked by nobody, so neither is asked about: the walk
/// never calls the script for them.
#[test]
fn only_a_workspace_pragma_with_a_tool_that_has_a_source_is_asked_about() {
    struct Crateish;

    impl Prototype for Crateish {
        type Ci = ();
        type Dev = (Mold, FastMath);
        type DevOpt = ();
        type Profiling = ();
        type Release = (Mold, Pgo, ExpandedLto);
    }

    let mut asked = Vec::new();
    let mut out = String::new();
    write_uncarried::<Crateish, _>(
        &target("unknown", "wasm32"),
        |tool| {
            asked.push(tool.name);
            true
        },
        &mut out,
    )
    .expect("a String takes every write");
    assert_eq!(asked, Vec::<&str>::new());
    assert_eq!(out, "");

    let mut asked = Vec::new();
    write_uncarried::<Everything, _>(
        &target("linux", "x86_64"),
        |tool| {
            asked.push(tool.name);
            false
        },
        &mut out,
    )
    .expect("a String takes every write");
    assert_eq!(asked, ["cranelift", "clang", "clang", "clang", "clang"]);
}

#[test]
fn the_crates_own_prototypes_write_what_the_design_says() {
    assert_eq!(
        generated::<Workspace>("hilavitkutin"),
        "# written by hilavitkutin-build for hilavitkutin: edits are lost\n\
         profile.release.lto = \"fat\"\n\
         profile.release.codegen-units = 1\n"
    );
    // A prototype with no pragma of workspace reach writes its claim alone.
    assert_eq!(
        generated::<Default>("hilavitkutin"),
        "# written by hilavitkutin-build for hilavitkutin: edits are lost\n"
    );
}

/// The file this workspace commits is the one its engine crate's script
/// writes, so a fresh clone builds with the keys from its first build.
#[test]
fn the_committed_file_is_the_written_one() {
    assert_eq!(
        include_str!("../../../.cargo/hilavitkutin-build.toml"),
        generated::<Workspace>("hilavitkutin")
    );
}

#[test]
fn a_file_is_claimed_by_the_crate_its_first_line_names() {
    let mine = generated::<Workspace>("hilavitkutin");
    assert_eq!(claim(Maybe::Is(&mine), "hilavitkutin"), Claim::Mine);
    assert_eq!(
        claim(Maybe::Is(&mine), "kaski-app"),
        Claim::Theirs("hilavitkutin")
    );
    // A name that only starts or ends the same is another crate's.
    assert_eq!(
        claim(Maybe::Is(&mine), "hilavitkutin-api"),
        Claim::Theirs("hilavitkutin")
    );
    assert_eq!(
        claim(Maybe::Is(&mine), "hila"),
        Claim::Theirs("hilavitkutin")
    );
    assert_eq!(claim(Maybe::Isnt, "hilavitkutin"), Claim::Free);

    // A file this crate did not write is somebody else's, never free.
    for foreign in [
        "",
        "\n",
        "profile.release.lto = \"fat\"\n",
        "# written by hand\n",
        "# written by hilavitkutin-build for hilavitkutin\n",
        "\n# written by hilavitkutin-build for hilavitkutin: edits are lost\n",
        " # written by hilavitkutin-build for hilavitkutin: edits are lost\n",
    ] {
        assert_eq!(
            claim(Maybe::Is(foreign), "hilavitkutin"),
            Claim::Theirs(""),
            "{foreign:?}"
        );
    }
    // The claim is the first line alone, whatever follows it.
    assert_eq!(
        claim(
            Maybe::Is("# written by hilavitkutin-build for a: edits are lost\ngarbage"),
            "a"
        ),
        Claim::Mine
    );
}

#[test]
fn a_config_file_takes_the_generated_file_by_an_include_before_any_table() {
    assert!(INCLUDE_LINE.contains(GENERATED_NAME));
    let mut written = String::new();
    write_cargo_config(&mut written).expect("a String takes every write");

    let taking = [
        written.as_str(),
        INCLUDE_LINE,
        "include = \"hilavitkutin-build.toml\"\n",
        "include = 'hilavitkutin-build.toml'\n",
        "include   =   [\"other.toml\", \"hilavitkutin-build.toml\"]\n[build]\njobs = 2\n",
        "# a comment\njobs = 1\ninclude = [\n    { path = \"other.toml\" },\n    { path = \"hilavitkutin-build.toml\", optional = true },\n]\n\n[build]\n",
    ];
    for text in taking {
        assert!(includes_generated(text), "{text:?}");
    }

    let not_taking = [
        "",
        "[build]\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
        "include = \"other.toml\"\n",
        "include = [\"my-hilavitkutin-build.toml\"]\n",
        "include = [{ path = \"sub/hilavitkutin-build.toml\" }]\n",
        "include = [\"hilavitkutin-build.toml.bak\"]\n",
        "# include = \"hilavitkutin-build.toml\"\n",
        "[env]\ninclude = \"hilavitkutin-build.toml\"\n",
        "include = \"other.toml\"\n[env]\nGENERATED = \"hilavitkutin-build.toml\"\n",
        "include = \"other.toml\"\nnote = \"hilavitkutin-build.toml\"\n",
        "included = \"hilavitkutin-build.toml\"\n",
    ];
    for text in not_taking {
        assert!(!includes_generated(text), "{text:?}");
    }
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
    assert!(write_generated::<Workspace, _>("a", "t", &mut Refusing).is_err());
}
