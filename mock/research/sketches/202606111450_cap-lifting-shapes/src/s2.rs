//! s2: bare usize const generics threaded from the Cfg bound via free
//! const fns, no field access in the generic constant.
//!
//! The documented s0 wall is the `.0` FIELD ACCESS inside the anon
//! const, not the associated const itself. arvo's cap_size precedent
//! (`CoreProgram<{ cap_size(PC::CAP) }, ..>` in plan/core_program.rs)
//! shows GCE accepts a CALL whose argument is an associated const; the
//! field access moves into the const fn body, which is ordinary const
//! code. Two forms:
//!
//! - form i: non-generic const fn, associated-const argument
//!   (`usize_raw(Cfg::MAX_PLAN_AFFECTING_RESOURCES)`), the exact
//!   cap_size shape.
//! - form ii: generic const fn, no argument
//!   (`plan_res_of::<Cfg>()`).
//!
//! Both against the real RunCfg. The threading cost to watch: the
//! `where [(); ...]:` bound must repeat on the struct, every impl
//! block, and every free fn naming the type.

use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, Ordering};

use arvo::USize;
use hilavitkutin_api::{DefaultRunCfg, RunCfg};

/// The cap_size-shaped projection: USize to usize, no field access in
/// the anon const at the use site.
pub const fn usize_raw(u: USize) -> usize {
    u.0
}

/// Form ii: generic const fn reading the Cfg associated const.
pub const fn plan_res_of<Cfg: RunCfg>() -> usize {
    Cfg::MAX_PLAN_AFFECTING_RESOURCES.0
}

// ------------------------------------------------------------- form i

pub struct SchedI<Cfg: RunCfg>
where
    [(); usize_raw(Cfg::MAX_PLAN_AFFECTING_RESOURCES)]:,
{
    plan_dirty: [AtomicBool; usize_raw(Cfg::MAX_PLAN_AFFECTING_RESOURCES)],
    _cfg: PhantomData<Cfg>,
}

impl<Cfg: RunCfg> SchedI<Cfg>
where
    [(); usize_raw(Cfg::MAX_PLAN_AFFECTING_RESOURCES)]:,
{
    pub fn new() -> Self {
        Self {
            // repeat-expr with const block over a generic length
            plan_dirty: [const { AtomicBool::new(false) };
                usize_raw(Cfg::MAX_PLAN_AFFECTING_RESOURCES)],
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

/// A free fn naming the type: the third kind of site that must carry
/// the where bound.
pub fn width_of<Cfg: RunCfg>(s: &SchedI<Cfg>) -> usize
where
    [(); usize_raw(Cfg::MAX_PLAN_AFFECTING_RESOURCES)]:,
{
    s.width()
}

// ------------------------------------------------------------ form ii

pub struct SchedII<Cfg: RunCfg>
where
    [(); plan_res_of::<Cfg>()]:,
{
    plan_dirty: [AtomicBool; plan_res_of::<Cfg>()],
    _cfg: PhantomData<Cfg>,
}

impl<Cfg: RunCfg> SchedII<Cfg>
where
    [(); plan_res_of::<Cfg>()]:,
{
    pub fn new() -> Self {
        Self {
            plan_dirty: [const { AtomicBool::new(false) }; plan_res_of::<Cfg>()],
            _cfg: PhantomData,
        }
    }

    pub fn width(&self) -> usize {
        self.plan_dirty.len()
    }
}

pub fn run() {
    let a: SchedI<DefaultRunCfg> = SchedI::new();
    a.mark_dirty(3);
    a.mark_dirty(255);
    let b: SchedII<DefaultRunCfg> = SchedII::new();
    println!(
        "s2: form i width={} dirty={} | form ii width={}",
        width_of(&a),
        a.dirty_count(),
        b.width()
    );
    assert_eq!(a.width(), 256);
    assert_eq!(b.width(), 256);
    println!("s2: WORKS");
}
