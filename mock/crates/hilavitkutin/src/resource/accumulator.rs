//! Per-thread convergence accumulators.
//!
//! Head+tail convergence: each thread gets its own stack-local
//! accumulator; merged after convergence via a fn-pointer combiner
//! (addition for additive, sequential fallback for non-commutative).

use arvo::USize;

#[derive(Copy, Clone)]
pub struct AccumulatorSlot<T: Copy> {
    pub value: T,
}

impl<T: Copy> AccumulatorSlot<T> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

/// Const-sized per-thread accumulator buffer.
#[derive(Copy, Clone)]
pub struct ConvergenceBuffer<T: Copy, const N: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    slots: [AccumulatorSlot<T>; N],
}

impl<T: Copy, const N: usize> ConvergenceBuffer<T, N> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    #[inline(always)]
    pub const fn new(zero: T) -> Self {
        Self {
            slots: [AccumulatorSlot { value: zero }; N],
        }
    }

    #[inline(always)]
    pub fn get(&self, i: USize) -> T {
        self.slots[*i].value
    }

    #[inline(always)]
    pub fn set(&mut self, i: USize, v: T) {
        self.slots[*i].value = v;
    }

    /// Merge all slots through a fn-pointer combiner.
    ///
    /// Closure-combiner support lands with 5a4 once the thread-pool
    /// story ships generic spawn.
    #[inline]
    pub fn combine(&self, init: T, combine: fn(T, T) -> T) -> T {
        let mut acc = init;
        let mut i = 0;
        while i < N {
            acc = combine(acc, self.slots[i].value);
            i += 1;
        }
        acc
    }
}
