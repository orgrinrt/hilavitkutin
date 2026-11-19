//! What a build script does about the workspace's wiring is decided by
//! `plan`, from what the script read. These hold `plan` to tables written
//! out by hand over every input it has, `ConfigFile::read` to the texts a
//! config file can carry, and the texts the crate writes to the letter.

use hilavitkutin_build::{
    ConfigFile,
    Notice,
    Plan,
    Tree,
    WRAPPER_NAME,
    WrapperFile,
    plan,
    write_cargo_config,
    write_notice,
    write_watch,
};
use notko::Maybe;

/// The values `RUSTC_WORKSPACE_WRAPPER` takes: unset, empty, the wrapper,
/// cargo's own under `cargo clippy`, and somebody else's.
const UNSET: Maybe<&str> = Maybe::Isnt;
const EMPTY: Maybe<&str> = Maybe::Is("");
const OURS: Maybe<&str> = Maybe::Is("/repo/.cargo/hilavitkutin-rustc-wrapper");
const CLIPPY: Maybe<&str> = Maybe::Is("/home/u/.rustup/toolchains/nightly/bin/clippy-driver");
const OTHER: Maybe<&str> = Maybe::Is("/usr/bin/sccache");
/// Somebody else's again, whose file name only ends with the wrapper's.
const LONGER: Maybe<&str> = Maybe::Is("/repo/.cargo/my-hilavitkutin-rustc-wrapper");

/// Whether the wrapper is written in the consumer's own tree, by what the
/// script found on disk.
const WRAPPER_WRITTEN: [(WrapperFile, bool); 3] = [
    (WrapperFile::Current, false),
    (WrapperFile::Stale, true),
    (WrapperFile::Missing, true),
];

/// Whether the config file is written in the consumer's own tree, by what
/// the script found on disk.
const CONFIG_WRITTEN: [(ConfigFile, bool); 3] = [
    (ConfigFile::Named, false),
    (ConfigFile::Unnamed, false),
    (ConfigFile::Missing, true),
];

/// What the script says in the consumer's own tree, one row for each
/// wrapper value, config file and whether there are flags. The wrapper
/// file is not a column: it never changes what is said.
const SAID: [(Maybe<&str>, ConfigFile, bool, Notice); 36] = [
    (LONGER, ConfigFile::Missing, true, Notice::Wired),
    (LONGER, ConfigFile::Missing, false, Notice::Wired),
    (LONGER, ConfigFile::Unnamed, true, Notice::AddLine),
    (LONGER, ConfigFile::Unnamed, false, Notice::AddLine),
    (LONGER, ConfigFile::Named, true, Notice::Dropped),
    (LONGER, ConfigFile::Named, false, Notice::Silent),
    (UNSET, ConfigFile::Missing, true, Notice::Wired),
    (UNSET, ConfigFile::Missing, false, Notice::Wired),
    (UNSET, ConfigFile::Unnamed, true, Notice::AddLine),
    (UNSET, ConfigFile::Unnamed, false, Notice::AddLine),
    (UNSET, ConfigFile::Named, true, Notice::Dropped),
    (UNSET, ConfigFile::Named, false, Notice::Silent),
    (EMPTY, ConfigFile::Missing, true, Notice::Wired),
    (EMPTY, ConfigFile::Missing, false, Notice::Wired),
    (EMPTY, ConfigFile::Unnamed, true, Notice::AddLine),
    (EMPTY, ConfigFile::Unnamed, false, Notice::AddLine),
    (EMPTY, ConfigFile::Named, true, Notice::Dropped),
    (EMPTY, ConfigFile::Named, false, Notice::Silent),
    (OTHER, ConfigFile::Missing, true, Notice::Wired),
    (OTHER, ConfigFile::Missing, false, Notice::Wired),
    (OTHER, ConfigFile::Unnamed, true, Notice::AddLine),
    (OTHER, ConfigFile::Unnamed, false, Notice::AddLine),
    (OTHER, ConfigFile::Named, true, Notice::Dropped),
    (OTHER, ConfigFile::Named, false, Notice::Silent),
    (OURS, ConfigFile::Missing, true, Notice::Silent),
    (OURS, ConfigFile::Missing, false, Notice::Silent),
    (OURS, ConfigFile::Unnamed, true, Notice::Silent),
    (OURS, ConfigFile::Unnamed, false, Notice::Silent),
    (OURS, ConfigFile::Named, true, Notice::Silent),
    (OURS, ConfigFile::Named, false, Notice::Silent),
    (CLIPPY, ConfigFile::Missing, true, Notice::Silent),
    (CLIPPY, ConfigFile::Missing, false, Notice::Silent),
    (CLIPPY, ConfigFile::Unnamed, true, Notice::Silent),
    (CLIPPY, ConfigFile::Unnamed, false, Notice::Silent),
    (CLIPPY, ConfigFile::Named, true, Notice::Silent),
    (CLIPPY, ConfigFile::Named, false, Notice::Silent),
];

