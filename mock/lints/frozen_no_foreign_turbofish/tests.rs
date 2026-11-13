//! Tests for `frozen-no-foreign-turbofish`: each refused shape, and each
//! look-alike that has to pass.

use super::*;

fn lines(text: &str) -> Vec<usize> {
    findings(text).into_iter().map(|f| f.line).collect()
}

#[test]
fn a_turbofish_on_a_contract_function_is_refused() {
    let text = "use hilavitkutin_contracts::bind;\nfn go() {\n    let b = bind::<Plane, L, S>(&t, &s);\n}\n";
    assert_eq!(lines(text), vec![3]);
}

#[test]
fn a_turbofish_on_a_foreign_method_is_refused() {
    assert_eq!(lines("fn go() { let w = r.get::<Heat>(c, i); }\n"), vec![1]);
}

#[test]
fn a_turbofish_on_a_foreign_type_is_refused() {
    assert_eq!(lines("fn go() { let m = PhantomData::<u8>; }\n"), vec![1]);
}

#[test]
fn a_qualified_path_turbofish_is_refused() {
    assert_eq!(
        lines("fn go() { let w = <K as ValueKernel>::at::<X>(r); }\n"),
        vec![1]
    );
}

#[test]
fn spacing_does_not_hide_it() {
    assert_eq!(lines("fn go() { bind :: < A > (t); }\n"), vec![1]);
}

#[test]
fn a_turbofish_on_an_item_the_file_declares_passes() {
    let text = "pub struct Columns<const N: usize>;\nimpl<const N: usize> Columns<N> { const fn new() -> Self { Self } }\nconst fn width<const N: usize>() -> usize { N }\nfn go() { let c = Columns::<4>::new(); let w = width::<4>(); }\n";
    assert!(lines(text).is_empty());
}

#[test]
fn const_fn_declares_the_function_not_fn() {
    let names = declared(
        "pub const fn width() {}\nconst LIMIT: () = ();\nmacro_rules! offsets { () => {} }\n",
    );
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    assert_eq!(names, vec!["LIMIT", "offsets", "width"]);
}

#[test]
fn a_declaration_elsewhere_in_the_crate_does_not_count() {
    assert_eq!(
        lines("use crate::plane::Plane;\nfn go() { Plane::<A>::new(); }\n"),
        vec![2]
    );
}

#[test]
fn comments_and_strings_are_not_read() {
    assert!(
        lines("// bind::<A>(t)\n/// bind::<A>(t)\nconst S: &str = \"bind::<A>(t)\";\n").is_empty()
    );
}

#[test]
fn no_escape_lets_a_line_pass() {
    assert_eq!(
        lines("fn go() { bind::<A>(t); } // lint:allow(frozen-no-foreign-turbofish) probing\n")
            .len(),
        1,
        "a lint:allow comment is not read"
    );
}
