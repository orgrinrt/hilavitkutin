//! What one call of a build or link bench does.
//!
//! A call has three parts, the harness's `timed!` blocks: [`Call::ready`],
//! untimed, writes the fixture and puts the cell's build into the state its
//! scenario starts from; [`Call::run`], timed, is one cargo build or one run of
//! a recorded link line; [`Call::record`], untimed, reads what cargo and the
//! linker said, appends one line to the bench's record and answers with the
//! call's result. The scenario and the profile come in the input bytes the
//! routine of the cell builds ([`routine`]), the arm names how it builds.
//!
//! Every cell builds in a directory of its own under the build directory the
//! run inherits (`CARGO_TARGET_DIR`, or the system's temporary directory where
//! none is set): `build-link/<bench>/<arm>/n<point>/`, holding the fixture
//! tree `ws/`, the target directory `target/` and the temporary directory
//! rustc writes into, `tmp/`. A call at one point removes its arm's
//! directories of every other point, so one point per arm is on the disk.

pub mod cargo;
pub mod elf;
pub mod hooks;
pub mod linkline;
pub mod record;
pub mod routine;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use build_fixture::{Fixture, Site};
use linkline::LinkLine;
use record::Line;

/// What a call does to the fixture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scenario {
    /// Every crate built into an empty target directory.
    Clean,
    /// One leaf implementation crate edited, then the workspace built.
    Leaf,
    /// One mid contract crate edited, then the workspace built.
    Mid,
    /// One root contract crate edited, then the workspace built.
    Root,
    /// Every object fresh and the final executable removed, then linked again.
    Link,
}

impl Scenario {
    pub const ALL: [Scenario; 5] =
        [Scenario::Clean, Scenario::Leaf, Scenario::Mid, Scenario::Root, Scenario::Link];

    pub const fn code(self) -> u8 {
        self as u8
    }

    pub fn from_code(c: u8) -> Option<Scenario> {
        Scenario::ALL.get(usize::from(c)).copied()
    }

    pub const fn label(self) -> &'static str {
        match self {
            Scenario::Clean => "clean",
            Scenario::Leaf => "leaf",
            Scenario::Mid => "mid",
            Scenario::Root => "root",
            Scenario::Link => "link",
        }
    }

    /// The edit site, for an edit scenario.
    pub const fn site(self) -> Option<Site> {
        match self {
            Scenario::Leaf => Some(Site::Leaf),
            Scenario::Mid => Some(Site::Mid),
            Scenario::Root => Some(Site::Root),
            _ => None,
        }
    }

    /// What the untimed setup has to have done for the timed block to have work.
    pub const fn what_setup_undoes(self) -> &'static str {
        match self {
            Scenario::Clean => "the target directory was not empty",
            Scenario::Link => "the final executable was still there",
            _ => "the edit changed no content",
        }
    }
}

/// The fixture's own profile a call builds in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    Dev,
    Release,
}

impl Profile {
    pub const fn code(self) -> u8 {
        self as u8
    }

    pub fn from_code(c: u8) -> Option<Profile> {
        match c {
            0 => Some(Profile::Dev),
            1 => Some(Profile::Release),
            _ => None,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Profile::Dev => "dev",
            Profile::Release => "release",
        }
    }

    /// The directory cargo writes this profile's outputs to.
    pub const fn dir(self) -> &'static str {
        match self {
            Profile::Dev => "debug",
            Profile::Release => "release",
        }
    }
}

/// How an arm builds or links.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    /// A whole-workspace build with the toolchain's default backend and linker.
    Build {
        incremental: bool,
    },
    /// Quantity A: the facade built again by cargo, every rlib fresh.
    Relink,
    /// Quantity B: the facade's recorded link line run again as a process.
    Replay,
}

/// A deliberately broken setup, which the checks must refuse. Only the
/// controls in this crate's tests use it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Control {
    /// A clean scenario that removes nothing.
    SkipClean,
    /// An edit that moves the slot's time and leaves its content.
    TimeOnly,
    /// A link that leaves the final executable in place.
    KeepOutput,
}

