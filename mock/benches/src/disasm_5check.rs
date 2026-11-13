//! 5-check disasm assertion module for Topic 4 axis A dispatch shapes.
//!
//! Implements CHANGE 7 of round 202605101036. After each `dispatch_*`
//! bench run, this module reads the emitted asm out of each variant's
//! release dylib via `objdump -d` (POSIX standard, available on Linux
//! and on macOS via the Xcode CLT), falls back to `otool -tvV` if
//! objdump is unavailable, runs five text-pattern checks against the
//! `bench_entry` symbol body, and writes a consolidated report.
//!
//! On any FAIL in a `dispatch_static_*` bench (the bench whose
//! invariant IS LLVM-transparent dispatch), the bench binary exits
//! non-zero so `cargo mock bench` surfaces the regression. The
//! `dispatch_dynamic_*` bench is the counter-example; FAILs there
//! are expected and recorded for the audit trail without driving
//! the exit code.
//!
//! Canonical statement of intent + map from each check to its
//! sketch-heritage validation: see
//! `mock/benches/codegen_dispatch_axis_a_assertions.md`.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fmt, fs};

use crate::disasm::{self, Insn};

/// One check's outcome.
#[derive(Debug, Clone)]
pub struct CheckOutcome {
    pub name:   &'static str,
    pub pass:   bool,
    pub detail: String,
}

/// Full 5-check report for one variant's symbol.
#[derive(Debug, Clone)]
pub struct CheckReport {
    pub variant:  String,
    pub dylib:    PathBuf,
    pub symbol:   String,
    pub outcomes: Vec<CheckOutcome>,
}

impl CheckReport {
    pub fn all_pass(&self) -> bool {
        self.outcomes.iter().all(|o| o.pass)
    }
}

/// Consolidated report across all variants of one bench / N pair.
#[derive(Debug, Clone)]
pub struct BenchReport {
    pub bench:   String,
    pub n:       usize,
    pub reports: Vec<CheckReport>,
}

impl BenchReport {
    pub fn any_fail(&self) -> bool {
        self.reports.iter().any(|r| !r.all_pass())
    }
}

/// Run all 5 checks against the named symbol in the given dylib.
///
/// `morsel_immediates` is the list of decimal morsel/Cfg constants
/// that codegen should have baked as immediates. The bench binary
/// passes values that include the per-call N (records per bench
/// call) plus any other Cfg constants expected to appear.
pub fn run_checks(
    dylib_path: &Path,
    symbol: &str,
    morsel_immediates: &[u64],
) -> Result<CheckReport, String> {
    let disasm = read_disasm(dylib_path, symbol)?;
    Ok(run_checks_on_disasm(
        &disasm,
        dylib_path,
        symbol,
        morsel_immediates,
    ))
}

/// Run the five checks against already-read disassembly text.
///
/// The gate uses this directly: it objdumps the dispatch monos by their exact
/// (nm-discovered) mangled symbol names, so it owns the read and reuses only the
/// platform-aware check logic rather than `read_disasm`'s symbol-name guessing.
pub fn run_checks_on_disasm(
    disasm: &str,
    dylib_path: &Path,
    symbol: &str,
    morsel_immediates: &[u64],
) -> CheckReport {
    let outcomes = vec![
        check_no_indirect_call(disasm),
        check_indexed_addressing(disasm),
        check_no_stack_in_inner_loop(disasm),
        check_immediate_morsel_size(disasm, morsel_immediates),
        check_no_bl_to_helpers(disasm),
    ];
    CheckReport {
        variant: dylib_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string(),
        dylib: dylib_path.to_path_buf(),
        symbol: symbol.to_string(),
        outcomes,
    }
}

