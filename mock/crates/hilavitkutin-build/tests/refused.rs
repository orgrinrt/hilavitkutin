//! A set that breaks a rule is refused where it is named, before any
//! build script runs. Each case here is a scratch crate whose script is
//! the one line, put through `cargo check`: the broken set fails with its
//! rule's own sentence, and beside it the set that keeps the rule does
//! not, so a check that refused everything would be seen.

mod harness;

use harness::{run_cargo, workspace_with};

/// A prototype of the script's own file with the three sets a case
/// varies; the other two are empty.
fn prototype(dev: &str, release: &str, ci: &str) -> String {
    format!(
        "use hilavitkutin_build::*;\nstruct Mine;\nimpl Prototype for Mine {{\n    type Dev = {dev};\n    type \
         DevOpt = ();\n    type Release = {release};\n    type Profiling = ();\n    type Ci = {ci};\n}}\n"
    )
}

/// Check the crate whose script names `Mine` with these sets, claiming
/// the workspace or not, and answer what cargo said.
fn checked(name: &str, sets: [&str; 3], claims: bool) -> harness::Built {
    let script = if claims { "crate::Mine, workspace" } else { "crate::Mine" };
    let root = workspace_with(name, &prototype(sets[0], sets[1], sets[2]), script);
    run_cargo(&root, "check", &[], &[])
}

/// The broken set is refused with `sentence`, as a compile error of the
/// script. The kept set is not: it checks.
fn refused_beside_kept(
    name: &str,
    broken: [&str; 3],
    kept: [&str; 3],
    claims: bool,
    sentence: &str,
) {
    let refused = checked(&format!("{name}-broken"), broken, claims);
    assert!(
        !refused.passed,
        "{name}: the broken set checked\n{}",
        refused.said
    );
    assert!(refused.said.contains(sentence), "{name}: {}", refused.said);
    assert!(
        refused.said.contains("error[E0080]"),
        "{name}: refused, but not at compile time\n{}",
        refused.said
    );

    let passed = checked(&format!("{name}-kept"), kept, claims);
    assert!(
        passed.passed,
        "{name}: the kept set was refused\n{}",
        passed.said
    );
    assert!(!passed.said.contains(sentence), "{name}: {}", passed.said);
}

#[test]
fn a_requirement_of_each_form_is_refused_where_unmet() {
    refused_beside_kept(
        "all",
        ["(MathPeephole,)", "()", "()"],
        ["(MathPeephole, FastMath)", "()", "()"],
        false,
        "MathPeephole stands on FastMath, which the set does not hold",
    );
    refused_beside_kept(
        "any",
        ["(Profiling,)", "()", "()"],
        ["(Profiling, Bolt)", "()", "()"],
        false,
        "Profiling stands on Pgo or Bolt, and the set holds neither",
    );
    // The backend's own tool is a component this toolchain may lack, which
    // would fail the kept set when its script runs. The requirement is the
    // set's, so it is held in a profile no build here runs.
    refused_beside_kept(
        "none",
        ["()", "(Cranelift, ThinLto)", "()"],
        ["()", "(Cranelift, NoIncremental)", "()"],
        true,
        "Cranelift builds without LTO, and the set holds an LTO pragma",
    );
}

#[test]
fn a_group_held_twice_and_a_pragma_named_twice_are_refused() {
    refused_beside_kept(
        "group",
        ["(Bfd, FastMath, Mold)", "()", "()"],
        ["(Bfd, FastMath)", "()", "()"],
        false,
        "the set holds two linkers, and a crate is linked by one",
    );
    refused_beside_kept(
        "twice",
        ["(SharedGenerics, SharedGenerics)", "()", "()"],
        ["(SharedGenerics,)", "()", "()"],
        false,
        "the set names one pragma twice",
    );
}

