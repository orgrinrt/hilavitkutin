//! The small point built for real through the same calls the arms make, and
//! the checks the hooks library applies, shown passing on honest calls and
//! refusing each broken setup: a clean that removes nothing, an edit by file
//! time only, and a link that leaves the executable in place.
//!
//! Each test uses a bench name of its own, so their records and directories
//! never meet.

use build_call::record::{self, Line};
use build_call::{Arm, Call, Control, Profile, Scenario, root};

const N: usize = 25;

fn input(s: Scenario) -> [u8; 8] {
    [s.code(), Profile::Dev.code(), 0, 0, 0, 0, 0, 0]
}

/// One call through ready, run and record, and the line it appended. Calls
/// run one at a time, as a bench's workers do, so no two cargos wait on each
/// other's locks.
fn call(
    bench: &'static str,
    arm_name: &'static str,
    arm: Arm,
    s: Scenario,
    control: Option<Control>,
) -> Line {
    static ONE_AT_A_TIME: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _turn = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut c = Call::ready_with(bench, arm_name, arm, &input(s), N, control);
    let out = c.run();
    c.record(&out);
    let (lines, _) = record::read(&root(), bench);
    lines.last().cloned().expect("the call appended a line")
}

fn checked(bench: &str, s: Scenario, lines: &[Line]) -> Result<String, String> {
    record::check(lines, bench, s, Profile::Dev, N)
}

const OFF: Arm = Arm::Build {
    incremental: false,
};

#[test]
fn every_scenario_compiles_what_the_generator_predicts_offline_and_locked() {
    let bench = "test-scenarios";
    record::reset(&root(), bench);
    for s in [Scenario::Clean, Scenario::Leaf, Scenario::Mid, Scenario::Root] {
        let line = call(bench, "honest", OFF, s, None);
        let verdict = checked(bench, s, std::slice::from_ref(&line));
        assert!(verdict.is_ok(), "{s:?}: {verdict:?} for {line:?}");
    }
    let (lines, _) = record::read(&root(), bench);
    let compiled: Vec<usize> = lines.iter().map(|l| l.compiled).collect();
    assert_eq!(
        compiled,
        vec![25, 2, 7, 19],
        "clean, leaf, mid, root at n=25"
    );
}

#[test]
fn a_clean_that_removes_nothing_is_refused() {
    let bench = "test-skip-clean";
    record::reset(&root(), bench);
    call(bench, "control", OFF, Scenario::Clean, None);
    let line = call(
        bench,
        "control",
        OFF,
        Scenario::Clean,
        Some(Control::SkipClean),
    );
    let verdict = checked(bench, Scenario::Clean, std::slice::from_ref(&line));
    let why = verdict.expect_err("a clean that removed nothing must be refused");
    assert!(why.contains("compiled 0 crates"), "{why}");
    assert!(why.contains("target directory was not empty"), "{why}");
}

#[test]
fn an_edit_by_file_time_only_is_refused() {
    let bench = "test-time-only";
    record::reset(&root(), bench);
    let line = call(
        bench,
        "control",
        OFF,
        Scenario::Leaf,
        Some(Control::TimeOnly),
    );
    let verdict = checked(bench, Scenario::Leaf, std::slice::from_ref(&line));
    let why = verdict.expect_err("an edit of the file time alone must be refused");
    assert!(why.contains("compiled 0 crates"), "{why}");
}

#[test]
fn a_relink_relinks_only_the_facade_and_one_that_keeps_the_executable_is_refused() {
    let bench = "test-relink";
    record::reset(&root(), bench);
    let first = call(bench, "relink", Arm::Relink, Scenario::Link, None);
    let second = call(bench, "relink", Arm::Relink, Scenario::Link, None);
    assert_eq!(
        second.setup_compiled, 0,
        "switching nothing recompiles nothing: {second:?}"
    );
    let ok = checked(bench, Scenario::Link, &[first.clone(), second]);
    assert!(ok.is_ok(), "{ok:?}");
    assert!(
        first.mark.starts_with("Linker: LLD"),
        "the default linker's mark: {}",
        first.mark
    );
    let kept = call(
        bench,
        "relink",
        Arm::Relink,
        Scenario::Link,
        Some(Control::KeepOutput),
    );
    let why = checked(bench, Scenario::Link, &[kept])
        .expect_err("a link that kept the executable must be refused");
    assert!(why.contains("compiled 0 crates"), "{why}");
}

#[test]
fn the_recorded_link_line_replays_to_the_same_program() {
    let bench = "test-replay";
    record::reset(&root(), bench);
    let relinked = call(bench, "default_relink", Arm::Relink, Scenario::Link, None);
    let replayed = call(bench, "default_replay", Arm::Replay, Scenario::Link, None);
    let again = call(bench, "default_replay", Arm::Replay, Scenario::Link, None);
    assert_eq!(
        again.setup_compiled, 0,
        "the second replay rebuilt something: {again:?}"
    );
    let ok = checked(bench, Scenario::Link, &[
        relinked.clone(),
        replayed.clone(),
        again,
    ]);
    assert!(ok.is_ok(), "{ok:?}");
    assert_eq!(replayed.mark, relinked.mark);
    assert_eq!(
        replayed.exe_bytes, relinked.exe_bytes,
        "the replay wrote another program"
    );
    let kept = call(
        bench,
        "default_replay",
        Arm::Replay,
        Scenario::Link,
        Some(Control::KeepOutput),
    );
    assert!(
        checked(bench, Scenario::Link, &[kept]).is_err(),
        "a replay over a kept output must be refused"
    );
}
