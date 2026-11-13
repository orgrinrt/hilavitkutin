//! Spike s4, seat 02: growth against the trunk's storage contract and its os-tier
//! memory provider. The checks are in `tests/growth.rs`.

/// Resident set size of this process in kibibytes, from `/proc/self/status`.
pub fn rss_kib() -> u64 {
    let s = std::fs::read_to_string("/proc/self/status").expect("procfs");
    for line in s.lines() {
        if let Some(rest) = line.strip_prefix("VmRSS:") {
            return rest.trim().trim_end_matches("kB").trim().parse().expect("kB");
        }
    }
    panic!("no VmRSS line")
}

pub fn overcommit_mode() -> String {
    std::fs::read_to_string("/proc/sys/vm/overcommit_memory").unwrap_or_else(|_| "unknown".into()).trim().to_string()
}
