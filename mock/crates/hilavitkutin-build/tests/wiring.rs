//! What a build script does about the workspace's wiring is decided by
//! `plan`, from what the script read. These hold `plan` to a table written
//! out by hand over every input it has, `ConfigFile::read` to the texts a
//! config file can carry, and the two texts the crate writes to the letter.

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
};
use notko::Maybe;

/// The values `RUSTC_WORKSPACE_WRAPPER` takes: unset, empty, the wrapper,
/// cargo's own under `cargo clippy`, and somebody else's.
const UNSET: Maybe<&str> = Maybe::Isnt;
const EMPTY: Maybe<&str> = Maybe::Is("");
const OURS: Maybe<&str> = Maybe::Is("/repo/.cargo/hilavitkutin-rustc-wrapper");
const CLIPPY: Maybe<&str> = Maybe::Is("/home/u/.rustup/toolchains/nightly/bin/clippy-driver");
const OTHER: Maybe<&str> = Maybe::Is("/usr/bin/sccache");

const WRAPPER_FILES: [WrapperFile; 3] =
    [WrapperFile::Current, WrapperFile::Stale, WrapperFile::Missing];

/// What the script says in the consumer's own tree, one row for each
/// wrapper value, config file and whether there are flags. The wrapper
/// file is not a column: it never changes what is said.
const SAID: [(Maybe<&str>, ConfigFile, bool, Notice); 30] = [
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

#[test]
fn the_table_has_a_row_for_every_wrapper_value_config_file_and_flags() {
    for wrapper in [UNSET, EMPTY, OURS, CLIPPY, OTHER] {
        for config_file in [ConfigFile::Missing, ConfigFile::Unnamed, ConfigFile::Named] {
            for flags in [true, false] {
                let rows = SAID
                    .iter()
                    .filter(|row| row.0 == wrapper && row.1 == config_file && row.2 == flags)
                    .count();
                assert_eq!(rows, 1, "{wrapper:?} {config_file:?} {flags}");
            }
        }
    }
}

#[test]
fn the_own_tree_is_written_what_is_missing_and_told_what_the_table_says() {
    for (wrapper, config_file, flags, notice) in SAID {
        for wrapper_file in WRAPPER_FILES {
            let expected = Plan {
                wrapper: wrapper_file != WrapperFile::Current,
                config: config_file == ConfigFile::Missing,
                notice,
            };
            assert_eq!(
                plan(wrapper, Tree::Own, wrapper_file, config_file, flags),
                expected,
                "{wrapper:?} {wrapper_file:?} {config_file:?} flags {flags}"
            );
        }
    }
}

#[test]
fn a_fetched_tree_is_written_nothing_and_told_nothing() {
    let nothing = Plan {
        wrapper: false,
        config:  false,
        notice:  Notice::Silent,
    };
    for (wrapper, config_file, flags, _) in SAID {
        for wrapper_file in WRAPPER_FILES {
            assert_eq!(
                plan(wrapper, Tree::Fetched, wrapper_file, config_file, flags),
                nothing,
                "{wrapper:?} {wrapper_file:?} {config_file:?} flags {flags}"
            );
        }
    }
}

/// The two cases a plan that only looked at the wrapper variable would get
/// wrong, each beside the own-tree case it must differ from.
#[test]
fn the_tree_and_the_wrapper_file_each_change_the_plan() {
    let fetched = plan(
        UNSET,
        Tree::Fetched,
        WrapperFile::Missing,
        ConfigFile::Missing,
        true,
    );
    let own = plan(
        UNSET,
        Tree::Own,
        WrapperFile::Missing,
        ConfigFile::Missing,
        true,
    );
    assert!(!fetched.wrapper && !fetched.config);
    assert!(own.wrapper && own.config);
    assert_ne!(fetched, own);

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
    assert!(!current.wrapper);
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
fn a_config_file_names_the_wrapper_by_a_key_that_ends_with_its_name() {
    for text in [
        "[build]\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
        "[build]\n  rustc-workspace-wrapper=\"/abs/.cargo/hilavitkutin-rustc-wrapper\"\n",
        "[build]\nrustc-workspace-wrapper = '.cargo/hilavitkutin-rustc-wrapper' # ours\n",
        "build.rustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
        "[build]\nincremental = true\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"",
    ] {
        assert_eq!(ConfigFile::read(text), ConfigFile::Named, "{text:?}");
    }
}

#[test]
fn a_config_file_without_the_key_or_with_another_wrapper_names_none() {
    for text in [
        "",
        "[build]\nincremental = true\n",
        "[build]\nrustc-workspace-wrapper = \"/usr/bin/sccache\"\n",
        "[build]\n# rustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
        "[build]\nrustc-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"\n",
        "[build]\nrustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper-old\"\n",
        "[env]\nNOTE = \"rustc-workspace-wrapper = .cargo/hilavitkutin-rustc-wrapper\"\n",
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
        "cargo::warning=hilavitkutin-build wrote hilavitkutin-rustc-wrapper and the config.toml that names it \
         under /repo/.cargo: this build ran without the wrapper, and the next one takes the flags\n"
    );
    assert_eq!(
        notice(Notice::AddLine),
        "cargo::warning=hilavitkutin-build left /repo/.cargo/config.toml as it is, and it names no wrapper: add \
         `rustc-workspace-wrapper = \".cargo/hilavitkutin-rustc-wrapper\"` under `[build]`, or the flags chosen \
         for this crate are dropped\n"
    );
    assert_eq!(
        notice(Notice::Dropped),
        "cargo::warning=hilavitkutin-build chose rustc flags for this crate and cargo runs it without the wrapper \
         /repo/.cargo/config.toml names, so they are dropped: run cargo from inside the workspace\n"
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
fn a_refusing_writer_is_reported_by_both_texts() {
    assert!(write_cargo_config(&mut Refusing).is_err());
    for said in [Notice::Wired, Notice::AddLine, Notice::Dropped] {
        assert!(
            write_notice(said, "/repo/.cargo", &mut Refusing).is_err(),
            "{said:?}"
        );
    }
    assert!(write_notice(Notice::Silent, "/repo/.cargo", &mut Refusing).is_ok());
}
