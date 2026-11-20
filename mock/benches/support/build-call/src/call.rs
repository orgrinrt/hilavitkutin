//! One call, from its untimed setup through its timed command to its record.
//!
//! The setup is per scenario: an empty target directory for `clean`; a
//! catch-up build and a new edit slot content for the edit scenarios; a
//! catch-up build and the facade's executable removed for a relink; the link
//! line rustc printed, kept, and its output removed for a replay. Before any
//! of it, the cell's disk is held to its budget ([`crate::space`]).

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};

use build_fixture::{Fixture, Site};
use hilavitkutin_build::Set;

use crate::linkline::{self, LinkLine};
use crate::record::{self, Line};
use crate::{Arm, Control, Dirs, Kind, Profile, Quantity, Scenario, cargo, linker, root, space};

/// One call, between its setup and its record.
pub struct Call {
    bench:          &'static str,
    arm_name:       &'static str,
    arm:            Arm,
    scenario:       Scenario,
    profile:        Profile,
    n:              usize,
    dirs:           Dirs,
    setup_compiled: usize,
    absent_before:  bool,
    exe:            Option<PathBuf>,
    command:        Option<Command>,
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
        if let Some(why) = arm.refusal() {
            panic!("build-call: arm `{arm_name}` of `{bench}`: {why}");
        }
        let fixture = Fixture::at(n).unwrap_or_else(|| panic!("no fixture of {n} crates"));
        let dirs = Dirs::of(bench, arm_name, n);
        prune(&root().join(bench).join(arm_name), &dirs.dir());
        dirs.make().unwrap_or_else(|e| panic!("{e}"));
        let free = match control {
            Some(Control::NoSpace) => 0,
            _ => space::free_bytes(&dirs.dir()).unwrap_or_else(|e| panic!("{e}")),
        };
        let held = cargo::bytes_under(&dirs.target());
        if let Some(why) = space::refusal(free, held, space::budget(n, arm.incremental())) {
            panic!("build-call: {}: {why}", dirs.dir().display());
        }
        let mut call = Call {
            bench,
            arm_name,
            arm,
            scenario,
            profile,
            n,
            dirs,
            setup_compiled: 0,
            absent_before: false,
            exe: None,
            command: None,
        };
        match (scenario.site(), call.arm.kind()) {
            (_, Kind::Replay) => call.ready_replay(&fixture, control),
            (_, Kind::Relink) => call.ready_relink(&fixture, control),
            (Some(site), Kind::Build) => call.ready_edit(&fixture, site, control),
            (None, Kind::Build) => call.ready_clean(&fixture, control),
        }
        call
    }

    /// A cargo command for this cell, its profile flag added.
    fn cargo(&self, args: &[&str]) -> Command {
        let mut c = cargo::command(&self.dirs, &self.arm, self.profile, args);
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
        Call::write(fixture, &self.dirs.ws(), None);
        let target = self.dirs.target();
        if control != Some(Control::SkipClean) && target.exists() {
            std::fs::remove_dir_all(&target)
                .unwrap_or_else(|e| panic!("{}: {e}", target.display()));
        }
        self.absent_before = !target.exists();
        self.command = Some(self.cargo(&["build", "--workspace", "--message-format=json"]));
    }

    fn ready_edit(&mut self, fixture: &Fixture, site: Site, control: Option<Control>) {
        let role = fixture.site_role(site);
        let counter = self.dirs.dir().join("edits");
        let current: u64 = std::fs::read_to_string(&counter)
            .ok()
            .and_then(|t| t.trim().parse().ok())
            .unwrap_or(0);
        Call::write(fixture, &self.dirs.ws(), Some((role, current)));
        let caught_up = Call::untimed(
            self.cargo(&["build", "--workspace", "--message-format=json"]),
            "the catch-up build",
        );
        self.setup_compiled = cargo::compiled(&String::from_utf8_lossy(&caught_up.stdout)).len();
        let slot = self.dirs.ws().join(Fixture::edit_path(role));
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
        self.command = Some(self.cargo(&["build", "--workspace", "--message-format=json"]));
    }

    fn ready_relink(&mut self, fixture: &Fixture, control: Option<Control>) {
        Call::write(fixture, &self.dirs.ws(), None);
        let args = ["build", "-p", "fx_facade", "--message-format=json"];
        let caught_up = Call::untimed(self.cargo(&args), "the catch-up build");
        let said = String::from_utf8_lossy(&caught_up.stdout);
        self.setup_compiled = cargo::compiled(&said).len();
        let exe = PathBuf::from(
            cargo::executable(&said).expect("the facade's executable in cargo's messages"),
        );
        if control != Some(Control::KeepOutput) {
            remove_facade(
                &exe,
                &self.dirs.target().join(self.profile.dir()).join("deps"),
            );
        }
        self.absent_before = !exe.exists();
        self.exe = Some(exe);
        self.command = Some(self.cargo(&args));
    }

    fn ready_replay(&mut self, fixture: &Fixture, control: Option<Control>) {
        Call::write(fixture, &self.dirs.ws(), None);
        let args = ["rustc", "-p", "fx_facade", "--bin", "fx_facade"];
        let mut c = self.cargo(&args);
        c.args(["--", "--print", "link-args", "-C", "save-temps"]);
        let caught_up = Call::untimed(c, "the build that records the link line");
        let said = String::from_utf8_lossy(&caught_up.stderr);
        self.setup_compiled = said
            .lines()
            .filter(|l| l.trim_start().starts_with("Compiling fx_"))
            .count();
        let kept = self.dirs.link_line(self.profile);
        if let Some(line) = linkline::find(&String::from_utf8_lossy(&caught_up.stdout)) {
            std::fs::write(&kept, line.to_text())
                .unwrap_or_else(|e| panic!("{}: {e}", kept.display()));
        }
        let line = std::fs::read_to_string(&kept)
            .ok()
            .and_then(|t| LinkLine::from_text(&t))
            .unwrap_or_else(|| panic!("{}: no recorded link line", kept.display()));
        let exe = line
            .output_under(&self.dirs.target().join(self.profile.dir()))
            .unwrap_or_else(|e| panic!("build-call: {}: {e}", kept.display()));
        if control != Some(Control::KeepOutput) && exe.exists() {
            std::fs::remove_file(&exe).unwrap_or_else(|e| panic!("{}: {e}", exe.display()));
        }
        self.absent_before = !exe.exists();
        self.exe = Some(exe);
        let mut replay = line.command();
        cargo::keep_only_allowed(&mut replay);
        for (k, v) in &line.set {
            replay.env(k, v);
        }
        self.command = Some(replay);
    }

    /// The timed command, started with its output piped.
    pub fn spawn(&mut self) -> std::io::Result<Child> {
        self.command
            .as_mut()
            .expect("every setup leaves the timed command")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
    }

    /// The timed block: one build, or one run of the link line.
    pub fn run(&mut self) -> Output {
        self.spawn()
            .and_then(Child::wait_with_output)
            .expect("the timed command starts")
    }

    /// The untimed record: one line appended to the bench's record. Answers
    /// with the crates compiled for a build arm, and the executable's size in
    /// bytes for a link arm. A replayed link line runs no cargo, so its line
    /// carries no crate count.
    pub fn record(self, out: &Output) -> u64 {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let kind = self.arm.kind();
        let compiled = match kind {
            Kind::Replay => None,
            Kind::Build | Kind::Relink => Some(cargo::compiled(&stdout).len()),
        };
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
            (Some(e), Scenario::Link) if present_after => linker::proof(e),
            _ => (String::new(), String::new()),
        };
        let target = self.dirs.target();
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
            expected: match kind {
                Kind::Replay => None,
                Kind::Build | Kind::Relink => record::expected(self.scenario, self.n),
            },
            absent_before: self.absent_before,
            present_after,
            exe_bytes,
            target_bytes: cargo::bytes_under(&target),
            incremental_bytes: cargo::bytes_under(
                &target.join(self.profile.dir()).join("incremental"),
            ),
            mark,
            check,
            exit: out
                .status
                .code()
                .map_or_else(|| "signal".to_string(), |c| c.to_string()),
            waited: cargo::waited(&String::from_utf8_lossy(&out.stderr)),
            linker: self.arm.linker().name().to_string(),
            quantity: kind.label().to_string(),
        };
        if !out.status.success() {
            eprintln!(
                "build-call: the timed command failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
        record::append(&root(), &line).unwrap_or_else(|e| panic!("{e}"));
        match kind {
            Kind::Build => compiled.map_or(0, |c| c as u64),
            Kind::Relink | Kind::Replay => exe_bytes,
        }
    }
}

/// One call of an arm, the whole body of every arm cdylib: [`Call::ready`]
/// untimed, [`Call::run`] timed, [`Call::record`] untimed into `output`.
///
/// The arm is its two type parameters: `S`, the set of hilavitkutin-build's
/// pragmas it builds with, and `Q`, the quantity it measures. `arm_name` is
/// the arm's directory and its label in the record.
pub fn bench_call<S: Set, Q: Quantity>(
    bench: &'static str,
    arm_name: &'static str,
    input: &[u8; 8],
    n: usize,
    output: &mut u64,
) -> mockspace_bench_core::FfiBenchCall {
    use mockspace_bench_core::timed;
    timed! {
        setup {
            let mut call = Call::ready(bench, arm_name, Arm::of::<S, Q>(), input, n);
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
            // FIXME(the_build_and_link_bench_assumes_an_elf_unix_host_and_fails): the facade's copy in `deps` is found as
            // the one file with no extension, which holds where an executable
            // has none; a `.exe` beside a `.pdb` is not found, so on such a
            // target the relink cell fails instead of being left out.
            if name.starts_with("fx_facade-") && !name.contains('.') {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
}
