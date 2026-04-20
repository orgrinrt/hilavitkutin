//! Dependency graph: adjacency between WU nodes (domains 11, 15).
//!
//! Skeleton uses a dense `[[bool; N]; N]` matrix. Swap for arvo-
//! graph CSR once arvo-graph gains const-generic support (BACKLOG).
//! Surface API (`has_edge` / `add_edge`) is stable across the swap.

use core::fmt;

/// Dense adjacency matrix over `MAX_UNITS` nodes.
#[derive(Copy, Clone)]
pub struct DependencyGraph<const MAX_UNITS: usize> {
    edges: [[bool; MAX_UNITS]; MAX_UNITS],
}

impl<const MAX_UNITS: usize> DependencyGraph<MAX_UNITS> {
    /// Empty graph (no edges).
    pub const fn new() -> Self {
        Self {
            edges: [[false; MAX_UNITS]; MAX_UNITS],
        }
    }

    /// True iff there's an edge `from → to`. False if either
    /// index is out of range.
    pub const fn has_edge(&self, from: u32, to: u32) -> bool {
        let f = from as usize;
        let t = to as usize;
        if f >= MAX_UNITS || t >= MAX_UNITS {
            return false;
        }
        self.edges[f][t]
    }

    /// Set edge `from → to`. No-op if either index is out of range.
    pub fn add_edge(&mut self, from: u32, to: u32) {
        let f = from as usize;
        let t = to as usize;
        if f >= MAX_UNITS || t >= MAX_UNITS {
            return;
        }
        self.edges[f][t] = true;
    }
}

impl<const MAX_UNITS: usize> Default for DependencyGraph<MAX_UNITS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const MAX_UNITS: usize> fmt::Debug for DependencyGraph<MAX_UNITS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DependencyGraph")
            .field("size", &MAX_UNITS)
            .finish()
    }
}

impl<const MAX_UNITS: usize> PartialEq for DependencyGraph<MAX_UNITS> {
    fn eq(&self, other: &Self) -> bool {
        let mut i = 0;
        while i < MAX_UNITS {
            let mut j = 0;
            while j < MAX_UNITS {
                if self.edges[i][j] != other.edges[i][j] {
                    return false;
                }
                j += 1;
            }
            i += 1;
        }
        true
    }
}

impl<const MAX_UNITS: usize> Eq for DependencyGraph<MAX_UNITS> {}
