//! The value-carrying WorkUnit list retained by the scheduler builder.
//!
//! A registered WorkUnit's TYPE accumulates in the builder's `Wus`
//! typestate (it drives the `AccessSet` membership proof). Its VALUE
//! accumulates here, on a parallel cons-list the engine's run walk
//! consumes: `WuNil` terminates, `WuCons` carries a unit instance plus
//! the tail. The builder prepends one node per registered WorkUnit, so
//! the unit's runtime value (including any `.with`-time configuration it
//! holds in its fields) survives into the built scheduler. The builder
//! appends one node per registered WorkUnit (via `WuAppend`), so the
//! carrier order equals registration order.
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

/// Append a unit value onto the end of the carrier.
///
/// The builder routes each registered WorkUnit through this so the carrier
/// grows in registration order (the head stays the first-registered unit and
/// the new unit lands at the tail), making the carrier type order equal the
/// registration order. Recursive: `WuNil` becomes a one-cell list; `WuCons`
/// keeps its head and appends onto its tail.
pub trait WuAppend<P> {
    /// The carrier type with `P` appended at the tail.
    type Out;

    /// Append `p` at the tail, preserving the existing head-to-tail order.
    fn append(self, p: P) -> Self::Out;
}

impl<P> WuAppend<P> for WuNil {
    type Out = WuCons<P, WuNil>;

    #[inline]
    fn append(self, p: P) -> Self::Out {
        WuCons { head: p, tail: WuNil }
    }
}

impl<P, H, T> WuAppend<P> for WuCons<H, T>
where
    T: WuAppend<P>,
{
    type Out = WuCons<H, <T as WuAppend<P>>::Out>;

    #[inline]
    fn append(self, p: P) -> Self::Out {
        WuCons { head: self.head, tail: self.tail.append(p) }
    }
}
