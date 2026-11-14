//! D6 ASM-dispatch gate.
//!
//! Stages the `asm_gate_fixtures` cdylib in release with `../stage.sh`, then
//! disassembles every dispatch body it emits and runs the five-check checklist
//! (`disasm_5check`) on each. Check 1 (zero indirect call through a stored
//! pointer) is the hard gate: any FAIL exits non-zero, catching the moment a
//! stored function pointer survives into a dispatch body and devirtualisation
//! is lost. Checks 2 to 5 (indexed addressing, no stack access, immediate
//! morsel, no helper call) are reported but do not gate, because they are
//! legitimately shape-dependent.
//!
//! Under fat LTO a dispatch lands in one of three places, and the gate checks
//! all of them. A single-caller dispatch inlines into its `#[no_mangle]`
//! fixture wrapper. A multi-caller `Scheduler::run` or `run_fused` mono stays
//! out of line as its own symbol, found by its mangled name. The parallel
//! path's per-core walk (`worker_main`, `run_core_phase`) inlines into the
//! closure `run_parallel` hands the pool, and std runs that closure inside
//! `__rust_begin_short_backtrace::<{closure}>`, which is the symbol the worker
//! body is found by.
//!
//! The gate is only evidence if it can fail, so the fixtures carry a control,
//! `asm_gate_control_stored_pointer`, which calls through a stored function
//! pointer. Check 1 has to fail on it, or the run fails as blind. A parallel
//! fixture with no worker body found fails the run too, since then the
//! parallel walk went unscanned.
//!
//! The report goes to `asm_gate_fixtures/asm_gate_report.md`, which is
//! committed. It names targets by label rather than by mangled name (the
//! mangling hashes differ between checkouts) and the dylib by file name, so it
//! regenerates byte-identical from the same tree and toolchain. `--check`
//! regenerates it and fails when it differs from the committed one.
//!
//! When no disassembler (`objdump` plus `nm`) is present, the gate skips with a
//! notice rather than passing silently or failing.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use benches::{disasm, disasm_5check};

/// The report's name inside `asm_gate_fixtures/`.
const REPORT: &str = "asm_gate_report.md";

/// The fixture wrapper names, without the platform's symbol prefix.
const FIXTURE_NAMES: [&str; 7] = [
    "asm_gate_column_chain",
    "asm_gate_fused_chain",
    "asm_gate_accumulator",
    "asm_gate_dirty_gated",
    "asm_gate_windowed_fibers",
    "asm_gate_resource_snapshot",
    "asm_gate_parallel_fiber_windows",
];

/// The control check 1 must fail on.
const CONTROL: &str = "asm_gate_control_stored_pointer";

/// The fixture that drives `run_parallel`, whose worker body must be found.
const PARALLEL: &str = "asm_gate_parallel_fiber_windows";

fn main() -> ExitCode {
    let check_only = std::env::args().skip(1).any(|a| a == "--check");
    match run(check_only) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("asm_gate: {e}");
            ExitCode::FAILURE
        },
    }
}

