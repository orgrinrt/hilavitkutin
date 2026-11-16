//! The record of each call, and the checks a cell is held to.
//!
//! A call appends one line to its bench's record. The hooks library copies
//! the record into the cell's run directory as `calls.tsv` and refuses the
//! cell when any line shows a call that did not do what its scenario must:
//! a crate count other than the one the generator predicts, an output present
//! before the timed block, an output missing after it, a wait on a lock, a
//! failed cargo or linker, and for a link a missing linker mark or a check
//! line differing between calls. The expected count is worked out again here
//! from the fixture and the scenario, never read from the line.

use std::path::{Path, PathBuf};

use build_fixture::{Fixture, Site};

use crate::{Profile, Scenario};

/// The columns of the record, as written into a run directory.
pub const HEADER: &str = "bench\tarm\tscenario\tprofile\tn\tpid\tcall\tsetup_compiled\tcompiled\texpected\t\
                          absent_before\tpresent_after\texe_bytes\ttarget_bytes\tincremental_bytes\tmark\tcheck\texit\twaited\n";

/// One call's line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub bench:             String,
    pub arm:               String,
    pub scenario:          String,
    pub profile:           String,
    pub n:                 usize,
    pub pid:               u32,
    pub call:              u64,
    pub setup_compiled:    usize,
    pub compiled:          usize,
    pub expected:          usize,
    pub absent_before:     bool,
    pub present_after:     bool,
    pub exe_bytes:         u64,
    pub target_bytes:      u64,
    pub incremental_bytes: u64,
    pub mark:              String,
    pub check:             String,
    pub exit:              String,
    pub waited:            bool,
}

impl Line {
    pub fn to_tsv(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            self.bench,
            self.arm,
            self.scenario,
            self.profile,
            self.n,
            self.pid,
            self.call,
            self.setup_compiled,
            self.compiled,
            self.expected,
            self.absent_before,
            self.present_after,
            self.exe_bytes,
            self.target_bytes,
            self.incremental_bytes,
            clean(&self.mark),
            clean(&self.check),
            self.exit,
            self.waited
        )
    }

    pub fn from_tsv(line: &str) -> Option<Line> {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() != 19 {
            return None;
        }
        Some(Line {
            bench:             f[0].into(),
            arm:               f[1].into(),
            scenario:          f[2].into(),
            profile:           f[3].into(),
            n:                 f[4].parse().ok()?,
            pid:               f[5].parse().ok()?,
            call:              f[6].parse().ok()?,
            setup_compiled:    f[7].parse().ok()?,
            compiled:          f[8].parse().ok()?,
            expected:          f[9].parse().ok()?,
            absent_before:     f[10] == "true",
            present_after:     f[11] == "true",
            exe_bytes:         f[12].parse().ok()?,
            target_bytes:      f[13].parse().ok()?,
            incremental_bytes: f[14].parse().ok()?,
            mark:              f[15].into(),
            check:             f[16].into(),
            exit:              f[17].into(),
            waited:            f[18] == "true",
        })
    }
}

/// A field with no tab and no newline in it.
fn clean(s: &str) -> String {
    s.replace(['\t', '\n'], " ")
}

/// The record of `bench` under `root`.
pub fn path(root: &Path, bench: &str) -> PathBuf {
    root.join(bench).join("calls.tsv")
}

/// Append `line` to its bench's record.
pub fn append(root: &Path, line: &Line) -> Result<(), String> {
    use std::io::Write as _;
    let p = path(root, &line.bench);
    if let Some(dir) = p.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&p)
        .and_then(|mut f| f.write_all(line.to_tsv().as_bytes()))
        .map_err(|e| format!("{}: {e}", p.display()))
}

/// Every line of `bench`'s record, and the raw text.
pub fn read(root: &Path, bench: &str) -> (Vec<Line>, String) {
    let text = std::fs::read_to_string(path(root, bench)).unwrap_or_default();
    (text.lines().filter_map(Line::from_tsv).collect(), text)
}

/// Empty `bench`'s record.
pub fn reset(root: &Path, bench: &str) {
    let _ = std::fs::remove_file(path(root, bench));
}

/// The count of fixture crates a call of `scenario` at `n` crates must compile,
/// worked out from the generator: every crate from clean, an edit site's reach,
/// the facade alone for a relink, and none for a replayed link line.
pub fn expected(scenario: Scenario, replay: bool, n: usize) -> Option<usize> {
    let f = Fixture::at(n)?;
    Some(match scenario {
        Scenario::Clean => n,
        Scenario::Leaf => f.reach(Site::Leaf),
        Scenario::Mid => f.reach(Site::Mid),
        Scenario::Root => f.reach(Site::Root),
        Scenario::Link if replay => 0,
        Scenario::Link => 1,
    })
}

/// Hold every line of a cell to its scenario. `Ok` carries a one-line note,
/// `Err` every line that fails and why.
pub fn check(
    lines: &[Line],
    bench: &str,
    scenario: Scenario,
    profile: Profile,
    n: usize,
) -> Result<String, String> {
    if lines.is_empty() {
        return Err("no call was recorded".into());
    }
    let mut bad = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        let replay = l.arm.ends_with("_replay");
        let want = expected(scenario, replay, n);
        let mut why = Vec::new();
        if l.bench != bench
            || l.scenario != scenario.label()
            || l.profile != profile.label()
            || l.n != n
        {
            why.push(format!(
                "recorded as {} {} {} n={}, the cell is {bench} {} {} n={n}",
                l.bench,
                l.scenario,
                l.profile,
                l.n,
                scenario.label(),
                profile.label()
            ));
        }
        if want != Some(l.compiled) {
            why.push(format!(
                "compiled {} crates, the generator predicts {want:?}",
                l.compiled
            ));
        }
        if !l.absent_before {
            why.push(format!(
                "the timed block had nothing to do: {}",
                scenario.what_setup_undoes()
            ));
        }
        if !l.present_after {
            why.push("its output is missing after the timed block".into());
        }
        if l.exit != "0" {
            why.push(format!("exit {}", l.exit));
        }
        if l.waited {
            why.push("cargo waited on a lock".into());
        }
        if scenario == Scenario::Link
            && !l.mark.starts_with("Linker:")
            && !l.mark.starts_with("mold ")
        {
            why.push(format!("no linker mark in the output: `{}`", l.mark));
        }
        if !why.is_empty() {
            bad.push(format!("line {}: {}", i + 1, why.join(", ")));
        }
    }
    if scenario == Scenario::Link {
        let first = &lines[0].check;
        if first.is_empty() || lines.iter().any(|l| &l.check != first) {
            bad.push("the program's check line is empty or differs between calls".into());
        }
    }
    if bad.is_empty() {
        Ok(format!(
            "{} calls, each compiled the {} crates its scenario reaches",
            lines.len(),
            lines[0].expected
        ))
    } else {
        Err(bad.join("; "))
    }
}