/// Where a bench's state lives: `build-link/` under the inherited build directory.
pub fn root() -> PathBuf {
    std::env::var_os("CARGO_TARGET_DIR")
        .map_or_else(std::env::temp_dir, PathBuf::from)
        .join("build-link")
}

/// One call, between its setup and its record.
pub struct Call {
    bench:          &'static str,
    arm_name:       &'static str,
    arm:            Arm,
    scenario:       Scenario,
    profile:        Profile,
    n:              usize,
    dir:            PathBuf,
    setup_compiled: usize,
    absent_before:  bool,
    exe:            Option<PathBuf>,
    command:        Command,
}

impl Call {
    /// The untimed setup of a call of `arm` at `n` crates, the scenario and
    /// profile read from `input`. Panics where the setup cannot be done, which
    /// fails the worker and so the cell.
    pub fn ready(
        bench: &'static str,
        arm_name: &'static str,
        arm: Arm,
        input: &[u8],
        n: usize,
    ) -> Call {
        Call::ready_with(bench, arm_name, arm, input, n, None)
    }

    /// [`Call::ready`] with a broken setup, for the controls.
    pub fn ready_with(
        bench: &'static str,
        arm_name: &'static str,
        arm: Arm,
        input: &[u8],
        n: usize,
        control: Option<Control>,
    ) -> Call {
        let scenario = input
            .first()
            .and_then(|c| Scenario::from_code(*c))
            .expect("a scenario code");
        let profile = input
            .get(1)
            .and_then(|c| Profile::from_code(*c))
            .expect("a profile code");
        let fixture = Fixture::at(n).unwrap_or_else(|| panic!("no fixture of {n} crates"));
        let arm_root = root().join(bench).join(arm_name);
        let dir = arm_root.join(format!("n{n}"));
        prune(&arm_root, &dir);
        for d in ["ws", "tmp"] {
            std::fs::create_dir_all(dir.join(d))
                .unwrap_or_else(|e| panic!("{}: {e}", dir.display()));
        }
        let mut call = Call {
            bench,
            arm_name,
            arm,
            scenario,
            profile,
            n,
            setup_compiled: 0,
            absent_before: false,
            exe: None,
            command: Command::new("true"),
            dir,
        };
        match (scenario.site(), arm) {
            (_, Arm::Replay) => call.ready_replay(&fixture, control),
            (_, Arm::Relink) => call.ready_relink(&fixture, control),
            (
                Some(site),
                Arm::Build {
                    ..
                },
            ) => call.ready_edit(&fixture, site, control),
            (
                None,
                Arm::Build {
                    ..
                },
            ) => call.ready_clean(&fixture, control),
        }
        call
    }

    fn ws(&self) -> PathBuf {
        self.dir.join("ws")
    }

    fn target(&self) -> PathBuf {
        self.dir.join("target")
    }

    fn incremental(&self) -> bool {
        matches!(self.arm, Arm::Build {
            incremental: true,
        })
    }

    /// A cargo command for this cell, its profile flag added.
    fn cargo(&self, args: &[&str]) -> Command {
        let mut c = cargo::command(
            &self.ws(),
            &self.target(),
            &self.dir.join("tmp"),
            self.incremental(),
            args,
        );
        if self.profile == Profile::Release {
            c.arg("--release");
        }
        c
    }