fn run(check_only: bool) -> Result<(), String> {
    let benches_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixtures_dir = benches_dir.join("asm_gate_fixtures");

    if !tool_present("objdump") || !tool_present("nm") {
        eprintln!(
            "asm_gate: skipped, no disassembler (`objdump` and `nm` are both required and at \
             least one is absent). The gate reports skipped rather than passing silently."
        );
        return Ok(());
    }

    let stage = std::env::temp_dir().join(format!("asm_gate_stage_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&stage);
    let staged = Command::new(benches_dir.join("stage.sh"))
        .arg(&fixtures_dir)
        .arg(&stage)
        .status()
        .map_err(|e| format!("could not run stage.sh: {e}"))?;
    if !staged.success() {
        return Err(format!("staging the fixtures failed ({staged})"));
    }
    let dylib = stage.join("release").join(format!(
        "{}asm_gate_fixtures{}",
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    ));
    let result = gate(&dylib, &fixtures_dir, check_only);
    let _ = std::fs::remove_dir_all(&stage);
    result
}

fn gate(dylib: &Path, fixtures_dir: &Path, check_only: bool) -> Result<(), String> {
    if !dylib.exists() {
        return Err(format!(
            "the staged fixtures have no dylib at {}",
            dylib.display()
        ));
    }
    let targets = gate_targets(dylib)?;
    if !targets.iter().any(|t| t.kind == Kind::Fixture) {
        return Err("found no asm_gate_* fixture symbols in the dylib (build problem)".to_string());
    }
    if !targets.iter().any(|t| t.kind == Kind::Control) {
        return Err(format!(
            "the control `{CONTROL}` is not in the dylib, so the gate cannot show it can fail"
        ));
    }
    let parallel = targets
        .iter()
        .any(|t| t.kind == Kind::Fixture && t.label.contains(PARALLEL));
    if parallel && !targets.iter().any(|t| t.kind == Kind::WorkerBody) {
        return Err(
            "no `run_parallel` worker body was found, so the parallel walk would go unscanned; \
             the gate's symbol pattern no longer matches the build"
                .to_string(),
        );
    }

    // The fixtures bake N = 256 where the shape fixes the record count as a
    // constant; check 4 looks for it and reports.
    let morsel_immediates = [256u64];
    let mut failures = Vec::new();
    let mut report = header()?;
    for target in &targets {
        let text = disasm::disassemble(dylib, &target.symbol)
            .ok_or_else(|| format!("could not disassemble {}", target.label))?;
        let checked =
            disasm_5check::run_checks_on_disasm(&text, dylib, &target.label, &morsel_immediates);
        let _ = write!(report, "{checked}");
        let c1 = checked
            .outcomes
            .first()
            .ok_or("the checklist returned no outcomes")?;
        match (target.kind, c1.pass) {
            (Kind::Control, true) => failures.push(format!(
                "check 1 passed the control `{CONTROL}`, which calls through a stored pointer, so it is blind"
            )),
            (Kind::Control, false) => {},
            (_, false) => failures.push(format!("HARD FAIL on {}: {}", target.label, c1.detail)),
            (_, true) => {},
        }
    }

    let path = fixtures_dir.join(REPORT);
    if check_only {
        let committed = std::fs::read_to_string(&path)
            .map_err(|e| format!("reading {}: {e}", path.display()))?;
        if committed != report {
            failures.push(format!(
                "{} differs from this build's report; run `cargo run --release --bin asm_gate` and commit it",
                path.display()
            ));
        }
    } else {
        std::fs::write(&path, &report).map_err(|e| format!("writing {}: {e}", path.display()))?;
        eprintln!("asm_gate: wrote {}", path.display());
    }

    if !failures.is_empty() {
        return Err(failures.join("\nasm_gate: "));
    }
    eprintln!(
        "asm_gate: PASS ({} dispatch bodies with zero indirect call through a stored pointer, control refused)",
        targets.iter().filter(|t| t.kind != Kind::Control).count()
    );
    Ok(())
}

/// The report header: what the targets are and what built them.
fn header() -> Result<String, String> {
    let rustc = first_line("rustc", &["-V"])?;
    let objdump = first_line("objdump", &["--version"])?;
    let mut h = String::new();
    let _ = writeln!(h, "# ASM-dispatch gate report");
    let _ = writeln!(h);
    let _ = writeln!(
        h,
        "Written by `mock/benches/src/bin/asm_gate.rs` from the staged `asm_gate_fixtures` release \
         build; `asm_gate --check` regenerates it and fails when this file differs. Check 1 gates \
         every target but the control, which it must fail; checks 2 to 5 are reported only."
    );
    let _ = writeln!(h);
    let _ = writeln!(h, "- target: `{}`", std::env::consts::ARCH);
    let _ = writeln!(h, "- toolchain: `{rustc}`");
    let _ = writeln!(h, "- disassembler: `{objdump}`");
    let _ = writeln!(h);
    Ok(h)
}

fn first_line(tool: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new(tool)
        .args(args)
        .current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/asm_gate_fixtures"))
        .output()
        .map_err(|e| format!("running {tool}: {e}"))?;
    Ok(String::from_utf8_lossy(&out.stdout)
        .lines()
        .next()
        .unwrap_or("")
        .trim()
        .to_string())
}

/// What a gate target is, in the order the report lists them.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Fixture,
    DispatchMono,
    WorkerBody,
    Control,
}

/// One gate target: its exact symbol, a label stable across checkouts, and its
/// kind.
struct Target {
    symbol: String,
    label:  String,
    kind:   Kind,
}

/// The gate targets, from `nm`: the fixture wrappers and the control, the
/// out-of-line `Scheduler::run` / `run_fused` monos, and the `run_parallel`
/// worker bodies.
///
/// `nm -n` lists by address in both its mangled and its demangled form, so the
/// two listings pair line by line. The out-of-line targets are numbered in the
/// order of their demangled names, which carry no crate hashes, so a label
/// names the same body in every checkout.
fn gate_targets(dylib: &Path) -> Result<Vec<Target>, String> {
    let mangled = nm(dylib, false)?;
    let demangled = nm(dylib, true)?;
    if mangled.len() != demangled.len() {
        return Err("nm listed a different number of symbols mangled and demangled".to_string());
    }
    let mut targets = Vec::new();
    let mut inner: Vec<(Kind, String, String)> = Vec::new();
    for (name, readable) in mangled.iter().zip(&demangled) {
        let bare = name
            .strip_prefix('_')
            .filter(|b| b.starts_with("asm_gate_"))
            .unwrap_or(name);
        if let Some(shape) = FIXTURE_NAMES.iter().find(|s| bare == **s) {
            targets.push(Target {
                symbol: name.clone(),
                label:  format!("fixture {shape}"),
                kind:   Kind::Fixture,
            });
        } else if bare == CONTROL {
            targets.push(Target {
                symbol: name.clone(),
                label:  format!("control {CONTROL}"),
                kind:   Kind::Control,
            });
        } else if let Some(kind) = classify(readable) {
            inner.push((kind, readable.clone(), name.clone()));
        }
    }
    inner.sort();
    let mut counts = [0usize; 2];
    for (kind, readable, symbol) in inner {
        let (slot, what) = match kind {
            Kind::WorkerBody => (1, "run_parallel worker body"),
            _ if readable.contains("::run_fused") => (0, "Scheduler::run_fused mono"),
            _ => (0, "Scheduler::run mono"),
        };
        counts[slot] += 1;
        targets.push(Target {
            symbol,
            label: format!("{what} #{}", counts[slot]),
            kind,
        });
    }
    targets.sort_by_key(|t| t.kind);
    Ok(targets)
}

/// Whether a demangled symbol is an out-of-line dispatch body, and which kind.
///
/// Read from the demangled name with its generic arguments collapsed, which
/// carries no crate hashes: `<Scheduler<..>>::run::<..>` reads `@::run::@`.
fn classify(readable: &str) -> Option<Kind> {
    const SCHEDULER: &str = "<hilavitkutin::scheduler::Scheduler<";
    // std runs a spawned closure inside this frame, instantiated over it. The
    // worker body is the instantiation over `run_parallel`'s own closure; the
    // ones over std's spawn closures start `std::thread`.
    const FRAME: &str = "std::sys::backtrace::__rust_begin_short_backtrace::<";
    if let Some(arg) = readable.strip_prefix(FRAME) {
        let ours = arg.starts_with(SCHEDULER)
            && arg.contains(">::run_parallel")
            && arg.contains("{closure");
        return ours.then_some(Kind::WorkerBody);
    }
    if !readable.starts_with(SCHEDULER) {
        return None;
    }
    matches!(
        collapse(readable).as_str(),
        "@::run::@" | "@::run" | "@::run_fused::@" | "@::run_fused"
    )
    .then_some(Kind::DispatchMono)
}

/// `s` with every generic argument list, nested or not, replaced by `@`.
fn collapse(s: &str) -> String {
    let mut out = String::new();
    let mut depth = 0usize;
    for c in s.chars() {
        match c {
            '<' => {
                if depth == 0 {
                    out.push('@');
                }
                depth += 1;
            },
            '>' => depth = depth.saturating_sub(1),
            _ if depth == 0 => out.push(c),
            _ => {},
        }
    }
    out
}

/// The defined text symbols of `dylib` in address order, demangled or not.
fn nm(dylib: &Path, demangle: bool) -> Result<Vec<String>, String> {
    let mut cmd = Command::new("nm");
    cmd.arg("-n").arg("--defined-only");
    if demangle {
        cmd.arg("-C");
    }
    let out = cmd
        .arg(dylib)
        .output()
        .map_err(|e| format!("spawn nm: {e}"))?;
    if !out.status.success() {
        return Err(format!("nm exited non-zero on {}", dylib.display()));
    }
    Ok(String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|l| {
            let mut f = l.splitn(3, ' ');
            let (_, kind, name) = (f.next()?, f.next()?, f.next()?);
            matches!(kind, "t" | "T").then(|| name.to_string())
        })
        .collect())
}

