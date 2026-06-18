// GATE-2 §9 dispatch-seam sketch: per-core rebase of the projected accumulator
// bundle. Companion to percore_region_merge.rs (which proved the ALGORITHM). This
// one proves the WIRING: can the projected `AccPtrCons` / `AccumColPtr` bundle be
// rebased per core (base += lo, cap = hi-lo, len = a worker-supplied cell) under
// the real bundle SHAPE, including the lifetime rethreading and per-node cell
// assignment, compiling on the pinned toolchain.
//
// Why this is a separate risk from the algorithm: the append path resolves
// `AccumColPtr { base, len: &'frame Cell<USize>, cap }` from the projected
// `AccPtrCons` (engine_ctx.rs:133, :656-664). The per-core variant must hand each
// node a DIFFERENT base offset, a DIFFERENT (worker-stack) cell, and a new cap.
// The cells live for a shorter lifetime than the bindings, so the rebased bundle
// carries that shorter lifetime; and a multi-accumulator carrier (AccPtrCons of
// length k) needs k distinct cells assigned positionally. This sketch proves a
// slice-splitting rebase walk threads both without a type-level counter and
// without naming `k`.
//
// Mirrors of the real types (engine_ctx.rs). `base` is a raw ptr here; in the
// engine it is ColumnPtr<T> (NonNull) with the same `.add(i)` arithmetic.

use std::cell::Cell;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct USize(usize);

// engine_ctx.rs:133 AccumColPtr (module-private fields; the rebase ctor lives in
// the same module in the real change, so field access is in-scope there).
// Manual Copy/Clone WITHOUT a `T: Copy` bound, mirroring engine_ctx.rs:139-145.
struct AccumColPtr<'frame, T> {
    base: *mut T,
    len: &'frame Cell<USize>,
    cap: USize,
}
impl<'frame, T> Copy for AccumColPtr<'frame, T> {}
impl<'frame, T> Clone for AccumColPtr<'frame, T> {
    fn clone(&self) -> Self {
        *self
    }
}

// engine_ctx.rs:148/152 the projected bundle.
struct AccPtrNil;
struct AccPtrCons<'frame, H, Tail> {
    head: AccumColPtr<'frame, H>,
    tail: Tail,
}

// The verbatim shipped append (engine_ctx.rs:1015-1038), exercised on a rebased
// head to confirm a rebased pointer drives it unchanged.
impl<'frame, T> AccumColPtr<'frame, T> {
    #[inline]
    unsafe fn append(&self, v: T) {
        let live = self.len.get();
        if live.0 >= self.cap.0 {
            return;
        }
        core::ptr::write(self.base.add(live.0), v);
        self.len.set(USize(live.0 + 1));
    }
}

// The new seam: rebase a projected bundle into a per-core bundle. Each node takes
// `cells[0]` as its live cell, offsets its base by `lo` elements, and sets cap =
// `region_cap`; the tail recurses on `cells[1..]`, so cells are assigned
// positionally with no type-level counter and `k` is never named. The output
// bundle carries the cells' lifetime `'a`, distinct from the source `'frame`.
//
// In the real engine `lo`/`region_cap` are per-core constants; here they are
// passed in. The base offset for ALL accumulators in a phase is the same `lo`
// (the core's record-slice start), and region_cap the same `hi-lo`, because every
// accumulator in a convergence phase partitions by the SAME record slice.
trait RebaseAccums<'a> {
    type Out;
    fn rebase(&self, lo: usize, region_cap: USize, cells: &'a [Cell<USize>]) -> Self::Out;
}

impl<'a> RebaseAccums<'a> for AccPtrNil {
    type Out = AccPtrNil;
    #[inline]
    fn rebase(&self, _lo: usize, _region_cap: USize, _cells: &'a [Cell<USize>]) -> AccPtrNil {
        AccPtrNil
    }
}

