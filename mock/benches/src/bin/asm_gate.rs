//! D6 ASM-dispatch gate.
//!
//! Builds the `asm_gate_fixtures` cdylib in release, then objdumps the emitted
//! `Scheduler::run` and `Scheduler::run_fused` dispatch monos (the monomorphised
//! bodies the fixtures force to be codegen'd) and runs the five-check
//! disassembly checklist (`disasm_5check`) on each. Check 1 (zero indirect call)
//! is the hard gate: any FAIL exits non-zero, catching the moment a stored
//! function pointer survives into a dispatch body and devirtualisation is lost.
//! Checks 2 to 5 (indexed addressing, no inner stack spill, immediate morsel, no
//! helper call) are reported but do not gate, because they are legitimately
//! shape-dependent (the accumulator runs full-range with no baked morsel
//! immediate, column counts vary).
//!
//! Under fat LTO the dispatch lands in one of two places, so the gate checks
//! both. A single-caller dispatch mono (`run_fused`, the accumulator `run`)
//! inlines into its `#[no_mangle]` fixture wrapper, so the wrapper body carries
//! the dispatch and is a gate target. A multi-caller mono (the column-chain
//! `run`, called by both the plain and the dirty-gated fixtures) stays
//! out-of-line as its own symbol, which the wrapper reaches by a direct `bl`; a
//! regression inside that mono is invisible from the wrapper, so the out-of-line
//! `Scheduler::run` / `run_fused` monos (found by mangled-name pattern via `nm`)
//! are gate targets too. The union covers every emitted dispatch body. Direct
//! `bl`s to the build mono, allocator, and panic paths are expected and do not
//! trip check 1 (which counts only indirect `blr` / `callq *`).
//!
//! When no disassembler (`objdump` plus `nm`) is present, the gate skips cleanly
//! with a notice rather than passing silently or failing, so a toolchain without
//! a disassembler does not produce a spurious gate result.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use benches::disasm_5check;

fn main() -> ExitCode {
    // The fixtures crate sits beside this bench package, resolved from the
    // compile-time manifest dir so the gate runs from any cwd.
    let benches_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixtures_dir = benches_dir.join("asm_gate_fixtures");
    let manifest = fixtures_dir.join("Cargo.toml");

    if !tool_present("objdump") || !tool_present("nm") {
        eprintln!(
            "asm_gate: skipped, no disassembler (`objdump` and `nm` are both required and at \
             least one is absent). The gate reports skipped rather than passing silently."
        );
        return ExitCode::SUCCESS;
    }

    eprintln!("asm_gate: building fixtures (release) at {}", manifest.display());
    match Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(&manifest)
        .status()
    {
        Ok(s) if s.success() => {}
        Ok(s) => {
            eprintln!("asm_gate: fixtures build failed ({s})");
            return ExitCode::FAILURE;
        }
        Err(e) => {
            eprintln!("asm_gate: could not spawn cargo to build fixtures: {e}");
            return ExitCode::FAILURE;
        }
    }

    let dylib = fixtures_dir
        .join("target")
        .join("release")
        .join(format!(
            "{}asm_gate_fixtures{}",
            std::env::consts::DLL_PREFIX,
            std::env::consts::DLL_SUFFIX
        ));
    if !dylib.exists() {
        eprintln!("asm_gate: fixtures dylib not found at {}", dylib.display());
        return ExitCode::FAILURE;
    }

    let targets = match gate_targets(&dylib) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("asm_gate: could not list symbols: {e}");
            return ExitCode::FAILURE;
        }
    };
    let fixture_count = targets.iter().filter(|t| t.is_fixture).count();
    if fixture_count == 0 {
        eprintln!(
            "asm_gate: found no asm_gate_* fixture symbols in the dylib; the fixtures were not \
             emitted (build problem)"
        );
        return ExitCode::FAILURE;
    }
    eprintln!(
        "asm_gate: checking {} target(s) ({fixture_count} fixture wrapper(s) + {} out-of-line dispatch mono(s))",
        targets.len(),
        targets.len() - fixture_count
    );

    // The fixtures bake N = 256 as the morsel/record immediate where the shape
    // fixes it as a constant; check 4 looks for it and reports (not gates).
    let morsel_immediates = [256u64];

    let mut hard_fail = false;
    let mut checked = 0usize;
    for target in &targets {
        let disasm = match objdump_symbol(&dylib, &target.symbol) {
            Some(d) => d,
            None => {
                eprintln!("asm_gate: could not disassemble `{}`", target.label);
                continue;
            }
        };
        let report =
            disasm_5check::run_checks_on_disasm(&disasm, &dylib, &target.label, &morsel_immediates);
        print!("{report}");
        checked += 1;
        if let Some(c1) = report.outcomes.first() {
            if !c1.pass {
                eprintln!(
                    "asm_gate: HARD FAIL on `{}`: {} ({})",
                    target.label, c1.name, c1.detail
                );
                hard_fail = true;
            }
        }
    }

    if checked == 0 {
        eprintln!("asm_gate: no target was disassemblable; treating as failure");
        return ExitCode::FAILURE;
    }
    if hard_fail {
        eprintln!(
            "asm_gate: at least one dispatch body reintroduced an indirect call; \
             single-core dispatch devirtualisation regressed"
        );
        return ExitCode::FAILURE;
    }
    eprintln!("asm_gate: PASS (zero indirect call in every dispatch body)");
    ExitCode::SUCCESS
}