/// Every combination of inputs, as a row of the three tables joined: the
/// wrapper value, the wrapper file, the config file, whether there are
/// flags, and what the tables say of each.
fn every_case() -> Vec<(Maybe<&'static str>, WrapperFile, ConfigFile, bool, Plan)> {
    let mut cases = Vec::new();
    for (wrapper, config_file, flags, notice) in SAID {
        let config_written = CONFIG_WRITTEN
            .iter()
            .find(|row| row.0 == config_file)
            .expect("a row for every config file")
            .1;
        for (wrapper_file, wrapper_written) in WRAPPER_WRITTEN {
            cases.push((wrapper, wrapper_file, config_file, flags, Plan {
                wrapper: wrapper_written,
                config: config_written,
                notice,
            }));
        }
    }
    cases
}

/// The joined table covers the product of the inputs once each: five
/// wrapper values, three wrapper files, three config files, flags or none.
#[test]
fn the_tables_cover_every_input_once() {
    let cases = every_case();
    assert_eq!(cases.len(), 6 * 3 * 3 * 2);
    for wrapper in [UNSET, EMPTY, OURS, CLIPPY, OTHER, LONGER] {
        for wrapper_file in [WrapperFile::Current, WrapperFile::Stale, WrapperFile::Missing] {
            for config_file in [ConfigFile::Missing, ConfigFile::Unnamed, ConfigFile::Named] {
                for flags in [true, false] {
                    let rows = cases
                        .iter()
                        .filter(|case| {
                            case.0 == wrapper
                                && case.1 == wrapper_file
                                && case.2 == config_file
                                && case.3 == flags
                        })
                        .count();
                    assert_eq!(
                        rows, 1,
                        "{wrapper:?} {wrapper_file:?} {config_file:?} {flags}"
                    );
                }
            }
        }
    }
}

#[test]
fn the_own_tree_is_written_and_told_what_the_tables_say() {
    for (wrapper, wrapper_file, config_file, flags, expected) in every_case() {
        assert_eq!(
            plan(wrapper, Tree::Own, wrapper_file, config_file, flags),
            expected,
            "{wrapper:?} {wrapper_file:?} {config_file:?} flags {flags}"
        );
    }
}

#[test]
fn a_fetched_and_a_foreign_tree_are_written_nothing_and_told_nothing() {
    let nothing = Plan {
        wrapper: false,
        config:  false,
        notice:  Notice::Silent,
    };
    for tree in [Tree::Fetched, Tree::Foreign] {
        for (wrapper, wrapper_file, config_file, flags, _) in every_case() {
            assert_eq!(
                plan(wrapper, tree, wrapper_file, config_file, flags),
                nothing,
                "{tree:?} {wrapper:?} {wrapper_file:?} {config_file:?} flags {flags}"
            );
        }
    }
}

/// The cases a plan that only looked at the wrapper variable would get
/// wrong, each beside the own-tree case it must differ from.
#[test]
fn the_tree_and_the_wrapper_file_each_change_the_plan() {
    let own = plan(
        UNSET,
        Tree::Own,
        WrapperFile::Missing,
        ConfigFile::Missing,
        true,
    );
    assert_eq!(own, Plan {
        wrapper: true,
        config:  true,
        notice:  Notice::Wired,
    });
    for tree in [Tree::Fetched, Tree::Foreign] {
        let left_alone = plan(UNSET, tree, WrapperFile::Missing, ConfigFile::Missing, true);
        assert_ne!(left_alone, own, "{tree:?}");
    }

    let stale = plan(OURS, Tree::Own, WrapperFile::Stale, ConfigFile::Named, true);
    let current = plan(
        OURS,
        Tree::Own,
        WrapperFile::Current,
        ConfigFile::Named,
        true,
    );
    assert_eq!(stale, Plan {
        wrapper: true,
        config:  false,
        notice:  Notice::Silent,
    });
    assert_eq!(current, Plan {
        wrapper: false,
        config:  false,
        notice:  Notice::Silent,
    });
}

#[test]
fn a_plan_is_made_before_the_program_runs() {
    const WIRED: Plan = plan(
        UNSET,
        Tree::Own,
        WrapperFile::Missing,
        ConfigFile::Missing,
        true,
    );
    assert_eq!(WIRED.notice, Notice::Wired);
}