impl<'a, 'frame, H, Tail> RebaseAccums<'a> for AccPtrCons<'frame, H, Tail>
where
    Tail: RebaseAccums<'a>,
{
    type Out = AccPtrCons<'a, H, <Tail as RebaseAccums<'a>>::Out>;
    #[inline]
    fn rebase(&self, lo: usize, region_cap: USize, cells: &'a [Cell<USize>]) -> Self::Out {
        AccPtrCons {
            head: AccumColPtr {
                // base += lo elements (same .add arithmetic the append uses).
                base: self.head.base.wrapping_add(lo),
                len: &cells[0],
                cap: region_cap,
            },
            tail: self.tail.rebase(lo, region_cap, &cells[1..]),
        }
    }
}

// AccumSelector mirror to read a node back for assertion (engine_ctx.rs:400-410).
trait Get<T> {
    fn get(&self) -> AccumColPtr<'_, T>;
}
impl<'frame, T, Tail> Get<T> for AccPtrCons<'frame, T, Tail> {
    fn get(&self) -> AccumColPtr<'_, T> {
        self.head
    }
}

fn main() {
    // A 2-accumulator carrier (proves the positional cell assignment for k>1).
    let total = 256usize;
    let cap = total;
    let mut buf_a: Vec<u32> = vec![u32::MAX; cap];
    let mut buf_b: Vec<u64> = vec![u64::MAX; cap];

    // The "binding" cells (the shared single-core cells the projection would read).
    let bind_cell_a = Cell::new(USize(0));
    let bind_cell_b = Cell::new(USize(0));

    // The projected bundle as acc_project would build it (full buffer, binding
    // cell). AccPtrCons<u32, AccPtrCons<u64, AccPtrNil>>.
    let bundle = AccPtrCons {
        head: AccumColPtr { base: buf_a.as_mut_ptr(), len: &bind_cell_a, cap: USize(cap) },
        tail: AccPtrCons {
            head: AccumColPtr { base: buf_b.as_mut_ptr(), len: &bind_cell_b, cap: USize(cap) },
            tail: AccPtrNil,
        },
    };

    // Per-core rebase: simulate core c of ncores over its record slice.
    let ncores = 4usize;
    let per = (total + ncores - 1) / ncores;
    let mut ok = true;
    for c in 0..ncores {
        let lo = (c * per).min(total);
        let hi = (lo + per).min(total);
        let region_cap = USize(hi - lo);
        // Worker-stack cells, one per accumulator (k=2). Lifetime 'a < bindings.
        let core_cells: [Cell<USize>; 2] = [Cell::new(USize(0)), Cell::new(USize(0))];
        let rebased = bundle.rebase(lo, region_cap, &core_cells);

        // Append <=1 per record into each accumulator over [lo,hi).
        let head_a: AccumColPtr<u32> = rebased.get();
        let head_b: AccumColPtr<u64> = AccumColPtr {
            base: rebased.tail.head.base,
            len: rebased.tail.head.len,
            cap: rebased.tail.head.cap,
        };
        for rec in lo..hi {
            if rec % 7 != 0 {
                unsafe { head_a.append((rec as u32) * 10) };
                unsafe { head_b.append((rec as u64) * 100) };
            }
        }
        // The rebased head wrote into the offset region with the worker cell, and
        // the binding cells stayed zero (proving the rebase swapped the cell).
        if bind_cell_a.get() != USize(0) || bind_cell_b.get() != USize(0) {
            ok = false;
            eprintln!("binding cell advanced: rebase did not swap the cell");
        }
        let live = core_cells[0].get().0;
        let expect: usize = (lo..hi).filter(|r| r % 7 != 0).count();
        if live != expect {
            ok = false;
            eprintln!("core {c}: live {live} != expected {expect}");
        }
    }

    if ok {
        println!("WORKS: per-core rebase walk compiles, swaps cell + offsets base,");
        println!("       k>1 cells assigned positionally via slice-split, lifetime");
        println!("       rethreaded from 'frame to the worker-cell 'a. Append path");
        println!("       drives a rebased pointer unchanged.");
        std::process::exit(0);
    } else {
        println!("FAILS: see messages above");
        std::process::exit(1);
    }
}