    /// Run `c` untimed, panicking on a failure, and answer with its output.
    fn untimed(mut c: Command, what: &str) -> Output {
        let out = c.output().unwrap_or_else(|e| panic!("{what}: {e}"));
        assert!(
            out.status.success(),
            "{what} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        out
    }

    fn write(fixture: &Fixture, ws: &Path, edited: Option<(build_fixture::graph::Role, u64)>) {
        fixture.write(ws, edited).unwrap_or_else(|e| panic!("{e}"));
    }

    fn ready_clean(&mut self, fixture: &Fixture, control: Option<Control>) {
        Call::write(fixture, &self.ws(), None);
        let target = self.target();
        if control != Some(Control::SkipClean) && target.exists() {
            std::fs::remove_dir_all(&target)
                .unwrap_or_else(|e| panic!("{}: {e}", target.display()));
        }
        self.absent_before = !target.exists();
        self.command = self.cargo(&["build", "--workspace", "--message-format=json"]);
    }

    fn ready_edit(&mut self, fixture: &Fixture, site: Site, control: Option<Control>) {
        let role = fixture.site_role(site);
        let counter = self.dir.join("edits");
        let current: u64 = std::fs::read_to_string(&counter)
            .ok()
            .and_then(|t| t.trim().parse().ok())
            .unwrap_or(0);
        Call::write(fixture, &self.ws(), Some((role, current)));
        let caught_up = Call::untimed(
            self.cargo(&["build", "--workspace", "--message-format=json"]),
            "the catch-up build",
        );
        self.setup_compiled = cargo::compiled(&String::from_utf8_lossy(&caught_up.stdout)).len();
        let slot = self.ws().join(Fixture::edit_path(role));
        if control == Some(Control::TimeOnly) {
            let now = std::time::SystemTime::now() + std::time::Duration::from_secs(1);
            std::fs::File::options()
                .write(true)
                .open(&slot)
                .and_then(|f| f.set_modified(now))
                .unwrap_or_else(|e| panic!("{}: {e}", slot.display()));
            self.absent_before = false;
        } else {
            let next = current + 1;
            std::fs::write(&counter, next.to_string())
                .unwrap_or_else(|e| panic!("{}: {e}", counter.display()));
            std::fs::write(&slot, build_fixture::text::edit_slot(next))
                .unwrap_or_else(|e| panic!("{}: {e}", slot.display()));
            self.absent_before = true;
        }
        self.command = self.cargo(&["build", "--workspace", "--message-format=json"]);
    }

    fn ready_relink(&mut self, fixture: &Fixture, control: Option<Control>) {
        Call::write(fixture, &self.ws(), None);
        let args = ["build", "-p", "fx_facade", "--message-format=json"];
        let caught_up = Call::untimed(self.cargo(&args), "the catch-up build");
        let said = String::from_utf8_lossy(&caught_up.stdout);
        self.setup_compiled = cargo::compiled(&said).len();
        let exe = PathBuf::from(
            cargo::executable(&said).expect("the facade's executable in cargo's messages"),
        );
        if control != Some(Control::KeepOutput) {
            remove_facade(&exe, &self.target().join(self.profile.dir()).join("deps"));
        }
        self.absent_before = !exe.exists();
        self.exe = Some(exe);
        self.command = self.cargo(&args);
    }

    fn ready_replay(&mut self, fixture: &Fixture, control: Option<Control>) {
        Call::write(fixture, &self.ws(), None);
        let args = ["rustc", "-p", "fx_facade", "--bin", "fx_facade"];
        let mut c = self.cargo(&args);
        c.args(["--", "--print", "link-args", "-C", "save-temps"]);
        let caught_up = Call::untimed(c, "the build that records the link line");
        let said = String::from_utf8_lossy(&caught_up.stderr);
        self.setup_compiled = said
            .lines()
            .filter(|l| l.trim_start().starts_with("Compiling fx_"))
            .count();
        let kept = self.dir.join("link-line.txt");
        if let Some(line) = linkline::find(&String::from_utf8_lossy(&caught_up.stdout)) {
            std::fs::write(&kept, line.to_text())
                .unwrap_or_else(|e| panic!("{}: {e}", kept.display()));
        }
        let line = std::fs::read_to_string(&kept)
            .ok()
            .and_then(|t| LinkLine::from_text(&t))
            .unwrap_or_else(|| panic!("{}: no recorded link line", kept.display()));
        let exe = PathBuf::from(line.output().expect("`-o` in the link line"));
        if control != Some(Control::KeepOutput) && exe.exists() {
            std::fs::remove_file(&exe).unwrap_or_else(|e| panic!("{}: {e}", exe.display()));
        }
        self.absent_before = !exe.exists();
        self.exe = Some(exe);
        self.command = line.command();
    }

    /// The timed block: one build, or one run of the link line.
    pub fn run(&mut self) -> Output {
        self.command.output().expect("the timed command starts")
    }

    /// The untimed record: one line appended to the bench's record. Answers
    /// with the crates compiled for a build arm, and the executable's size in
    /// bytes for a link arm.
    pub fn record(self, out: &Output) -> u64 {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let compiled = if self.arm == Arm::Replay { 0 } else { cargo::compiled(&stdout).len() };
        let exe = self
            .exe
            .clone()
            .or_else(|| cargo::executable(&stdout).map(PathBuf::from));
        let present_after = exe.as_ref().is_some_and(|e| e.is_file());
        let exe_bytes = exe
            .as_ref()
            .and_then(|e| std::fs::metadata(e).ok())
            .map_or(0, |m| m.len());
        let (mark, check) = match (&exe, self.scenario) {
            (Some(e), Scenario::Link) if present_after => link_proof(e),
            _ => (String::new(), String::new()),
        };
        let line = Line {
            bench: self.bench.to_string(),
            arm: self.arm_name.to_string(),
            scenario: self.scenario.label().to_string(),
            profile: self.profile.label().to_string(),
            n: self.n,
            pid: std::process::id(),
            call: next_call(),
            setup_compiled: self.setup_compiled,
            compiled,
            expected: record::expected(self.scenario, self.arm == Arm::Replay, self.n).unwrap_or(0),
            absent_before: self.absent_before,
            present_after,
            exe_bytes,
            target_bytes: cargo::bytes_under(&self.target()),
            incremental_bytes: cargo::bytes_under(
                &self.target().join(self.profile.dir()).join("incremental"),
            ),
            mark,
            check,
            exit: out
                .status
                .code()
                .map_or_else(|| "signal".to_string(), |c| c.to_string()),
            waited: cargo::waited(&String::from_utf8_lossy(&out.stderr)),
        };
        if !out.status.success() {
            eprintln!(
                "build-call: the timed command failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
        record::append(&root(), &line).unwrap_or_else(|e| panic!("{e}"));
        match self.arm {
            Arm::Build {
                ..
            } => compiled as u64,
            Arm::Relink | Arm::Replay => exe_bytes,
        }
    }
}

/// One call of an arm, the whole body of every arm cdylib: [`Call::ready`]
/// untimed, [`Call::run`] timed, [`Call::record`] untimed into `output`.
pub fn bench_call(
    bench: &'static str,
    arm_name: &'static str,
    arm: Arm,
    input: &[u8; 8],
    n: usize,
    output: &mut u64,
) -> mockspace_bench_core::FfiBenchCall {
    use mockspace_bench_core::timed;
    timed! {
        setup {
            let mut call = Call::ready(bench, arm_name, arm, input, n);
        }
        run {
            let said = call.run();
        }
        *output = call.record(&said);
    }
}

/// The calls this process has recorded.
fn next_call() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CALLS: AtomicU64 = AtomicU64::new(0);
    CALLS.fetch_add(1, Ordering::Relaxed) + 1
}

/// Remove every directory of `arm_root` but `keep`.
fn prune(arm_root: &Path, keep: &Path) {
    let Ok(entries) = std::fs::read_dir(arm_root) else {
        return;
    };
    for e in entries.filter_map(Result::ok) {
        if e.path() != keep && e.path().is_dir() {
            let _ = std::fs::remove_dir_all(e.path());
        }
    }
}

/// Remove the facade's executable and the file in `deps` it is a copy of,
/// so cargo has to link it again.
fn remove_facade(exe: &Path, deps: &Path) {
    let _ = std::fs::remove_file(exe);
    if let Ok(entries) = std::fs::read_dir(deps) {
        for e in entries.filter_map(Result::ok) {
            let name = e.file_name().to_string_lossy().into_owned();
            if name.starts_with("fx_facade-") && !name.contains('.') {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
}

/// The linker's mark in `exe` and the check line the program prints.
fn link_proof(exe: &Path) -> (String, String) {
    let mark = elf::comment(exe)
        .map(|c| elf::linker_mark(&c).unwrap_or_else(|| format!("none among {c:?}")))
        .unwrap_or_else(|e| e);
    let check = Command::new(exe)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|e| format!("did not run: {e}"));
    (mark, check)
}