#[test]
fn a_config_file_names_the_wrapper_in_the_three_places_cargo_reads_it() {
    for text in [
        "[build]\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
        "[build]\n  rustc-workspace-wrapper=\"/abs/.cargo/hilavitkutin-rustc-wrapper\"\n",
        "[build]\nrustc-workspace-wrapper = \"hilavitkutin-rustc-wrapper\"\n",
        "[build]\nrustc-workspace-wrapper = \"C:\\\\repo\\\\.cargo\\\\hilavitkutin-rustc-wrapper\"\n",
        "[build]\nrustc-workspace-wrapper = '.cargo/hilavitkutin-rustc-wrapper' # ours\n",
        "[build]\nincremental = true\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"",
        "[env]\nA = \"b\"\n\n[ build ]\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
        "build.rustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
        "build = { rustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\" }\n",
        "build = { incremental = true, rustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\", jobs = 2 }\n",
    ] {
        assert_eq!(ConfigFile::read(text), ConfigFile::Named, "{text:?}");
    }
}

#[test]
fn a_config_file_names_none_without_the_key_or_with_it_anywhere_else() {
    for text in [
        "",
        "[build]\nincremental = true\n",
        "[build]\nrustc-workspace-wrapper = \"/usr/bin/sccache\"\n",
        "[build]\n# rustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
        "[build]\nrustc-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
        "[build]\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper-old\"\n",
        "[build]\nrustc-workspace-wrapper = \".cargo/my-hilavitkutin-rustc-wrapper\"\n",
        "[build]\nrustc-workspace-wrapper = \"/usr/bin/sccache\" # was hilavitkutin-rustc-wrapper\n",
        "[build]\nrustc-workspace-wrapper = .cargo/hilavitkutin-rustc-wrapper\n",
        "[env]\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
        "[build]\nincremental = true\n\n[env]\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
        "[env]\nNOTE = \"rustc-workspace-wrapper = .cargo/hilavitkutin-rustc-wrapper\"\n",
        "rustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
        "build = { rustc-workspace-wrapper = \"/usr/bin/sccache\" }\n",
        "builder.rustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
    ] {
        assert_eq!(ConfigFile::read(text), ConfigFile::Unnamed, "{text:?}");
    }
}

fn cargo_config() -> String {
    let mut out = String::new();
    write_cargo_config(&mut out).expect("writing to a String cannot fail");
    out
}

fn notice(notice: Notice) -> String {
    let mut out = String::new();
    write_notice(notice, "/repo/.cargo", &mut out).expect("writing to a String cannot fail");
    out
}

/// The file written where there is none is one the crate itself reads as
/// naming the wrapper, by the path cargo resolves against the workspace.
#[test]
fn the_written_config_is_the_build_table_with_the_one_line() {
    assert_eq!(
        cargo_config(),
        "[build]\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n"
    );
    assert_eq!(ConfigFile::read(&cargo_config()), ConfigFile::Named);
    assert_eq!(WRAPPER_NAME, "hilavitkutin-rustc-wrapper");
}