/// One gate target: its exact (nm) symbol name, a short label, and whether it is
/// a fixture wrapper (vs an out-of-line dispatch mono).
struct Target {
    symbol: String,
    label: String,
    is_fixture: bool,
}

/// The six fixture-wrapper shape names (without platform symbol prefix).
const FIXTURE_NAMES: [&str; 6] = [
    "asm_gate_column_chain",
    "asm_gate_fused_chain",
    "asm_gate_accumulator",
    "asm_gate_dirty_gated",
    "asm_gate_windowed_fibers",
    "asm_gate_resource_snapshot",
];

/// Collect the gate targets from `nm`: the six `#[no_mangle]` fixture wrappers
/// (which carry inlined dispatch for the single-caller shapes) plus any
/// out-of-line `Scheduler::run` / `run_fused` monos (the multi-caller shapes the
/// wrappers reach by direct `bl`). The union covers every emitted dispatch body
/// regardless of where LTO placed it.
fn gate_targets(dylib: &Path) -> Result<Vec<Target>, String> {
    let out = Command::new("nm")
        .arg(dylib)
        .output()
        .map_err(|e| format!("spawn nm: {e}"))?;
    if !out.status.success() {
        return Err(format!("nm exited non-zero on {}", dylib.display()));
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let mut targets = Vec::new();
    let mut run_idx = 0usize;
    let mut fused_idx = 0usize;
    for line in text.lines() {
        // nm line: `<addr> <type> <name>`; defined text symbols are `t`/`T`.
        let name = match line.split_whitespace().last() {
            Some(n) => n,
            None => continue,
        };
        // Fixture wrappers: `_asm_gate_<shape>` (macho) / `asm_gate_<shape>` (elf).
        if let Some(shape) = FIXTURE_NAMES
            .iter()
            .find(|s| name == **s || name == format!("_{s}"))
        {
            targets.push(Target {
                symbol: name.to_string(),
                label: format!("fixture `{shape}`"),
                is_fixture: true,
            });
            continue;
        }
        // Out-of-line dispatch monos.
        if !name.contains("12hilavitkutin9scheduler") || !name.contains("9Scheduler") {
            continue;
        }
        if name.contains("16SchedulerBuilder") {
            continue;
        }
        if name.contains("9run_fused") {
            fused_idx += 1;
            targets.push(Target {
                symbol: name.to_string(),
                label: format!("Scheduler::run_fused mono #{fused_idx}"),
                is_fixture: false,
            });
        } else if name.contains("3run") {
            run_idx += 1;
            targets.push(Target {
                symbol: name.to_string(),
                label: format!("Scheduler::run mono #{run_idx}"),
                is_fixture: false,
            });
        }
    }
    Ok(targets)
}

/// Disassemble one symbol by its exact name via the `=` form, returning the text
/// only when a real per-symbol instruction block came back.
fn objdump_symbol(dylib: &Path, symbol: &str) -> Option<String> {
    let out = Command::new("objdump")
        .arg("-d")
        .arg(format!("--disassemble-symbols={symbol}"))
        .arg(dylib)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).into_owned();
    if s.contains(&format!("<{symbol}>:")) {
        Some(s)
    } else {
        None
    }
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
