//! How many walkers each arm's fold runs, observed rather than declared.
//!
//! A test binary of its own, with one test in it, because it counts the
//! process's threads: another test running beside it would add its own.
//! The count is read from `/proc/self/task`, so it is built on Linux only.
#![cfg(target_os = "linux")]

use std::collections::HashSet;
use std::sync::Mutex;
use std::thread::ThreadId;

use stc_support::arms::{
    Arm,
    HeadTail,
    HeadTailDynamic,
    KernelNway,
    KernelSeq,
    Nway,
    Sequential,
    StdParallel,
};
use stc_support::data;
use stc_support::engine::Op;
use stc_support::kernel::Kernel;
use stc_support::split::morsel;

/// The threads that applied [`Counting`] since the set was last cleared.
static APPLIED: Mutex<Option<HashSet<ThreadId>>> = Mutex::new(None);

/// A kernel that records which thread applied it.
#[derive(Copy, Clone, Debug, Default)]
struct Counting;

impl Kernel for Counting {
    fn apply(x: u32) -> u32 {
        APPLIED
            .lock()
            .unwrap()
            .get_or_insert_with(HashSet::new)
            .insert(std::thread::current().id());
        x
    }
}

fn threads() -> usize {
    std::fs::read_dir("/proc/self/task").unwrap().count()
}

/// What one arm's fold at `w` workers does: the threads building it spawned,
/// and the most distinct threads that walked records in one frame, over a
/// few frames so the floating meeting point's second walker gets a claim in.
/// Per frame, since `std_parallel` spawns its threads anew every frame.
fn observe<A: Arm>(w: usize) -> (usize, usize) {
    let n = morsel() * 512 + 37;
    let xs = data::column(3, n);
    let before = threads();
    let mut fold = A::fold::<Counting>(xs, w);
    let spawned = threads() - before;
    let mut walked = 0;
    for _ in 0 .. 16 {
        *APPLIED.lock().unwrap() = Some(HashSet::new());
        fold(Op::Frame);
        walked = walked.max(APPLIED.lock().unwrap().as_ref().map_or(0, HashSet::len));
    }
    drop(fold);
    (spawned, walked)
}

/// Head-and-tail runs two walkers at any W from 2 up and one at W = 1, the
/// N-way arms run W, the one-walker arms one. The arms on the crew are also
/// held to the threads they park, since a crew built at W for a split that
/// walks two would wake idle workers every frame without walking a record
/// more. `Arm::walkers` is held to both.
#[test]
fn every_arm_runs_the_walkers_it_claims() {
    fn on_crew<A: Arm>(w: usize, want: usize) {
        let (spawned, walked) = observe::<A>(w);
        assert_eq!(
            walked,
            want,
            "{} walked with other than {want} at W={w}",
            A::NAME
        );
        assert_eq!(
            spawned,
            want,
            "{} parked other than {want} workers at W={w}",
            A::NAME
        );
        assert_eq!(
            A::walkers(w),
            want,
            "{} declares other than it runs at W={w}",
            A::NAME
        );
    }
    fn walking<A: Arm>(w: usize, want: usize) {
        let (_, walked) = observe::<A>(w);
        assert_eq!(
            walked,
            want,
            "{} walked with other than {want} at W={w}",
            A::NAME
        );
        assert_eq!(
            A::walkers(w),
            want,
            "{} declares other than it runs at W={w}",
            A::NAME
        );
    }
    for w in [1, 2, 3, 4] {
        on_crew::<HeadTail>(w, w.min(2));
        on_crew::<HeadTailDynamic>(w, w.min(2));
        on_crew::<KernelNway>(w, w);
        on_crew::<KernelSeq>(w, 1);
        walking::<Nway>(w, w);
        walking::<StdParallel>(w, w);
        walking::<Sequential>(w, 1);
    }
}
