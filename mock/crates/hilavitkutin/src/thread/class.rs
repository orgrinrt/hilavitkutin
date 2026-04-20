//! Core class: heterogeneous-core awareness (domain 20).
//!
//! P-cores get critical-path trunks + larger morsels. E-cores
//! get branches/leaves + proportionally smaller ranges. Runtime
//! detection is a follow-up (CPUID leaf 0x1A / sysfs / IOKit —
//! see BACKLOG).

/// Heterogeneous-core class.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CoreClass {
    /// Performance core — critical-path trunks + larger morsels.
    P,
    /// Efficiency core — branches/leaves + smaller morsels.
    E,
}

impl Default for CoreClass {
    fn default() -> Self {
        Self::P
    }
}
