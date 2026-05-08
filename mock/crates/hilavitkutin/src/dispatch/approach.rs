//! Dispatch approach enum (domain 17).
//!
//! Which codegen approach the plan picked. Selected by record
//! count + fiber count: small record counts (<10K target) get
//! `IndirectPerFiber` or `TrunkMega`; large record counts get
//! `ScheduleMega`. Exact thresholds are benchmarked: see BACKLOG
//! → `select_approach` follow-up.

/// Dispatch codegen approach.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum DispatchApproach {
    /// One indirect call per fiber per morsel. Lowest-overhead
    /// path for small record counts with many fibers.
    IndirectPerFiber,
    /// One fused mega-function per trunk. Picked when a trunk
    /// dominates the pipeline and record counts stay small.
    TrunkMega,
    /// One fused mega-function per schedule (whole pipeline).
    /// Picked for large record counts where indirect-call
    /// overhead matters less than LLVM's whole-pipeline
    /// optimisation window.
    ScheduleMega,
}
