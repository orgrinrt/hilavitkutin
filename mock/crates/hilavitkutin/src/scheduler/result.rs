//! Per-fiber pipeline execution result (R7).

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PipelineResult {
    /// Fiber completed all assigned work successfully.
    Completed,
    /// Fiber encountered an error during execution.
    Failed,
    /// Fiber's dependency failed and dependents were poisoned.
    Poisoned,
}
