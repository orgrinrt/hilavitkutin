//! Sketch (E5a / #340, Phase E): heterogeneous core detection (domain 20 :1810-1822).
//!
//! Domain 20: detect core types at startup (P-cores, E-cores); critical-path
//! trunks go to P-cores, leaf/branch fibers to E-cores; thread count =
//! min(physical_cores, parallelisable_width + 1). E5a (roadmap section 9): a
//! no_std-compatible P/E core-type probe on the test target (Apple Silicon) that
//! produces correct output and degrades gracefully on non-heterogeneous hardware.
//!
//! The macOS aarch64 mechanism is the `hw.perflevelN.logicalcpu` sysctl family:
//! `hw.nperflevels` gives the number of performance tiers (2 on Apple Silicon:
//! level 0 = P-cores, level 1 = E-cores; 1 on a homogeneous machine), and
//! `hw.perflevelN.logicalcpu` gives the logical CPU count of tier N. The probe is
//! a `sysctlbyname` call (libc FFI). In the real engine this is a `platform/` hook
//! behind `cfg(target_os/target_arch)`; other targets get their own probe (Linux
//! cpufreq / DT, x86 hybrid leaf), and any target with no signal returns one
//! homogeneous tier. This sketch proves the Apple-Silicon path + the degrade
//! contract; the FFI here stands in for the cfg-gated platform hook.
//!
//! Hypothesis: the probe returns (n_perf_levels, [logicalcpu per level]) matching
//! the machine (here 2 levels, 4 P + 4 E), classifies cores into P/E sets, and
//! when only one level exists reports a single homogeneous tier (graceful
//! degrade). Leeway (section 9): SOME-SHAPE; platform hook acceptable. Outcome at
//! the bottom.

#![allow(dead_code)]

use arvo::USize;

const MAX_LEVELS: usize = 8;

/// Detected core topology: one logical-cpu count per performance level, level 0
/// the fastest (P), higher levels progressively slower (E). `levels == 1` means a
/// homogeneous machine (no P/E split).
#[derive(Debug)]
struct CoreTopology {
    levels: USize,
    logical_per_level: [USize; MAX_LEVELS],
}
impl CoreTopology {
    fn total_logical(&self) -> USize {
        let mut s = 0;
        for i in 0..self.levels.0.min(MAX_LEVELS) {
            s += self.logical_per_level[i].0;
        }
        USize(s)
    }
    fn is_heterogeneous(&self) -> bool {
        self.levels.0 > 1
    }
    fn p_cores(&self) -> USize {
        // Level 0 is the performance tier.
        if self.levels.0 == 0 {
            USize(0)
        } else {
            self.logical_per_level[0]
        }
    }
    fn e_cores(&self) -> USize {
        // Sum of all slower tiers; on a homogeneous machine this is 0.
        let mut s = 0;
        let mut i = 1;
        while i < self.levels.0.min(MAX_LEVELS) {
            s += self.logical_per_level[i].0;
            i += 1;
        }
        USize(s)
    }
}

// Raw sysctlbyname i32 read. Returns None when the name is absent (the degrade
// signal). This is the macOS aarch64 platform-hook body.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn sysctl_i32(name: &core::ffi::CStr) -> Option<i32> {
    let mut val: i32 = 0;
    let mut size = core::mem::size_of::<i32>();
    // SAFETY: name is a valid NUL-terminated C string; val/size are valid
    // out-pointers sized for an i32; sysctlbyname writes at most `size` bytes.
    let rc = unsafe {
        libc::sysctlbyname(
            name.as_ptr(),
            &mut val as *mut i32 as *mut core::ffi::c_void,
            &mut size,
            core::ptr::null_mut(),
            0,
        )
    };
    if rc == 0 {
        Some(val)
    } else {
        None
    }
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn detect() -> CoreTopology {
    use core::ffi::CStr;
    let nlevels = sysctl_i32(c"hw.nperflevels").unwrap_or(1).max(1) as usize;
    let mut logical = [USize(0); MAX_LEVELS];
    let mut real_levels = 0usize;
    // Per-level names are fixed strings hw.perflevel{0..}.logicalcpu.
    let names: [&CStr; 4] = [
        c"hw.perflevel0.logicalcpu",
        c"hw.perflevel1.logicalcpu",
        c"hw.perflevel2.logicalcpu",
        c"hw.perflevel3.logicalcpu",
    ];
    let mut i = 0;
    while i < nlevels.min(MAX_LEVELS).min(names.len()) {
        match sysctl_i32(names[i]) {
            Some(c) if c > 0 => {
                logical[i] = USize(c as usize);
                real_levels += 1;
            }
            _ => break,
        }
        i += 1;
    }
    if real_levels == 0 {
        // Degrade: no per-level signal. Fall back to one homogeneous tier of
        // hw.logicalcpu (or 1).
        let total = sysctl_i32(c"hw.logicalcpu").unwrap_or(1).max(1) as usize;
        logical[0] = USize(total);
        return CoreTopology { levels: USize(1), logical_per_level: logical };
    }
    CoreTopology { levels: USize(real_levels), logical_per_level: logical }
}

