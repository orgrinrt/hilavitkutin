//! The close-of-frame walk that finds the first accumulator to refuse an append.

use arvo::USize;
use hilavitkutin_api::AccumRefused;
use notko::Maybe;
use notko::lend::Exhausted;

use super::{AccumBinding, BindingNil, ColKind, ColumnBinding, ResourceBinding, VirtualBinding};

/// Find the first accumulator, in registration order, that refused an append.
///
/// An `AccumBinding` has refused this frame exactly when its `wanted` count is
/// above its live length. The store list is built by prepending, so the head
/// is the last registered store and registration order is tail first: each
/// node asks its tail before looking at itself. `seen` counts the accumulators
/// already passed in registration order and becomes the refusing
/// accumulator's position in the returned `AccumRefused`.
///
/// Read at the close, after the last band, on the thread that owns the
/// bindings for the frame; on `run_parallel` that is the main thread with every
/// worker parked, so no cell moves under the walk. For an accumulator-free
/// carrier the walk visits no `AccumBinding` and returns `Isnt`.
pub trait FirstRefusal {
    /// The first refusing accumulator in registration order, if any.
    fn first_refusal(&self, seen: &mut USize) -> Maybe<AccumRefused>;
}

impl FirstRefusal for BindingNil {
    #[inline]
    fn first_refusal(&self, _seen: &mut USize) -> Maybe<AccumRefused> {
        Maybe::Isnt
    }
}

impl<T, Tail: FirstRefusal> FirstRefusal for ResourceBinding<T, Tail> {
    #[inline]
    fn first_refusal(&self, seen: &mut USize) -> Maybe<AccumRefused> {
        self.tail.first_refusal(seen)
    }
}

impl<T, Tail: FirstRefusal, K: ColKind> FirstRefusal for ColumnBinding<T, Tail, K> {
    #[inline]
    fn first_refusal(&self, seen: &mut USize) -> Maybe<AccumRefused> {
        self.tail.first_refusal(seen)
    }
}

impl<T, Tail: FirstRefusal> FirstRefusal for VirtualBinding<T, Tail> {
    #[inline]
    fn first_refusal(&self, seen: &mut USize) -> Maybe<AccumRefused> {
        self.tail.first_refusal(seen)
    }
}

impl<T, Tail: FirstRefusal> FirstRefusal for AccumBinding<T, Tail> {
    #[inline]
    fn first_refusal(&self, seen: &mut USize) -> Maybe<AccumRefused> {
        let earlier = self.tail.first_refusal(seen);
        let at = *seen;
        seen.0 += 1; // lint:allow(no-bare-numeric) reason: accumulator position step; tracked: #121
        match earlier {
            Maybe::Is(r) => Maybe::Is(r),
            Maybe::Isnt => {
                let wanted = self.wanted.get();
                if wanted.0 > self.len.get().0 {
                    Maybe::Is(AccumRefused::new(at, Exhausted {
                        wanted: wanted.0,
                        had:    self.cap.0,
                    }))
                } else {
                    Maybe::Isnt
                }
            },
        }
    }
}
