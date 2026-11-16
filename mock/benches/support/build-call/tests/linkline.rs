//! The printed link line read back, and a line it cannot read refused.

use build_call::linkline::{LinkLine, parse};

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
