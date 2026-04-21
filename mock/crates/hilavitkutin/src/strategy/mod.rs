//! Strategy selection (domain 21).
//!
//! Plan-time selection based on record count + pipeline shape.

use arvo::USize;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Strategy {
    Sequential,
    Adaptive,
    PipeChase,
    Phased,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PhaseStrategy {
    MaxFuse,
    Balanced,
    MaxSplit,
}

/// Plan-time strategy selector.
pub trait StrategySelector {
    fn select(&self, record_count: USize, depth: USize, fibers: USize, roots: USize) -> Strategy;
}

/// Default selector per DESIGN thresholds.
pub struct DefaultSelector;

impl StrategySelector for DefaultSelector {
    fn select(&self, record_count: USize, depth: USize, fibers: USize, roots: USize) -> Strategy {
        // <10K records → Sequential.
        if *record_count < 10_000 {
            return Strategy::Sequential;
        }
        // Deep (depth > fibers/2, roots ≤ 2) → Sequential.
        if *depth > *fibers / 2 && *roots <= 2 {
            return Strategy::Sequential;
        }
        // Wide (roots > depth/2) → Adaptive / PipeChase.
        if *roots > *depth / 2 {
            return Strategy::Adaptive;
        }
        // Mixed → Phased.
        Strategy::Phased
    }
}
