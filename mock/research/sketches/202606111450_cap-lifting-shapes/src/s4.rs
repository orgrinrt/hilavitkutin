//! s4: associated-const indirection through a helper trait whose const
//! is already a bare usize.
//!
//! The field access moves into the helper trait's const DEFINITION body
//! (ordinary const code); the anon const at the array-length use site
//! is then a plain associated-const path, no call, no field access. A
//! blanket impl over RunCfg means one definition site covers every
//! consumer Cfg.

use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, Ordering};

use hilavitkutin_api::{DefaultRunCfg, RunCfg};

/// Bare-usize projection of the RunCfg caps. The `.0` happens here, in
/// const-definition position, where field access is ordinary const
/// evaluation.
pub trait CapsUsize {
    const PLAN_DIRTY: usize;
}

impl<C: RunCfg> CapsUsize for C {
    const PLAN_DIRTY: usize = C::MAX_PLAN_AFFECTING_RESOURCES.0;
}

pub struct Sched4<Cfg: RunCfg>
where
    [(); <Cfg as CapsUsize>::PLAN_DIRTY]:,
{
    plan_dirty: [AtomicBool; <Cfg as CapsUsize>::PLAN_DIRTY],
    _cfg: PhantomData<Cfg>,
}

impl<Cfg: RunCfg> Sched4<Cfg>
where
    [(); <Cfg as CapsUsize>::PLAN_DIRTY]:,
{
    pub fn new() -> Self {
        Self {
            plan_dirty: [const { AtomicBool::new(false) }; <Cfg as CapsUsize>::PLAN_DIRTY],
            _cfg: PhantomData,
        }
    }

    pub fn mark_dirty(&self, i: usize) {
        self.plan_dirty[i].store(true, Ordering::Relaxed);
    }

    pub fn dirty_count(&self) -> usize {
        self.plan_dirty
            .iter()
            .filter(|b| b.load(Ordering::Relaxed))
            .count()
    }

    pub fn width(&self) -> usize {
        self.plan_dirty.len()
    }
}

/// The free-fn site, carrying the same where bound.
pub fn width_of<Cfg: RunCfg>(s: &Sched4<Cfg>) -> usize
where
    [(); <Cfg as CapsUsize>::PLAN_DIRTY]:,
{
    s.width()
}

pub fn run() {
    let s: Sched4<DefaultRunCfg> = Sched4::new();
    s.mark_dirty(0);
    s.mark_dirty(128);
    println!("s4: width={} dirty={}", width_of(&s), s.dirty_count());
    assert_eq!(s.width(), 256);
    println!("s4: WORKS");
}