fn read_disasm(dylib: &Path, symbol: &str) -> Result<String, String> {
    if let Some(text) = disasm::disassemble(dylib, symbol) {
        return Ok(text);
    }
    // Fall back to otool on macOS. Dumps the whole text segment;
    // checks scan for `bench_entry` markers in pattern matching.
    let otool = Command::new("otool")
        .arg("-tvV")
        .arg(dylib)
        .output()
        .map_err(|e| format!("disasm: neither objdump nor otool worked: {e}"))?;
    if !otool.status.success() {
        return Err(format!("disasm: otool failed on {}", dylib.display()));
    }
    Ok(String::from_utf8_lossy(&otool.stdout).into_owned())
}

/// The instructions of a listing, lines that are not instructions dropped.
fn insns(disasm: &str) -> impl Iterator<Item = Insn<'_>> {
    disasm.lines().filter_map(Insn::parse)
}

fn check_no_indirect_call(disasm: &str) -> CheckOutcome {
    // An indirect call whose target the body loaded from an import's GOT
    // slot is the dynamic linker's (`memset`, `syscall`), not a stored
    // function pointer, and is reported but not counted. Every other one is
    // a target read from data, which is the regression this check refuses.
    let calls = disasm::indirect_calls(disasm);
    let mut imports = calls.to_imports.clone();
    imports.sort();
    imports.dedup();
    let detail = if calls.unresolved.is_empty() {
        format!(
            "0 indirect calls through a stored pointer ({} to imports via the GOT: {})",
            calls.to_imports.len(),
            list_or_none(&imports)
        )
    } else {
        format!(
            "{} indirect calls through a stored pointer, at {} ({} more to imports via the GOT)",
            calls.unresolved.len(),
            calls.unresolved.join(", "),
            calls.to_imports.len()
        )
    };
    CheckOutcome {
        name: "1. zero indirect call in inner body",
        pass: calls.unresolved.is_empty(),
        detail,
    }
}

fn list_or_none(names: &[String]) -> String {
    if names.is_empty() { "none".to_string() } else { names.join(", ") }
}

fn check_indexed_addressing(disasm: &str) -> CheckOutcome {
    // aarch64 indexed load: `ldr xN, [xBase, xIdx, lsl #SCALE]`.
    // x86_64 AT&T indexed operand: `(%base,%idx,SCALE)`.
    let total = insns(disasm)
        .filter(|i| {
            let o = i.operands;
            (o.contains('[') && o.contains(", lsl #"))
                || ["2)", "4)", "8)"]
                    .iter()
                    .any(|s| o.contains(&format!(",{s}")) && o.contains(",%"))
        })
        .count();
    CheckOutcome {
        name:   "2. indexed addressing on column loads",
        pass:   total > 0,
        detail: format!("{total} indexed-addressing instructions"),
    }
}

fn check_no_stack_in_inner_loop(disasm: &str) -> CheckOutcome {
    // Stack-relative loads/stores anywhere in the symbol body
    // signal register spills. Prologue/epilogue stack ops are
    // counted too; the check reports the total rather than
    // locating the loop, and gates nothing.
    let n = insns(disasm)
        .filter(|i| i.operands.contains("[sp") || i.operands.contains("(%rsp)"))
        .count();
    CheckOutcome {
        name:   "3. no stack-relative accesses in inner loop",
        pass:   n == 0,
        detail: format!("{n} stack-relative accesses"),
    }
}

fn check_immediate_morsel_size(disasm: &str, morsel_immediates: &[u64]) -> CheckOutcome {
    let all: Vec<Insn<'_>> = insns(disasm).collect();
    let found = morsel_immediates
        .iter()
        .filter(|&&m| {
            // aarch64 `#256` / `#0x100`, x86_64 AT&T `$256` / `$0x100`.
            let forms =
                [format!("#{m}"), format!("#0x{m:x}"), format!("${m}"), format!("$0x{m:x}")];
            all.iter().any(|i| {
                i.operands
                    .split([',', ' '])
                    .any(|t| forms.iter().any(|f| t.trim() == f))
            })
        })
        .count();
    CheckOutcome {
        name:   "4. immediate-constant morsel size baked",
        pass:   found > 0,
        detail: format!(
            "found {found} of {} expected immediates",
            morsel_immediates.len()
        ),
    }
}

