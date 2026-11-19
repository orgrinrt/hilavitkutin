//! The printed link line read back, a line it cannot read refused, and a
//! kept line held to the profile it is replayed for.

use std::path::Path;

use build_call::linkline::{LinkLine, parse};
use build_call::{Dirs, Profile};

#[test]
fn each_profile_keeps_its_link_line_in_a_file_of_its_own() {
    let dirs = Dirs::of("test-linkline", "default_replay", 25);
    assert_ne!(
        dirs.link_line(Profile::Dev),
        dirs.link_line(Profile::Release)
    );
    assert!(dirs.link_line(Profile::Dev).starts_with(dirs.dir()));
}

#[test]
fn a_line_writing_outside_the_replayed_profile_is_refused() {
    let debug = Path::new("/c/target/debug");
    let line = |printed: &str| parse(printed).expect("a printed command");
    let own = line(r#""cc" "-o" "/c/target/debug/deps/fx_facade-1""#);
    assert_eq!(own.output_under(debug), Ok(debug.join("deps/fx_facade-1")));
    let relative = line(r#"cd "/c/target/debug" && "cc" "-o" "deps/fx_facade-1""#);
    assert_eq!(
        relative.output_under(debug),
        Ok(debug.join("deps/fx_facade-1"))
    );
    for (what, printed) in [
        (
            "the release profile's line",
            r#""cc" "-o" "/c/target/release/deps/fx_facade-1""#,
        ),
        (
            "a line climbing out of the profile",
            r#""cc" "-o" "/c/target/debug/../release/deps/fx_facade-1""#,
        ),
        (
            "a directory that only starts like the profile's",
            r#""cc" "-o" "/c/target/debugx/deps/fx_facade-1""#,
        ),
        (
            "a relative path from elsewhere",
            r#"cd "/c/target/release" && "cc" "-o" "deps/fx_facade-1""#,
        ),
        ("a line with no output", r#""cc" "x.o""#),
    ] {
        let why = line(printed).output_under(debug).expect_err(what);
        assert!(
            why.contains("not under") || why.contains("no `-o`"),
            "{what}: {why}"
        );
    }
}

#[test]
fn a_printed_command_reads_back_into_its_parts() {
    let printed = r#"LC_ALL="C" PATH="/a/bin:/usr/bin" VSLANG="1033" "cc" "-m64" "/t/rustcX/symbols.o" "-o" "/t/deps/fx_facade-1" "-Wl,--as-needed" "q\"uote\\d""#;
    let l = parse(printed).expect("a printed command");
    assert_eq!(l.set[0], ("LC_ALL".to_string(), "C".to_string()));
    assert_eq!(l.set.len(), 3);
    assert_eq!(l.argv[0], "cc");
    assert_eq!(l.output(), Some("/t/deps/fx_facade-1"));
    assert_eq!(l.argv.last().map(String::as_str), Some("q\"uote\\d"));
    assert_eq!(LinkLine::from_text(&l.to_text()), Some(l));
}

#[test]
fn a_cwd_and_a_removed_variable_are_kept() {
    let l = parse(r#"cd "/w" && env -u FOO A="1" "ld" "-o" "x""#).expect("a printed command");
    assert_eq!(l.cwd.as_deref(), Some("/w"));
    assert_eq!(l.removed, vec!["FOO".to_string()]);
    assert_eq!(l.argv, vec!["ld", "-o", "x"]);
}

#[test]
fn an_unterminated_or_unquoted_word_is_refused() {
    assert!(parse(r#""cc" "-o"#).is_err());
    assert!(parse(r#""cc" -o "x""#).is_err());
    assert!(
        parse(r#""cc" A="1""#).is_err(),
        "a variable after the program"
    );
    assert!(parse("").is_err());
}
