//! Tests for `engine-spawn-through-gate`: each refused shape planted in a
//! fixture, and each look-alike that has to pass.

use super::*;

/// The shape the engine ships, trimmed to what the lint reads.
const SHIPPED: &str = r#"
use hilavitkutin_api::platform::{OnePointerClosure, ThreadPoolApi};

/// Hand `f` to the executor after forcing the gate. `pool.spawn(f)` in a
/// doc comment is not a call.
fn spawn_one_pointer<P, F>(pool: &P, f: F)
where
    P: ThreadPoolApi,
    F: FnOnce() + Send + 'static,
{
    let () = OnePointerClosure::<F>::FITS;
    pool.spawn(f);
}

fn run_parallel<P: ThreadPoolApi>(pool: &P) {
    let mut c = 0;
    while c < 4 {
        spawn_one_pointer(pool, move || {
            let cp = c;
        });
        c += 1;
    }
}
"#;

fn lines(text: &str) -> Vec<usize> {
    findings(text).into_iter().map(|f| f.line).collect()
}

fn line_of(text: &str, needle: &str) -> usize {
    text.lines()
        .position(|l| l.contains(needle))
        .expect("needle in fixture")
        + 1
}

// ---- what passes ------------------------------------------------------------

#[test]
fn the_shipped_shape_is_clean() {
    assert_eq!(findings(SHIPPED), Vec::new());
}

#[test]
fn calling_the_gate_fn_is_not_a_spawn() {
    assert!(spawn_calls("spawn_one_pointer(pool, f);").is_empty());
    assert!(spawn_calls("self.spawn_one_pointer(pool, f);").is_empty());
}

#[test]
fn words_that_contain_spawn_are_not_calls() {
    for code in [
        "let spawned = x.spawned.0;",
        "respawn(f);",
        "x.respawn(f);",
        "let spawn = 3;",
        "if spawn { }",
        "x.spawn_count",
        "fn spawn(&self, f: F)",
    ] {
        assert!(spawn_calls(code).is_empty(), "{code}");
    }
}

#[test]
fn comments_and_strings_are_not_read() {
    let text = "\
fn f<P: ThreadPoolApi>(pool: &P) {
    // pool.spawn(f);
    /* pool.spawn(f); */
    /// pool.spawn(f)
    //! pool.spawn(f)
    let s = \"pool.spawn(f)\";
    let q = '\"'; let t = \"pool.spawn(g)\";
    let e = \"a \\\" pool.spawn(h)\";
}
";
    assert_eq!(findings(text), Vec::new());
}

#[test]
fn a_multi_line_block_comment_is_not_read() {
    let text = "fn f() {\n    /*\n    pool.spawn(f);\n    */\n}\n";
    assert_eq!(findings(text), Vec::new());
}

#[test]
fn a_lifetime_does_not_open_a_char_literal() {
    let text = "fn f<'a, P: ThreadPoolApi>(pool: &'a P) { pool.spawn(g); }\n";
    assert_eq!(lines(text), vec![1]);
}

#[test]
fn the_allow_escape_silences_its_line() {
    let text = "fn f() {\n    pool.spawn(g); // lint:allow(engine-spawn-through-gate) reason: x; tracked: #1\n}\n";
    assert_eq!(findings(text), Vec::new());
}

#[test]
fn only_the_engine_crate_is_checked() {
    assert!(applies_to("hilavitkutin"));
    for other in ["hilavitkutin-api", "hilavitkutin-providers", "hilavitkutin-kit", "arvo"] {
        assert!(!applies_to(other), "{other}");
    }
}

// ---- what is refused --------------------------------------------------------

#[test]
fn a_direct_spawn_elsewhere_in_the_engine_is_refused() {
    let text = SHIPPED.replace("spawn_one_pointer(pool, move || {", "pool.spawn(move || {");
    assert_eq!(lines(&text), vec![line_of(&text, "pool.spawn(move ||")]);
}

