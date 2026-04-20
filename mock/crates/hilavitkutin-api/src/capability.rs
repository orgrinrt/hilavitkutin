//! Capability atoms for the consumer contract surface.
//!
//! Small, composable traits for "receive items" and "report progress"
//! capabilities. Named composites in `sink` bound these atoms; codec
//! traits in `codec` consume them. A sink implementor picks the atoms
//! its storage supports; API signatures pick the atoms they need.

use arvo::{Bool, USize};
use notko::Outcome;

/// Receive one item by value.
///
/// Infallible; overflow is the implementor's problem. Sinks that
/// refuse on full conditions additionally implement `BoundedPush<T>`.
pub trait Push<T> {
    /// Accept `item` for storage / forwarding / counting / discard
    /// at the implementor's discretion.
    fn push(&mut self, item: T);
}

/// Receive a slice of `Copy` items.
///
/// Default implementation pushes per-item in order. Implementors
/// with a bulk-optimised path (byte emitters with memcpy, SIMD
/// writes) should override. Requires `Push<T>` because bulk push is
/// meaningless without per-item semantics.
pub trait BulkPush<T>: Push<T> {
    /// Accept `items` as a contiguous slice.
    fn push_bulk(&mut self, items: &[T])
    where
        T: Copy,
    {
        for item in items {
            self.push(*item);
        }
    }
}

/// Report item count.
///
/// Consumers branch on "did we emit anything" via `is_empty`.
pub trait Len {
    /// Current item count.
    fn len(&self) -> USize;

    /// `Bool::TRUE` when no items have been received.
    fn is_empty(&self) -> Bool {
        Bool::from(*self.len() == 0)
    }
}

/// Report total and remaining capacity.
///
/// Separated from `Len` so a sink may expose one without the other
/// (a counting sink has length but no capacity; a bounded ring
/// buffer has both).
pub trait Capacity {
    /// Total capacity in items.
    fn capacity(&self) -> USize;

    /// Free items until the sink refuses.
    fn remaining(&self) -> USize;
}

/// Overflow-aware push.
///
/// Implementors MUST NOT silently drop items on refusal; `try_push`
/// returns `Outcome::Err(Full)` instead. `Capacity` supertrait lets
/// callers introspect headroom before pushing.
pub trait BoundedPush<T>: Push<T> + Capacity {
    /// Attempt a push; `Outcome::Err(Full)` on refusal.
    fn try_push(&mut self, item: T) -> Outcome<(), Full>;
}

/// Refusal marker returned by `BoundedPush::try_push`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Full;