fn tool_present(tool: &str) -> bool {
    Command::new(tool)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::{Kind, classify, collapse};

    // Names as `nm -C` prints them for the gate build of 13 Nov, cut short
    // inside the generic arguments where those do not matter.
    const SCHED: &str =
        "<hilavitkutin::scheduler::Scheduler<hilavitkutin_api::run_cfg::DefaultRunCfg, W<a::S1>>>";

    #[test]
    fn generic_arguments_collapse_at_every_depth() {
        assert_eq!(collapse(&format!("{SCHED}::run::<P<Q>, R>")), "@::run::@");
        assert_eq!(collapse("a::b"), "a::b");
    }

    #[test]
    fn the_worker_body_is_found_and_std_s_spawn_frame_is_not() {
        let worker = format!(
            "std::sys::backtrace::__rust_begin_short_backtrace::<{SCHED}::run_parallel<E>::{{closure#0}}, ()>"
        );
        let spawn = format!(
            "std::sys::backtrace::__rust_begin_short_backtrace::<std::thread::lifecycle::spawn_unchecked<{SCHED}::run_parallel<E>::{{closure#0}}, ()>::{{closure#1}}::{{closure#0}}, ()>"
        );
        assert_eq!(classify(&worker), Some(Kind::WorkerBody));
        assert_eq!(classify(&spawn), None);
    }

    #[test]
    fn a_run_mono_is_found_and_its_neighbours_are_not() {
        assert_eq!(
            classify(&format!("{SCHED}::run::<E>")),
            Some(Kind::DispatchMono)
        );
        assert_eq!(
            classify(&format!("{SCHED}::run_fused::<E>")),
            Some(Kind::DispatchMono)
        );
        assert_eq!(
            classify("<hilavitkutin::scheduler::builder::SchedulerBuilder<A>>::build::<B>"),
            None
        );
        assert_eq!(classify(&format!("core::ptr::drop_glue::<{SCHED}>")), None);
        assert_eq!(classify(&format!("{SCHED}::run_parallel::<E>")), None);
    }
}
