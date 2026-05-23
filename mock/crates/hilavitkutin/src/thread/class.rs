//! Per-OS heterogeneous-core detection (Topic 6 axis E).
//!
//! P-cores take critical-path trunks with larger morsels; E-cores
//! handle branches / leaves at proportionally smaller widths. The
//! engine queries `classify_cores()` once at pool construction
//! and bakes the result into the per-core dispatch programs.
//!
//! Detection paths, in priority order:
//!
//! - Linux: read `/sys/devices/system/cpu/cpuN/topology/cluster_id`
//!   plus `cpufreq/cpuinfo_max_freq` to partition by max frequency.
//!   The highest-frequency cluster is P; the rest E.
//! - macOS: `sysctlbyname("hw.perflevel0.physicalcpu")` reports the
//!   count of P-cores (perflevel 0); the remainder are E.
//! - x86_64: CPUID leaf `0x1A` (when available, Alder Lake+) names
//!   the core type per logical processor. Fallback to the OS path
//!   above when leaf 0x1A is absent.
//! - Windows: `GetSystemCpuSetInformation` exposes EfficiencyClass
//!   per logical processor (compile-stubbed; no Win32 dep in
//!   no_std today).
//!
//! Stub fallback when every detection path fails: all-P. The
//! caller's `CoreAssignment` is unaffected (P-cores receive
//! critical-path trunks; with all-P every core is on the same
//! footing).

use arvo::USize;

/// Heterogeneous-core class.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CoreClass {
    /// Performance core: critical-path trunks + larger morsels.
    P,
    /// Efficiency core: branches/leaves + smaller morsels.
    E,
}

impl Default for CoreClass {
    fn default() -> Self {
        Self::P
    }
}

/// Worst-case logical processor count the engine pre-allocates for.
/// Heterogeneous detection writes into a fixed array of this size;
/// extras stay `CoreClass::P` (the safe default for under-counting).
pub const MAX_CORES: usize = 256; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121

/// Classify each logical processor by performance/efficiency class.
///
/// `total_cores` is the count of logical processors the caller
/// expects to populate; cores beyond `total_cores` stay at the
/// `Default` (`P`). All detection paths fall through to all-P on
/// failure.
pub fn classify_cores(total_cores: USize) -> [CoreClass; MAX_CORES] {
    let mut classes = [CoreClass::P; MAX_CORES];
    let count = core::cmp::min(total_cores.0, MAX_CORES);
    detect_into(&mut classes[..count]);
    classes
}

#[cfg(all(target_os = "linux", feature = "platform-os"))]
fn detect_into(classes: &mut [CoreClass]) {
    // Linux sysfs path: each core's `topology/cluster_id` partitions
    // by cluster; `cpufreq/cpuinfo_max_freq` ranks clusters. The
    // highest-frequency cluster is P. Reading sysfs requires libc
    // open/read/close (no_std-friendly under `feature = "platform-os"`).
    //
    // Implementation deferred to a follow-up commit: the sysfs probe
    // needs a small stack buffer + atoi parser; landing the path
    // selection now keeps the build green while the impl follows.
    let _ = classes;
}

#[cfg(all(target_os = "macos", feature = "platform-os"))]
fn detect_into(classes: &mut [CoreClass]) {
    // macOS path: `sysctlbyname("hw.perflevel0.physicalcpu")`
    // reports the number of P-cores. Logical processors before this
    // count are P; the rest E. Apple's perflevels are ordered:
    // perflevel0 = highest performance, perflevel1 = efficiency.
    //
    // Implementation deferred to a follow-up commit; sysctl FFI
    // wrappers and the perflevel parsing land alongside the parking
    // ulock wrappers (same FFI surface area).
    let _ = classes;
}

#[cfg(all(target_os = "windows", feature = "platform-os"))]
fn detect_into(classes: &mut [CoreClass]) {
    // Windows path: `GetSystemCpuSetInformation` exposes
    // `EfficiencyClass` per logical processor. EfficiencyClass 0
    // is the lowest-performance class; higher values are P.
    //
    // Implementation deferred: Windows FFI surface lands alongside
    // the `WaitOnAddress` parking wrapper.
    let _ = classes;
}

#[cfg(not(any(
    all(target_os = "linux", feature = "platform-os"),
    all(target_os = "macos", feature = "platform-os"),
    all(target_os = "windows", feature = "platform-os"),
)))]
fn detect_into(classes: &mut [CoreClass]) {
    // Stub fallback: every core stays `CoreClass::P`. The pool
    // operates correctly under this assumption; the adapt
    // subsystem's heterogeneous-core branch becomes a no-op.
    let _ = classes;
}
