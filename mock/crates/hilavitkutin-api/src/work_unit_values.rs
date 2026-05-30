//! The value-carrying WorkUnit list retained by the scheduler builder.
//!
//! A registered WorkUnit's TYPE accumulates in the builder's `Wus`
//! typestate (it drives the `AccessSet` membership proof). Its VALUE
//! accumulates here, on a parallel cons-list the engine's run walk
//! consumes: `WuNil` terminates, `WuCons` carries a unit instance plus
//! the tail. The builder prepends one node per registered WorkUnit, so
//! the unit's runtime value (including any `.with`-time configuration it
//! holds in its fields) survives into the built scheduler.
//!
//! These types live in the api crate (not the engine) so the builder
//! routing in `store_values` can construct them. The engine re-exports
//! them next to `RunFiber` / `run_fiber_walk`, which consume the list;
//! the walk trait itself stays in the engine because it references
//! engine projection types.
//!
//! The cell convention follows `PtrCons` / `PtrNil` (head value plus
//! tail), the same shape the engine's pointer lists use.

/// Terminator for a fiber's value-carrying WorkUnit sequence.
pub struct WuNil;

/// Cons cell: a WorkUnit instance at this position plus the tail.
pub struct WuCons<W, Tail> {
    /// The unit at this position in the sequence.
    pub head: W,
    /// The remaining units.
    pub tail: Tail,
}
