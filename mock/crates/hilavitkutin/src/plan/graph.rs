//! Dependency graph: adjacency between WU nodes (domains 11, 15).
//!
//! Skeleton uses a dense `[[Bool; N]; N]` matrix. Swap for arvo-
//! graph CSR once arvo-graph gains const-generic support (BACKLOG).
//! Surface API (`has_edge` / `add_edge`) is stable across the swap.

use arvo::USize;
use arvo::newtype::Bool;
use core::fmt;

/// Dense adjacency matrix over `MAX_UNITS` nodes.
#[derive(Copy, Clone)]
pub struct DependencyGraph<const MAX_UNITS: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    edges: [[Bool; MAX_UNITS]; MAX_UNITS],
}

impl<const MAX_UNITS: usize> DependencyGraph<MAX_UNITS> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    /// Empty graph (no edges).
    pub const fn new() -> Self {
        Self {
            edges: [[Bool::FALSE; MAX_UNITS]; MAX_UNITS],
        }
    }

    /// True iff there's an edge `from → to`. False if either
    /// index is out of range.
    pub const fn has_edge(&self, from: USize, to: USize) -> Bool {
        let f = from.0;
        let t = to.0;
        if f >= MAX_UNITS || t >= MAX_UNITS {
            return Bool::FALSE;
        }
        self.edges[f][t]
    }

    /// Set edge `from → to`. No-op if either index is out of range.
    pub fn add_edge(&mut self, from: USize, to: USize) {
        let f = from.0;
        let t = to.0;
        if f >= MAX_UNITS || t >= MAX_UNITS {
            return;
        }
        self.edges[f][t] = Bool::TRUE;
    }
}

impl<const MAX_UNITS: usize> Default for DependencyGraph<MAX_UNITS> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    fn default() -> Self {
        Self::new()
    }
}

impl<const MAX_UNITS: usize> fmt::Debug for DependencyGraph<MAX_UNITS> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DependencyGraph")
            .field("size", &MAX_UNITS)
            .finish()
    }
}

impl<const MAX_UNITS: usize> PartialEq for DependencyGraph<MAX_UNITS> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    fn eq(&self, other: &Self) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: PartialEq trait impl requires bool return; tracked: #72
        let mut i = 0;
        while i < MAX_UNITS {
            let mut j = 0;
            while j < MAX_UNITS {
                if self.edges[i][j].0 != other.edges[i][j].0 {
                    return false;
                }
                j += 1;
            }
            i += 1;
        }
        true
    }
}

impl<const MAX_UNITS: usize> Eq for DependencyGraph<MAX_UNITS> {} // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