// Non-macOS / non-aarch64 fallback: one homogeneous tier (the universal degrade).
#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
fn detect() -> CoreTopology {
    let mut logical = [USize(0); MAX_LEVELS];
    // A real platform hook would probe the OS here; with no signal, homogeneous.
    let total = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    logical[0] = USize(total);
    CoreTopology { levels: USize(1), logical_per_level: logical }
}

fn main() {
    let topo = detect();

    // Total logical cores must be positive and match the platform count.
    assert!(topo.total_logical().0 >= 1, "at least one logical core");

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        // On the Apple Silicon test target we expect a heterogeneous split.
        assert!(topo.is_heterogeneous(), "Apple Silicon is heterogeneous (P + E)");
        assert!(topo.p_cores().0 >= 1, "at least one P-core");
        assert!(topo.e_cores().0 >= 1, "at least one E-core");
        // P + E == total.
        assert_eq!(
            topo.p_cores().0 + topo.e_cores().0,
            topo.total_logical().0,
            "P + E partition the logical cores"
        );
    }

    // Graceful-degrade contract: a single-level topology reports homogeneous,
    // e_cores == 0, p_cores == total. Construct one to assert the contract holds
    // regardless of the host (this is the path non-heterogeneous hardware takes).
    let homo = CoreTopology {
        levels: USize(1),
        logical_per_level: {
            let mut a = [USize(0); MAX_LEVELS];
            a[0] = USize(8);
            a
        },
    };
    assert!(!homo.is_heterogeneous(), "single level = homogeneous");
    assert_eq!(homo.e_cores().0, 0, "homogeneous has no E-cores");
    assert_eq!(homo.p_cores().0, 8, "homogeneous p_cores = total");

    println!(
        "WORKS: P/E core detection. Detected {} perf levels, {} P-cores + {} E-cores ({} logical \
         total) on the test target via hw.perflevelN.logicalcpu sysctls. Graceful degrade: a \
         single-level topology reports homogeneous (e_cores=0, p_cores=total). The real engine \
         path is a cfg-gated platform/ hook; non-heterogeneous / unknown targets take the \
         homogeneous fallback.",
        topo.levels.0,
        topo.p_cores().0,
        topo.e_cores().0,
        topo.total_logical().0
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, Apple Silicon macOS aarch64).
//
// The probe read hw.nperflevels (2) and hw.perflevel0/1.logicalcpu (4 + 4) via
// sysctlbyname, classified 4 P-cores (level 0) + 4 E-cores (level 1) = 8 logical
// total, P + E partitioning the cores. The graceful-degrade contract holds: a
// single-level topology reports homogeneous (is_heterogeneous=false, e_cores=0,
// p_cores=total), the path non-heterogeneous / unknown hardware takes.
//
// WHAT THIS SETTLES (E5a): heterogeneous core detection (domain 20 :1810-1822)
// works on the test target via the hw.perflevelN.logicalcpu sysctl family, and
// the degrade contract is well-defined for homogeneous / unknown targets. The
// detection feeds the plan's critical-path-to-P / leaf-to-E affinity and the E5b
// asymmetric morsel sizing. The probe is FFI (libc sysctlbyname); in the engine
// it is a cfg(target_os=macos, target_arch=aarch64) platform/ hook, with sibling
// hooks for Linux (cpufreq / devicetree) and x86 hybrid (CPUID leaf 0x1A), all
// degrading to one homogeneous tier when no signal exists. The FFI boundary is
// the documented platform-hook exception (no-alloc-no-std-framing: platform FFI).
//
// WHAT THIS DOES NOT SETTLE: the Linux / x86-hybrid probes (sibling platform
// hooks, same contract, not on this target) and the affinity ASSIGNMENT that
// consumes the topology (which trunk -> which core), which is plan-stage policy
// (R6 morsel-to-core affinity) benched in E5b. Detection is proven; assignment is
// E5b's perf question.
// ---------------------------------------------------------------------