fn check_no_bl_to_helpers(disasm: &str) -> CheckOutcome {
    // A direct call to a per-record helper that should have inlined, named
    // by the `<symbol>` objdump prints beside the target address.
    let hits = insns(disasm)
        .filter(|i| i.is_call() && !i.is_indirect_call())
        .filter(|i| {
            let target = i.comment_symbol().unwrap_or("");
            ["morsel", "fiber_dispatch", "wu_fn", "invoke_wu", "iter_morsel"]
                .iter()
                .any(|h| target.contains(h))
        })
        .count();
    CheckOutcome {
        name:   "5. no direct call to morsel/fiber/wu_fn helpers",
        pass:   hits == 0,
        detail: format!("{hits} direct calls to helper symbols"),
    }
}

impl fmt::Display for CheckReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "### variant `{}`", self.variant)?;
        writeln!(f)?;
        // The file name, not the path: the path is the build directory of
        // whoever ran it, and a committed report regenerates byte-identical.
        let dylib = self.dylib.file_name().map_or_else(
            || self.dylib.display().to_string(),
            |n| n.to_string_lossy().into_owned(),
        );
        writeln!(f, "- dylib: `{dylib}`")?;
        writeln!(f, "- symbol: `{}`", self.symbol)?;
        writeln!(
            f,
            "- status: {}",
            if self.all_pass() { "PASS" } else { "FAIL" }
        )?;
        writeln!(f)?;
        writeln!(f, "| Check | Result | Detail |")?;
        writeln!(f, "|---|---|---|")?;
        for o in &self.outcomes {
            writeln!(
                f,
                "| {} | {} | {} |",
                o.name,
                if o.pass { "PASS" } else { "FAIL" },
                o.detail
            )?;
        }
        writeln!(f)
    }
}

impl fmt::Display for BenchReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# 5-check disasm report: {} n={}", self.bench, self.n)?;
        writeln!(f)?;
        writeln!(
            f,
            "Canonical 5 checks (see `codegen_dispatch_axis_a_assertions.md`):"
        )?;
        writeln!(f)?;
        writeln!(f, "1. Zero indirect call in inner body.")?;
        writeln!(f, "2. Indexed addressing on column loads.")?;
        writeln!(f, "3. No stack-relative accesses inside the inner loop.")?;
        writeln!(f, "4. Immediate-constant morsel size baked in.")?;
        writeln!(
            f,
            "5. No direct call to morsel-size, fiber-dispatch, wu_fn helpers."
        )?;
        writeln!(f)?;
        writeln!(
            f,
            "Status: {}",
            if self.any_fail() { "AT LEAST ONE FAIL" } else { "ALL PASS" }
        )?;
        writeln!(f)?;
        for r in &self.reports {
            writeln!(f, "{}", r)?;
        }
        Ok(())
    }
}

/// Run 5-check across every variant in the bench config + write
/// the consolidated report to `<bench>_n<N>_5check.md`.
pub fn run_and_write(
    bench: &str,
    n: usize,
    variant_paths: &[PathBuf],
    morsel_immediates: &[u64],
    report_dir: &Path,
) -> Result<BenchReport, String> {
    let mut reports = Vec::with_capacity(variant_paths.len());
    for dylib in variant_paths {
        let r = run_checks(dylib, "bench_entry", morsel_immediates)?;
        reports.push(r);
    }
    let bench_report = BenchReport {
        bench: bench.to_string(),
        n,
        reports,
    };
    let path = report_dir.join(format!("{}_n{}_5check.md", bench, n));
    fs::write(&path, bench_report.to_string())
        .map_err(|e| format!("write 5check report {}: {e}", path.display()))?;
    Ok(bench_report)
}
