//! `--summary <results dir>`: every promoted CSV folded into one table.
//!
//! A CSV row is one batch: its `algo_ns` is the mean time of one call over
//! the batch's calls. The summary groups the batches of every pass by
//! section, record count, mode and arm, and writes the median, the quartiles
//! and the fastest batch of each group, with the median's ratio to the
//! `sequential` and the `kernel_seq` arms of the same section, size and mode.
//! The harness's own report mixes the warm and cold modes into one figure;
//! the two are kept apart here because their spreads differ by an order of
//! magnitude on this bench.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use crate::section;

/// The arms, in the sequence the table lists them.
const ARMS: [&str; 7] = [
    "sequential",
    "nway",
    "head_tail",
    "head_tail_dyn",
    "kernel_seq",
    "kernel_nway",
    "std_parallel",
];

/// The modes, in the sequence the table lists them.
const MODES: [&str; 2] = ["warm", "cold"];

/// The `q` quantile of sorted `xs`, interpolated between neighbours.
fn quantile(xs: &[f64], q: f64) -> f64 {
    let i = q * (xs.len() - 1) as f64;
    let (lo, hi) = (i.floor() as usize, i.ceil() as usize);
    xs[lo] + (xs[hi] - xs[lo]) * (i - lo as f64)
}

/// One group's statistics, in nanoseconds per call.
///
/// Every pass runs an arm in a worker subprocess of its own, so the spread
/// between passes is the spread between processes; `pass_lo` and `pass_hi`
/// are the lowest and highest of the per-pass medians.
struct Stats {
    median:  f64,
    p25:     f64,
    p75:     f64,
    min:     f64,
    pass_lo: f64,
    pass_hi: f64,
    batches: usize,
}

impl Stats {
    fn of(batches: &[(String, f64)]) -> Self {
        let mut xs: Vec<f64> = batches.iter().map(|(_, x)| *x).collect();
        xs.sort_by(f64::total_cmp);
        let mut passes: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
        for (pass, x) in batches {
            passes.entry(pass).or_default().push(*x);
        }
        let mut medians: Vec<f64> = passes
            .into_values()
            .map(|mut p| {
                p.sort_by(f64::total_cmp);
                quantile(&p, 0.5)
            })
            .collect();
        medians.sort_by(f64::total_cmp);
        Self {
            median:  quantile(&xs, 0.5),
            p25:     quantile(&xs, 0.25),
            p75:     quantile(&xs, 0.75),
            min:     xs[0],
            pass_lo: medians[0],
            pass_hi: medians[medians.len() - 1],
            batches: xs.len(),
        }
    }
}

/// `(workload, workers, section, n)`, the key a table block is sorted by.
type Block = (String, u32, String, usize);

/// Fold every CSV under `dir` into `dir/summary.csv`.
pub fn run(dir: &Path) -> Result<PathBuf, String> {
    let mut files = Vec::new();
    collect(dir, &mut files)?;
    if files.is_empty() {
        return Err(format!("no result CSV under {}", dir.display()));
    }
    let mut groups: BTreeMap<Block, BTreeMap<(String, String), Vec<(String, f64)>>> =
        BTreeMap::new();
    for file in &files {
        read(file, &mut groups)?;
    }
    let mut out = String::from(
        "workload,workers,section,n,mode,arm,median_ns,p25_ns,p75_ns,min_ns,pass_lo_ns,\
         pass_hi_ns,batches,vs_sequential,vs_kernel_seq\n",
    );
    for ((workload, workers, section, n), arms) in &groups {
        for mode in MODES {
            let stats: Vec<(&str, Stats)> = ARMS
                .iter()
                .filter_map(|&arm| {
                    let xs = arms.get(&(mode.to_string(), arm.to_string()))?;
                    Some((arm, Stats::of(xs)))
                })
                .collect();
            let median_of = |name: &str| {
                stats
                    .iter()
                    .find(|(a, _)| *a == name)
                    .map(|(_, s)| s.median)
            };
            let seq = median_of("sequential");
            let kseq = median_of("kernel_seq");
            for (arm, s) in &stats {
                let ratio = |base: Option<f64>| {
                    base.map(|b| format!("{:.3}", s.median / b))
                        .unwrap_or_default()
                };
                writeln!(
                    out,
                    "{workload},{workers},{section},{n},{mode},{arm},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{},{},{}",
                    s.median,
                    s.p25,
                    s.p75,
                    s.min,
                    s.pass_lo,
                    s.pass_hi,
                    s.batches,
                    ratio(seq),
                    ratio(kseq),
                )
                .expect("writing to a String");
            }
        }
    }
    let path = dir.join("summary.csv");
    std::fs::write(&path, out).map_err(|e| format!("writing {}: {e}", path.display()))?;
    Ok(path)
}

/// Every `<section>_n<N>.csv` under `dir`, skipping hidden directories (the
/// harness's in-flight staging) and the summary itself.
fn collect(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("reading {}: {e}", dir.display()))?;
    for entry in entries {
        let path = entry
            .map_err(|e| format!("reading {}: {e}", dir.display()))?
            .path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name.starts_with('.') {
            continue;
        }
        if path.is_dir() {
            collect(&path, files)?;
        } else if name.ends_with(".csv") && name != "summary.csv" {
            files.push(path);
        }
    }
    Ok(())
}

/// Add one CSV's batches to `groups`.
fn read(
    file: &Path,
    groups: &mut BTreeMap<Block, BTreeMap<(String, String), Vec<(String, f64)>>>,
) -> Result<(), String> {
    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let bad = |what: &str| format!("{}: {what}", file.display());
    let (name, n) = stem
        .rsplit_once("_n")
        .ok_or_else(|| bad("not named <section>_n<N>.csv"))?;
    let n: usize = n.parse().map_err(|_| bad("record count is not a number"))?;
    let sec = section::parse(name).ok_or_else(|| bad("not a <workload>_w<W> section"))?;
    let text = std::fs::read_to_string(file).map_err(|e| bad(&e.to_string()))?;
    let mut lines = text.lines();
    let header: Vec<&str> = lines
        .next()
        .ok_or_else(|| bad("empty"))?
        .split(',')
        .collect();
    let col = |c: &str| {
        header
            .iter()
            .position(|h| *h == c)
            .ok_or_else(|| bad(&format!("no `{c}` column")))
    };
    let (pass, mode, variant, algo) =
        (col("pass")?, col("mode")?, col("variant")?, col("algo_ns")?);
    let block = groups
        .entry((sec.workload.to_string(), sec.workers, name.to_string(), n))
        .or_default();
    for line in lines {
        let f: Vec<&str> = line.split(',').collect();
        let ns: f64 = f
            .get(algo)
            .and_then(|v| v.parse().ok())
            .ok_or_else(|| bad(&format!("unreadable row `{line}`")))?;
        block
            .entry((f[mode].to_string(), f[variant].to_string()))
            .or_default()
            .push((f[pass].to_string(), ns));
    }
    Ok(())
}
