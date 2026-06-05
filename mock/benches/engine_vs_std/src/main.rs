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

use engine_vs_std::{SIZES, WORKLOADS, measure};

fn main() {
    println!("# engine_vs_std (#660/#664): single-core engine vs optimal fused std");
    println!(
        "# workload, N, engine_startup_ns(med/min), std_startup_ns(med/min), \
         engine_runtime_ns(med/min), std_runtime_ns(med/min), startup_ratio, runtime_ratio, \
         checksum_ok"
    );
    for &name in &WORKLOADS {
        for &n in &SIZES {
            let m = measure(name, n);
            println!(
                "{}, {n}, {}/{}, {}/{}, {}/{}, {}/{}, {:.3}, {:.3}, {}",
                m.name,
                m.eng_startup.median_ns,
                m.eng_startup.min_ns,
                m.std_startup.median_ns,
                m.std_startup.min_ns,
                m.eng_runtime.median_ns,
                m.eng_runtime.min_ns,
                m.std_runtime.median_ns,
                m.std_runtime.min_ns,
                m.startup_ratio(),
                m.runtime_ratio(),
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