/// Every one of the five sets is held to its rules, also the set of a
/// profile the build at hand is not of.
#[test]
fn a_set_of_a_profile_not_being_built_is_refused_all_the_same() {
    refused_beside_kept(
        "other-profile",
        ["()", "()", "(MathPeephole,)"],
        ["()", "()", "(MathPeephole, FastMath)"],
        false,
        "MathPeephole stands on FastMath, which the set does not hold",
    );
}

/// A pragma of workspace reach is named by the claiming script alone. The
/// same sets check in the script that claims.
#[test]
fn a_pragma_of_workspace_reach_is_refused_in_a_script_that_does_not_claim() {
    const REACH: &str = "the set holds a pragma that reaches the whole workspace";
    let sets = ["()", "(ThinLto,)", "()"];
    let refused = checked("reach-broken", sets, false);
    assert!(
        !refused.passed && refused.said.contains(REACH),
        "{}",
        refused.said
    );
    assert!(refused.said.contains("error[E0080]"), "{}", refused.said);

    let claimed = checked("reach-kept", sets, true);
    assert!(claimed.passed, "{}", claimed.said);
    assert!(!claimed.said.contains(REACH), "{}", claimed.said);
}

/// The crate's own `Workspace` prototype is refused the same way without
/// the claim, so the one line cannot name it by mistake.
#[test]
fn the_workspace_prototype_is_refused_without_the_claim() {
    let root = workspace_with("workspace-unclaimed", "", "Workspace");
    let refused = run_cargo(&root, "check", &[], &[]);
    assert!(!refused.passed, "{}", refused.said);
    assert!(
        refused
            .said
            .contains("the set holds a pragma that reaches the whole workspace"),
        "{}",
        refused.said
    );

    let root = workspace_with("workspace-claimed", "", "Workspace, workspace");
    let claimed = run_cargo(&root, "check", &[], &[]);
    assert!(claimed.passed, "{}", claimed.said);
}

/// A claiming prototype whose sets hold two link drivers across profiles
/// would write one key twice, so it is refused. Two prototypes are kept:
/// one with no driver in any set, and one with the same driver in all
/// five, which is the case a check that refused every driver would fail.
#[test]
fn a_claiming_prototype_whose_shared_groups_differ_is_refused() {
    const DIFFER: &str = "hold the same link driver and the same C compiler in every profile";
    let refused = checked(
        "shared-broken",
        ["(GccDriver,)", "(ClangDriver,)", "(GccDriver,)"],
        true,
    );
    assert!(
        !refused.passed && refused.said.contains(DIFFER),
        "{}",
        refused.said
    );
    assert!(refused.said.contains("error[E0080]"), "{}", refused.said);
    // A driver in three of the five sets and none in the other two differs too.
    let partial = checked(
        "shared-partial",
        ["(GccDriver,)", "(GccDriver,)", "(GccDriver,)"],
        true,
    );
    assert!(
        !partial.passed && partial.said.contains(DIFFER),
        "{}",
        partial.said
    );

    let none = checked("shared-kept", ["(ThinLto,)", "(ExpandedLto,)", "()"], true);
    assert!(none.passed, "{}", none.said);
    assert!(!none.said.contains(DIFFER), "{}", none.said);

    // gcc is the driver this toolchain links through by itself, so the
    // script that runs after the check finds its tool on any machine that
    // builds the suite.
    let every = "use hilavitkutin_build::*;\nstruct Mine;\nimpl Prototype for Mine {\n    type Dev = (GccDriver,);\n    \
                 type DevOpt = (GccDriver,);\n    type Release = (GccDriver, ThinLto);\n    type Profiling = \
                 (GccDriver,);\n    type Ci = (GccDriver,);\n}\n";
    let root = workspace_with("shared-every", every, "crate::Mine, workspace");
    let all = run_cargo(&root, "check", &[], &[]);
    assert!(all.passed, "{}", all.said);
    assert!(!all.said.contains(DIFFER), "{}", all.said);
}