#[test]
fn every_call_shape_outside_the_gate_is_refused() {
    for call in [
        "pool.spawn(f);",
        "pool . spawn (f);",
        "pool.spawn::<F>(f);",
        "P::spawn(pool, f);",
        "ThreadPoolApi::spawn(pool, f);",
        "<P as ThreadPoolApi>::spawn(pool, f);",
        "self.pool.spawn(f);",
        "std::thread::spawn(f);",
    ] {
        let text = format!("fn g<P: ThreadPoolApi>(pool: &P) {{\n    {call}\n}}\n");
        assert_eq!(lines(&text), vec![2], "{call}");
    }
}

#[test]
fn deleting_the_forcing_statement_is_refused() {
    let text = SHIPPED.replace("    let () = OnePointerClosure::<F>::FITS;\n", "");
    assert_eq!(lines(&text), vec![line_of(&text, "pool.spawn(f);")]);
}

#[test]
fn a_forcing_that_does_not_bind_unit_is_refused() {
    for forcing in [
        "let _ = OnePointerClosure::<F>::FITS;",
        "OnePointerClosure::<F>::FITS;",
        "let () = OnePointerClosure::<F>::OTHER;",
        "let () = SomethingElse::<F>::FITS;",
    ] {
        let text = SHIPPED.replace("let () = OnePointerClosure::<F>::FITS;", forcing);
        assert_eq!(
            lines(&text),
            vec![line_of(&text, "pool.spawn(f);")],
            "{forcing}"
        );
    }
}

#[test]
fn forcing_after_the_spawn_is_refused() {
    let text = SHIPPED.replace(
        "    let () = OnePointerClosure::<F>::FITS;\n    pool.spawn(f);\n",
        "    pool.spawn(f);\n    let () = OnePointerClosure::<F>::FITS;\n",
    );
    assert_eq!(lines(&text), vec![line_of(&text, "pool.spawn(f);")]);
}

#[test]
fn a_second_spawn_after_the_gate_body_closes_is_refused() {
    let text = format!(
        "{SHIPPED}\nfn later<P: ThreadPoolApi>(pool: &P) {{\n    pool.spawn(|| {{}});\n}}\n"
    );
    assert_eq!(lines(&text), vec![line_of(&text, "pool.spawn(|| {});")]);
}

#[test]
fn nested_braces_in_the_gate_body_keep_it_open() {
    let text = SHIPPED.replace(
        "    pool.spawn(f);\n",
        "    if true {\n        let x = { 1 };\n    }\n    pool.spawn(f);\n",
    );
    assert_eq!(findings(&text), Vec::new());
}

#[test]
fn a_one_line_gate_fn_is_read() {
    let ok = "fn spawn_one_pointer<P: ThreadPoolApi, F>(pool: &P, f: F) { let () = OnePointerClosure::<F>::FITS; pool.spawn(f); }\n";
    assert_eq!(findings(ok), Vec::new());
    let bad = "fn spawn_one_pointer<P: ThreadPoolApi, F>(pool: &P, f: F) { pool.spawn(f); }\n";
    assert_eq!(lines(bad), vec![1]);
}

#[test]
fn a_gate_fn_by_a_longer_name_does_not_count() {
    let text = SHIPPED.replace("fn spawn_one_pointer<", "fn spawn_one_pointer_unchecked<");
    let spawn = line_of(&text, "pool.spawn(f);");
    // Its forcing no longer counts, so its spawn is outside the gate.
    assert_eq!(lines(&text), vec![spawn]);
}

#[test]
fn a_finding_carries_a_message_naming_the_gate() {
    let text = "fn g() {\n    pool.spawn(f);\n}\n";
    let found = findings(text);
    assert_eq!(found.len(), 1);
    assert!(
        found[0].message.contains("spawn_one_pointer"),
        "{}",
        found[0].message
    );
}
