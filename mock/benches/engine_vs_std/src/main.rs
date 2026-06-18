//! #660/#664 reporting bench: prints the engine-vs-std measurement matrix for
//! every workload at every size. The pass-or-fail assertions live in
//! `tests/perf_gate.rs`; this binary only reports.
//!
//! Op directive (2026-06-04): before any multi-threaded work, check whether the
//! engine running single-core beats the same workload written on a std base as
//! optimally as possible, on STARTUP (get ready) and RUNTIME (process to
//! finish). The single-core design target is parity-or-better (0.95x to 1.02x);
//! the gap measured here is the distance to Gate-1 (#661) perf-done, closed by
//! Phase D (#340). See the crate-level docs and
//! `mock/research/202606052000_single-core-engine-ideal-vs-actual-audit.md`.
//!
//! Run: `caffeinate -dimsu cargo run --release` (darwin pinning; release is
//! opt3 / lto-fat / cgu1 per Cargo.toml).

use engine_vs_std::{Mode, SIZES, WORKLOADS, expected_ratio, measure};

fn main() {
    println!("# engine_vs_std (#660/#664): engine vs optimal std across the full spectrum");
    println!(
        "# workload, N, eng_runtime_ns, std_runtime_ns, runtime_ratio, runtime_expect, \
         eng_par_ns, par_ratio, par_expect, startup_ratio, checksum_ok"
    );
    for &name in &WORKLOADS {
        for &n in &SIZES {
            let m = measure(name, n);
            let par_ns = m.eng_runtime_par.map(|p| p.median_ns.to_string()).unwrap_or_else(|| "-".into());
            let par_ratio = m.par_ratio().map(|r| format!("{r:.3}")).unwrap_or_else(|| "-".into());
            let par_expect = if m.eng_runtime_par.is_some() {
                format!("{:.2}", expected_ratio(name, n, Mode::Parallel))
            } else {
                "-".into()
            };
            println!(
                "{}, {n}, {}, {}, {:.3}, {:.2}, {}, {}, {}, {:.3}, {}",
                m.name,
                m.eng_runtime.median_ns,
                m.std_runtime.median_ns,
                m.runtime_ratio(),
                expected_ratio(name, n, Mode::SingleCore),
                par_ns,
                par_ratio,
                par_expect,
                m.startup_ratio(),
                m.checksum_ok,
            );
            if !m.checksum_ok {
                eprintln!(
                    "CHECKSUM MISMATCH {} N={n}: engine={:#x} std={:#x}",
                    m.name, m.eng_hash, m.std_hash
                );
            }
        }
    }
}
