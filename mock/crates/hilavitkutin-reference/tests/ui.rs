//! The compile-fail pairs: each refusal beside a twin that compiles, its diagnostic committed.

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}

#[test]
#[ignore = "catalogue: a role list naming one column twice, bound with its places written out, puts one column's members on two roles and builds; tracked #hilavitkutin_catalogue_explicit_places_bind_one_column_s_mem"]
fn explicit_places_keep_one_role_per_column() {
    // FIXME(64c5363cc578): nothing refuses two places for one column once `P` is written out.
    trybuild::TestCases::new().compile_fail("tests/ui/catalogue/explicit_places.rs");
}