#[test]
fn each_notice_is_one_warning_line_and_silence_is_nothing() {
    assert_eq!(notice(Notice::Silent), "");
    assert_eq!(
        notice(Notice::Wired),
        "cargo::warning=hilavitkutin-build put hilavitkutin-rustc-wrapper and the config.toml that names it in \
         place under /repo/.cargo: this build ran without the wrapper, and a build of that workspace takes the \
         flags from the next one on\n"
    );
    assert_eq!(
        notice(Notice::AddLine),
        "cargo::warning=hilavitkutin-build left /repo/.cargo/config.toml as it is, and it names no wrapper: add \
         `rustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"` under `[build]`, or the flags chosen \
         for this crate are dropped\n"
    );
    assert_eq!(
        notice(Notice::Dropped),
        "cargo::warning=hilavitkutin-build chose rustc flags for this crate and cargo was started without the \
         wrapper /repo/.cargo/config.toml names, so they are dropped in this build: a build started inside the \
         workspace, once that file is there, takes them\n"
    );
    assert_eq!(
        notice(Notice::NoShell),
        "cargo::warning=hilavitkutin-build chose rustc flags for this crate and hands them over through a POSIX \
         sh wrapper, which this host does not run, so they are dropped\n"
    );
}

/// A host that cannot run the wrapper is written nothing whatever is on
/// disk, and told the flags are dropped only where there are flags.
#[test]
fn a_host_with_no_shell_is_written_nothing_and_told_of_dropped_flags() {
    for (wrapper, wrapper_file, config_file, flags, _) in every_case() {
        let expected = Plan {
            wrapper: false,
            config:  false,
            notice:  if flags { Notice::NoShell } else { Notice::Silent },
        };
        assert_eq!(
            plan(wrapper, Tree::NoShell, wrapper_file, config_file, flags),
            expected,
            "{wrapper:?} {wrapper_file:?} {config_file:?} flags {flags}"
        );
    }
}

/// The tree by what the script found: the root, cargo's home, whether the
/// root holds `Cargo.toml.orig`, whether cargo's wrapper sits elsewhere,
/// and whether the host is unix.
#[test]
fn the_tree_is_decided_from_the_root_the_cargo_home_and_the_host() {
    const HOME: Maybe<&str> = Maybe::Is("/home/u/.cargo");
    let cases: [(&str, Maybe<&str>, bool, bool, bool, Tree); 16] = [
        // The consumer's own workspace, on unix.
        ("/home/u/work/app", HOME, false, false, true, Tree::Own),
        (
            "/home/u/work/app",
            Maybe::Isnt,
            false,
            false,
            true,
            Tree::Own,
        ),
        // Under the cargo home, a checkout cargo fetched.
        (
            "/home/u/.cargo/git/checkouts/dep-1a2b/3c4d",
            HOME,
            false,
            false,
            true,
            Tree::Fetched,
        ),
        ("/home/u/.cargo", HOME, false, false, true, Tree::Fetched),
        (
            "/home/u/.cargo/registry/src/x",
            Maybe::Is("/home/u/.cargo/"),
            false,
            false,
            true,
            Tree::Fetched,
        ),
        (
            "C:\\Users\\u\\.cargo\\git\\x",
            Maybe::Is("C:\\Users\\u\\.cargo"),
            false,
            false,
            false,
            Tree::Fetched,
        ),
        // A root that only starts with the same letters is not under it.
        (
            "/home/u/.cargo-work/app",
            HOME,
            false,
            false,
            true,
            Tree::Own,
        ),
        ("/home/u/.carg", HOME, false, false, true, Tree::Own),
        // An empty cargo home holds nothing.
        (
            "/home/u/work/app",
            Maybe::Is(""),
            false,
            false,
            true,
            Tree::Own,
        ),
        // An unpacked crate, wherever it sits.
        (
            "/home/u/work/target/package/app-0.1.0",
            HOME,
            true,
            false,
            true,
            Tree::Fetched,
        ),
        // A fetched tree stays one on a host with no shell, and elsewhere.
        (
            "/home/u/work/target/package/app-0.1.0",
            HOME,
            true,
            true,
            false,
            Tree::Fetched,
        ),
        (
            "/home/u/.cargo/git/x",
            HOME,
            false,
            true,
            true,
            Tree::Fetched,
        ),
        // A host that is not unix, whatever cargo names.
        ("C:\\work\\app", HOME, false, false, false, Tree::NoShell),
        ("C:\\work\\app", HOME, false, true, false, Tree::NoShell),
        // Cargo's wrapper sits in another root.
        ("/home/u/work/app", HOME, false, true, true, Tree::Foreign),
        (
            "/home/u/work/app",
            Maybe::Isnt,
            false,
            true,
            true,
            Tree::Foreign,
        ),
    ];
    for (root, cargo_home, unpacked, elsewhere, unix, expected) in cases {
        assert_eq!(
            Tree::of(root, cargo_home, unpacked, elsewhere, unix),
            expected,
            "{root} {cargo_home:?} unpacked {unpacked} elsewhere {elsewhere} unix {unix}"
        );
    }
    const DECIDED: Tree = Tree::of("/home/u/work/app", HOME, false, false, true);
    assert_eq!(DECIDED, Tree::Own);
}

#[test]
fn the_watch_lines_name_the_two_files_of_the_wiring() {
    let mut out = String::new();
    write_watch("/repo/.cargo", &mut out).expect("writing to a String cannot fail");
    assert_eq!(
        out,
        "cargo::rerun-if-changed=/repo/.cargo/hilavitkutin-rustc-wrapper\n\
         cargo::rerun-if-changed=/repo/.cargo/config.toml\n"
    );
}

/// A sink that refuses every write.
struct Refusing;

impl core::fmt::Write for Refusing {
    fn write_str(&mut self, _: &str) -> core::fmt::Result {
        Err(core::fmt::Error)
    }
}

#[test]
fn a_refusing_writer_is_reported_by_every_text() {
    assert!(write_cargo_config(&mut Refusing).is_err());
    assert!(write_watch("/repo/.cargo", &mut Refusing).is_err());
    for said in [Notice::Wired, Notice::AddLine, Notice::Dropped, Notice::NoShell] {
        assert!(
            write_notice(said, "/repo/.cargo", &mut Refusing).is_err(),
            "{said:?}"
        );
    }
    assert!(write_notice(Notice::Silent, "/repo/.cargo", &mut Refusing).is_ok());
}
