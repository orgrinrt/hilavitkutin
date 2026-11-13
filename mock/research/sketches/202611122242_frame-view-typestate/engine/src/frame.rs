//! The one-thread reference runner: the frame, its step, and its close.

use crate::kernel::{CloseUnit, Kernel};
use crate::lane::Lane;
use crate::registry::{Pick, RawLanes, Registry};
use crate::view::Landed;

/// The frame: every lane of the registry, N rows each, committed and staged.
pub struct Frame<Reg: Registry, const N: usize> {
    committed: Reg::Bufs<N>,
    staged:    Reg::Bufs<N>,
}

impl<Reg: Registry, const N: usize> Frame<Reg, N> {
    const ROWS: u32 = {
        assert!(
            N <= u32::MAX as usize,
            "a frame holds at most u32::MAX rows"
        );
        N as u32
    };

    pub fn new() -> Self {
        Frame {
            committed: Reg::init(),
            staged:    Reg::init(),
        }
    }

    /// One step with one kernel.
    ///
    /// The `const` block is the check the position mask exists for: it is
    /// evaluated where this is instantiated, which is where the kernel is.
    pub fn step<K: Kernel<Reg, Ps>, Ps>(&mut self, _kernel: K) {
        const {
            assert!(
                K::MASK.count_ones() == K::LEN,
                "a kernel declares one staged lane twice"
            );
        }
        self.staged = self.committed;
        {
            let view = Reg::shared(&self.committed);
            let raw = RawLanes::new(&mut self.staged);
            // SAFETY: the const block proves the write list names each lane
            // once, and this kernel is the only holder of the raw lanes.
            unsafe { K::run_in(view, Self::ROWS, &raw) };
        }
        core::mem::swap(&mut self.committed, &mut self.staged);
    }

    /// One step with two kernels, both handed their staged handles from the
    /// same staging buffers.
    pub fn step2<A, B, Pa, Pb>(&mut self, _a: A, _b: B)
    where
        A: Kernel<Reg, Pa>,
        B: Kernel<Reg, Pb>,
    {
        const {
            assert!(
                A::MASK.count_ones() == A::LEN,
                "a kernel declares one staged lane twice"
            );
            assert!(
                B::MASK.count_ones() == B::LEN,
                "a kernel declares one staged lane twice"
            );
            assert!(
                A::MASK & B::MASK == 0,
                "two kernels of one step declare the same staged lane"
            );
        }
        self.staged = self.committed;
        {
            let view = Reg::shared(&self.committed);
            let raw = RawLanes::new(&mut self.staged);
            // SAFETY: the const block proves each write list names each lane
            // once and the two lists are disjoint, so the two kernels' staged
            // handles never reach one lane.
            unsafe {
                A::run_in(view, Self::ROWS, &raw);
                B::run_in(view, Self::ROWS, &raw);
            }
        }
        core::mem::swap(&mut self.committed, &mut self.staged);
    }

    /// The close, after a step has landed: the unit is handed the landed
    /// stores it names and nothing of the view.
    pub fn close<C: CloseUnit<Reg>>(&mut self, _unit: C) {
        C::close(Landed::new(Reg::excl(&mut self.committed), Self::ROWS));
    }

    /// The committed rows of one lane, for whoever owns the frame. A kernel
    /// never reaches this: it has the view.
    pub fn committed<'a, X, P>(&'a self, _lane: X) -> &'a [X::Item]
    where
        X: Lane,
        Reg::Shared<'a>: Pick<X, P, V = &'a [X::Item]>,
    {
        Reg::shared(&self.committed).pick()
    }
}

impl<Reg: Registry, const N: usize> Default for Frame<Reg, N> {
    fn default() -> Self {
        Self::new()
    }
}
