//! Per-process arm state, built once and reused across calls.
//!
//! The harness calls an arm many times in one worker process with the same
//! input. Building the data, the scheduler and the worker pool is setup, not
//! the thing measured, and the engine's design is to schedule once and reuse
//! across frames, so the state lives here between calls. One slot per thread:
//! a new key drops the old state first (joining its workers) before the new
//! one is built, so two large states never coexist.

use std::any::Any;
use std::cell::RefCell;

/// What identifies an arm's state: everything its construction reads.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Key {
    pub arm:     &'static str,
    pub seed:    u64,
    pub kind:    u32,
    pub workers: u32,
    pub n:       usize,
}

struct Slot {
    key:   Option<Key>,
    state: Option<Box<dyn Any>>,
}

thread_local! {
    static SLOT: RefCell<Slot> = const { RefCell::new(Slot { key: None, state: None }) };
}

/// Run `body` on the state for `key`, building it with `make` when the slot
/// holds another key or another type.
pub fn with<T: 'static, R>(
    key: Key,
    make: impl FnOnce() -> T,
    body: impl FnOnce(&mut T) -> R,
) -> R {
    SLOT.with(|slot| {
        let mut slot = slot.borrow_mut();
        let fresh = slot.key != Some(key) || !slot.state.as_ref().is_some_and(|s| s.is::<T>());
        if fresh {
            // Drop before building: the old state's workers join here.
            slot.state = None;
            slot.key = None;
            slot.state = Some(Box::new(make()));
            slot.key = Some(key);
        }
        let state = slot
            .state
            .as_mut()
            .and_then(|s| s.downcast_mut::<T>())
            .expect("the slot was just filled with this type");
        body(state)
    })
}

/// Drop the state this thread holds, joining any workers it owns.
pub fn clear() {
    SLOT.with(|slot| {
        let mut slot = slot.borrow_mut();
        slot.state = None;
        slot.key = None;
    });
}
