//! Column classification: per-fiber column role (domain 15).
//!
//! Per-fiber column classification determines the codegen shape:
//! internal columns live register-to-register, input columns come
//! from the preceding fiber's arena, output columns spill to the
//! store-buffer-friendly tail of the dispatch function.

/// How a column is used by a given fiber.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ColumnClassification {
    /// Fiber-local; register-to-register (dead-store eliminated).
    Internal,
    /// Loaded from upstream at fiber start.
    Input,
    /// Written at fiber end; flows to downstream fibers.
    Output,
}

impl Default for ColumnClassification {
    fn default() -> Self {
        Self::Internal
    }
}
