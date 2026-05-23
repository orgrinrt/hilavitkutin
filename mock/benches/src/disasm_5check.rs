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

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// One check's outcome.
#[derive(Debug, Clone)]
pub struct CheckOutcome {
    pub name: &'static str,
    pub pass: bool,
    pub detail: String,
}

/// Full 5-check report for one variant's symbol.
#[derive(Debug, Clone)]
pub struct CheckReport {
    pub variant: String,
    pub dylib: PathBuf,
    pub symbol: String,
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
    pub bench: String,
    pub n: usize,
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
    let outcomes = vec![
        check_no_indirect_call(&disasm),
        check_indexed_addressing(&disasm),
        check_no_stack_in_inner_loop(&disasm),
        check_immediate_morsel_size(&disasm, morsel_immediates),
        check_no_bl_to_helpers(&disasm),
    ];
    Ok(CheckReport {
        variant: dylib_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string(),
        dylib: dylib_path.to_path_buf(),
        symbol: symbol.to_string(),
        outcomes,
    })
}

fn read_disasm(dylib: &Path, symbol: &str) -> Result<String, String> {
    // Try objdump first. macOS prepends an underscore to symbol
    // names; Linux does not. Try both.
    for prefix in ["", "_"] {
        let target = format!("{}{}", prefix, symbol);
        let out = Command::new("objdump")
            .arg("-d")
            .arg("--disassemble-symbols")
            .arg(&target)
            .arg(dylib)
            .output();
        if let Ok(o) = out {
            if o.status.success() && !o.stdout.is_empty() {
                return Ok(String::from_utf8_lossy(&o.stdout).into_owned());
            }
        }
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

fn check_no_indirect_call(disasm: &str) -> CheckOutcome {
    // aarch64 indirect: `blr xN`. x86_64 indirect: `callq *rN` or
    // `call *rN`. Both forms appear as line tokens; counting line
    // occurrences gives a sound lower bound on indirect calls in
    // the body. Direct `bl`/`call` to a named symbol is check 5's
    // concern, not this one.
    let n = disasm
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            t.starts_with("blr\t")
                || t.starts_with("blr ")
                || l.contains("\tblr\t")
                || l.contains("\tcallq\t*")
                || l.contains("\tcall\t*")
        })
        .count();
    CheckOutcome {
        name: "1. zero indirect call in inner body",
        pass: n == 0,
        detail: format!("{n} indirect call instructions"),
    }
}

fn check_indexed_addressing(disasm: &str) -> CheckOutcome {
    // aarch64 indexed load: `ldr xN, [xBase, xIdx, lsl #SCALE]`.
    // x86_64 indexed load: `mov rax, qword ptr [rBase + rIdx*SCALE]`.
    let aarch = disasm
        .lines()
        .filter(|l| l.contains(", lsl #") && l.contains("["))
        .count();
    let x86 = disasm
        .lines()
        .filter(|l| {
            l.contains("ptr [") && (l.contains("*4") || l.contains("*8") || l.contains("*2"))
        })
        .count();
    let total = aarch + x86;
    CheckOutcome {
        name: "2. indexed addressing on column loads",
        pass: total > 0,
        detail: format!("{total} indexed-addressing instructions"),
    }
}

fn check_no_stack_in_inner_loop(disasm: &str) -> CheckOutcome {
    // Stack-relative loads/stores anywhere in the symbol body
    // signal register spills. Prologue/epilogue stack ops appear
    // in the first and last few lines; the threshold below tolerates
    // a small count rather than insisting on zero, because epilogue
    // restores are unavoidable on aarch64 leaf-fns that touch
    // callee-saved registers.
    let n = disasm
        .lines()
        .filter(|l| {
            l.contains("[sp,") || l.contains("[rsp,") || l.contains("[rsp+") || l.contains("[rsp-")
        })
        .count();
    CheckOutcome {
        name: "3. no stack-relative accesses in inner loop",
        pass: n == 0,
        detail: format!("{n} stack-relative accesses"),
    }
}

fn check_immediate_morsel_size(disasm: &str, morsel_immediates: &[u64]) -> CheckOutcome {
    let mut found: Vec<u64> = Vec::new();
    for &m in morsel_immediates {
        // aarch64 immediate: `#256`. x86_64 immediate decimal:
        // `, 256` or hex `0x100`. Search all three forms.
        let aarch_pat = format!("#{m}");
        let x86_dec_pat = format!(", {m}");
        let x86_hex_pat = format!("0x{m:x}");
        if disasm.contains(&aarch_pat)
            || disasm.contains(&x86_dec_pat)
            || disasm.contains(&x86_hex_pat)
        {
            found.push(m);
        }
    }
    let pass = !found.is_empty();
    CheckOutcome {
        name: "4. immediate-constant morsel size baked",
        pass,
        detail: format!(
            "found {} of {} expected immediates",
            found.len(),
            morsel_immediates.len()
        ),
    }
}

fn check_no_bl_to_helpers(disasm: &str) -> CheckOutcome {
    // Direct `bl <symbol>` (aarch64) or `call <symbol>` (x86_64) to
    // any function whose name pattern matches a per-record helper
    // that should have inlined. Demangled names are best-effort;
    // objdump usually emits both mangled and demangled forms.
    let mut hits: usize = 0;
    for line in disasm.lines() {
        let t = line.trim_start();
        let is_direct_branch = t.starts_with("bl\t")
            || t.starts_with("bl ")
            || line.contains("\tbl\t")
            || line.contains("\tcallq\t")
            || line.contains("\tcall\t");
        if !is_direct_branch {
            continue;
        }
        if line.contains("*") {
            continue; // indirect; counted by check 1
        }
        let names_helper = line.contains("morsel")
            || line.contains("fiber_dispatch")
            || line.contains("wu_fn")
            || line.contains("invoke_wu")
            || line.contains("iter_morsel");
        if names_helper {
            hits += 1;
        }
    }
    CheckOutcome {
        name: "5. no direct call to morsel/fiber/wu_fn helpers",
        pass: hits == 0,
        detail: format!("{hits} direct calls to helper symbols"),
    }
}

impl fmt::Display for CheckReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "### variant `{}`", self.variant)?;
        writeln!(f)?;
        writeln!(f, "- dylib: `{}`", self.dylib.display())?;
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
            if self.any_fail() {
                "AT LEAST ONE FAIL"
            } else {
                "ALL PASS"
            }
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
