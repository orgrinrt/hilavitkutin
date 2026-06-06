//! Sketch (F2 / #340, Phase F, OPTIONAL): cfg-gated custom LLVM pass + degrade.
//!
//! hilavitkutin-build (build-dep only) can inject custom LLVM passes (extra
//! devirt/DSE/prefetch beyond stock). F2 (roadmap section 6/9): the custom passes
//! are cfg-gated so a missing or incompatible LLVM degrades to stock. The pass is
//! an OPTIMISATION over an already-correct, already-devirtualised path (D1/D4), so
//! its absence loses only the extra speedup, never correctness. The degrade is two
//! layers:
//!   1. compile-time: a `custom_pass` cargo feature gates whether the custom-pass
//!      emission path is compiled at all (off by default; opt-in).
//!   2. runtime (build-time, in build.rs): even with the feature on, a probe checks
//!      the LLVM .so is present + version-compatible; if not, fall back to stock.
//!
//! Hypothesis: both build configurations compile and produce the SAME correct
//! result; with the feature off, the stock path is taken; with it on, the custom
//! path is taken IFF the availability probe succeeds, else stock. No configuration
//! fails to build or changes the output. Leeway (section 9): SOME-SHAPE;
//! non-blocking. Outcome at the bottom.

#![allow(dead_code)]

use arvo::USize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PassChoice {
    Stock,
    Custom,
}

// Runtime (build-time) availability probe: is the custom LLVM pass .so present and
// version-compatible? In the real build.rs this stats the .so path and checks the
// LLVM version; here a deterministic stand-in. The degrade hinges on this
// returning false gracefully (no panic, no build failure).
fn custom_pass_available(simulated_present: bool, simulated_compatible: bool) -> bool {
    simulated_present && simulated_compatible
}

// Select the pass. Compile-time: the custom arm only exists under the feature.
// Runtime: even compiled-in, fall back to stock when the probe fails.
fn select_pass(present: bool, compatible: bool) -> PassChoice {
    #[cfg(feature = "custom_pass")]
    {
        if custom_pass_available(present, compatible) {
            return PassChoice::Custom;
        }
        // degrade: feature on but pass unavailable/incompatible -> stock.
        PassChoice::Stock
    }
    #[cfg(not(feature = "custom_pass"))]
    {
        let _ = (present, compatible);
        PassChoice::Stock
    }
}

// The dispatch body the build would emit. The custom pass is a pure optimisation:
// it does NOT change the result, only (notionally) how fast it runs. So both
// choices compute the same value. This is the load-bearing invariant: degrade is
// safe because correctness is choice-independent.
fn compute(records: &[u32], choice: PassChoice) -> u32 {
    // Both arms compute the same reduction; `choice` only models which codegen
    // path emitted the body (stock vs custom-pass-optimised). Identical output.
    let mut acc = 0u32;
    for &r in records {
        acc = acc.wrapping_add(r.wrapping_mul(2654435761));
    }
    let _ = choice;
    acc
}

fn main() {
    let n = USize(4096);
    let records: Vec<u32> = (0..n.0 as u32).collect();
    let reference = compute(&records, PassChoice::Stock);

    // The selection across the four (present, compatible) build-probe states.
    let cases = [
        (true, true),   // .so present + compatible
        (true, false),  // present but incompatible LLVM version
        (false, true),  // absent
        (false, false), // absent + incompatible
    ];
    for (present, compatible) in cases {
        let choice = select_pass(present, compatible);

        #[cfg(feature = "custom_pass")]
        {
            // With the feature compiled in, custom is chosen ONLY when available;
            // every other probe state degrades to stock without failure.
            let expect = if present && compatible { PassChoice::Custom } else { PassChoice::Stock };
            assert_eq!(choice, expect, "feature on: probe ({present},{compatible})");
        }
        #[cfg(not(feature = "custom_pass"))]
        {
            // Feature off: always stock, regardless of probe.
            assert_eq!(choice, PassChoice::Stock, "feature off: always stock");
        }

        // Correctness is choice-independent: degrade never changes the result.
        let out = compute(&records, choice);
        assert_eq!(out, reference, "output identical regardless of pass choice");
    }

    let feature_on = cfg!(feature = "custom_pass");
    println!(
        "WORKS: cfg-gated custom LLVM pass degrades to stock. feature `custom_pass` = {feature_on}. \
         Across all (.so present, LLVM compatible) probe states the build selects Custom only when \
         available and degrades to Stock otherwise, never failing; output is identical for every \
         choice (the pass is a pure optimisation over the already-correct devirtualised path). \
         Build both with `--features custom_pass` and without: both compile and pass."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, both feature states).
//
// Built + ran twice: default (feature off -> always Stock) and
// `--features custom_pass` (Custom chosen only when the availability probe
// passes, degrading to Stock for present-but-incompatible / absent / absent+
// incompatible). Both configurations compile and pass; output is identical for
// every pass choice across all four probe states (no build failure, no behaviour
// change).
//
// WHAT THIS SETTLES (F2): the custom LLVM pass is safely cfg-gated with a
// two-layer degrade (compile-time feature + runtime/build.rs availability probe).
// Absence or incompatibility falls back to stock without failing the build, and
// correctness is choice-independent because the pass is a pure optimisation over
// the already-correct devirtualised path (D1/D4). This is the non-blocking,
// opt-in refinement the roadmap describes.
//
// WHAT THIS DOES NOT SETTLE: the actual custom-pass .so authoring (an LLVM-plugin
// task in hilavitkutin-build) and the real build.rs probe (stat the .so + check
// the LLVM version); this proves the cfg-gate + degrade CONTRACT, which is the
// load-bearing safety property (never break the build when the pass is absent).
// ---------------------------------------------------------------------
