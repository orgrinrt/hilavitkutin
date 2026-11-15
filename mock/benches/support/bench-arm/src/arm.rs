//! One arm per worker process, built on its first call and reused.

use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::mem::ManuallyDrop;

use mockspace_bench_core::FfiBenchCall;

/// One arm of a bench: state plus the three things a call does with it.
///
/// Every arm starts a call with the same input in place and nothing else in
/// common. The arms keep different working sets between calls, an engine arm
/// its columns in an arena and a std arm whatever buffers it holds, and a call
/// starts on whatever the last one left. The input is the one part of the
/// cache state that is the same across arms.
pub trait Arm<O> {
    /// Put the input records in place. Untimed.
    fn load(&mut self, records: &[u32]);
    /// Run once. This is the measured region.
    fn frame(&mut self);
    /// Digest what the run wrote. Untimed.
    fn digest(&mut self) -> O;
}

/// An `Arm` built from a state value and three closures over it.
///
/// An engine arm's state is a `Scheduler` whose type cannot be written down: it
/// is the output of a builder call sequence. Holding it in a generic field and
/// erasing the whole thing behind `Box<dyn Arm<O>>` is what lets the engine
/// arms and the plain arms share one shape.
pub struct StateArm<S, L, F, D> {
    state:  S,
    load:   L,
    frame:  F,
    digest: D,
}

impl<S, L, F, D> StateArm<S, L, F, D> {
    /// Assemble an arm. The bounds are here as well as on the `Arm` impl so the
    /// closures' parameter types are inferred at the call site.
    pub fn new<O>(state: S, load: L, frame: F, digest: D) -> Self
    where
        L: FnMut(&mut S, &[u32]),
        F: FnMut(&mut S),
        D: FnMut(&mut S) -> O,
    {
        Self {
            state,
            load,
            frame,
            digest,
        }
    }
}

impl<S, L, F, D, O> Arm<O> for StateArm<S, L, F, D>
where
    L: FnMut(&mut S, &[u32]),
    F: FnMut(&mut S),
    D: FnMut(&mut S) -> O,
{
    fn load(&mut self, records: &[u32]) {
        (self.load)(&mut self.state, records)
    }

    fn frame(&mut self) {
        (self.frame)(&mut self.state)
    }

    fn digest(&mut self) -> O {
        (self.digest)(&mut self.state)
    }
}

thread_local! {
    /// The per-process arm and the key it was built for.
    ///
    /// The framework sanctions this: each worker is a dedicated subprocess per
    /// variant, mode and pass, and at least one warmup call always runs before
    /// any timed batch, so a built scheduler lands in untimed territory. It is
    /// a `thread_local` rather than a `static` because a `Scheduler` is
    /// neither `Sync` nor `Unpin`, and the entry point is only ever called on
    /// the worker's own thread. It holds `dyn Any` because a `thread_local`
    /// cannot be generic, and `call` downcasts it to the key and digest types
    /// of the member asking.
    ///
    /// `ManuallyDrop` keeps the arm alive until the process exits. A parallel
    /// engine arm has worker threads parked on its scheduler, and a
    /// thread-local destructor running at exit would free the scheduler under
    /// them. The process reclaims the memory either way.
    static SLOT: ManuallyDrop<RefCell<Option<Box<dyn Any>>>> =
        const { ManuallyDrop::new(RefCell::new(None)) };
}

/// One call of a bench: the whole body of every variant.
///
/// The arm `build` makes is built on the first call of the process and reused.
/// The records are loaded and the digest is read outside the timed region; the
/// `run` block holds the arm's `frame` and nothing else.
///
/// The key is fixed for the life of a worker process: one dylib is one arm, the
/// worker is spawned with one `--n`, and whatever else the key holds comes from
/// the section that spawned it. A key that changed would mean dropping a
/// scheduler whose worker threads may still be parked on it, so a change is a
/// panic rather than a rebuild.
pub fn call<K, O>(
    key: K,
    records: &[u32],
    build: impl FnOnce() -> Box<dyn Arm<O>>,
    output: &mut O,
) -> FfiBenchCall
where
    K: Copy + PartialEq + Debug + 'static,
    O: 'static,
{
    SLOT.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.is_none() {
            let held: (K, Box<dyn Arm<O>>) = (key, build());
            *slot = Some(Box::new(held));
        }
        let (held_key, arm) = slot
            .as_mut()
            .expect("the arm was just placed")
            .downcast_mut::<(K, Box<dyn Arm<O>>)>()
            .expect("this process already holds an arm of another member's key or digest type");
        assert!(
            *held_key == key,
            "this process already holds the arm {held_key:?} and was asked for {key:?}. One \
             worker process runs one arm under one key."
        );
        let call = mockspace_bench_core::timed! {
            setup { arm.load(records); }
            run { arm.frame(); }
        };
        *output = arm.digest();
        call
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn summing(builds: std::rc::Rc<std::cell::Cell<u32>>) -> Box<dyn Arm<u64>> {
        builds.set(builds.get() + 1);
        Box::new(StateArm::new(
            Vec::<u32>::new(),
            |s: &mut Vec<u32>, r: &[u32]| {
                s.clear();
                s.extend_from_slice(r);
            },
            |s: &mut Vec<u32>| {
                for v in s.iter_mut() {
                    *v = v.wrapping_mul(3);
                }
            },
            |s: &mut Vec<u32>| s.iter().map(|v| u64::from(*v)).sum(),
        ))
    }

    #[test]
    fn the_arm_is_built_once_and_runs_every_call() {
        let builds = std::rc::Rc::new(std::cell::Cell::new(0));
        let mut out = 0u64;
        call(1u32, &[1, 2, 3], || summing(builds.clone()), &mut out);
        assert_eq!(out, 18);
        call(1u32, &[4, 5], || summing(builds.clone()), &mut out);
        assert_eq!(out, 27);
        assert_eq!(builds.get(), 1);
    }

    #[test]
    fn a_changed_key_is_refused() {
        let builds = std::rc::Rc::new(std::cell::Cell::new(0));
        let mut out = 0u64;
        call(1u32, &[1], || summing(builds.clone()), &mut out);
        let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            call(2u32, &[1], || summing(builds.clone()), &mut out)
        }));
        assert!(refused.is_err());
    }
}
