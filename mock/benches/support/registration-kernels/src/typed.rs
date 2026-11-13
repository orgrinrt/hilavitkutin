//! Typed registration: bodies monomorphised into the loop, column
//! pointers resolved once.
use super::*;

pub struct Bound {
    pub a:       *const u32,
    pub b:       *const u32,
    pub c:       *const u32,
    pub t:       *mut u32,
    pub out:     *mut u32,
    pub records: usize,
}

pub fn bind(layout: &Layout) -> Bound {
    Bound {
        a:       layout.column[A as usize],
        b:       layout.column[B as usize],
        c:       layout.column[C as usize],
        t:       layout.column[T as usize],
        out:     layout.column[OUT as usize],
        records: layout.records,
    }
}

#[inline(always)]
pub fn run(s: &Bound) {
    // Copied out once, so the loops below hold their pointers the way the
    // table-bound arms hold theirs after a lookup, and the arms differ in
    // where a pointer comes from and in nothing else.
    let (a, b, c, t, out, records) = (s.a, s.b, s.c, s.t, s.out, s.records);
    let mut lo = 0;
    while lo < records {
        let hi = (lo + MORSEL).min(records);
        // SAFETY: every column holds `records` values and `hi <= records`.
        unsafe {
            let mut i = lo;
            while i < hi {
                *t.add(i) = unit_one(*a.add(i), *b.add(i));
                i += 1;
            }
            let mut i = lo;
            while i < hi {
                *out.add(i) = unit_two(*t.add(i), *c.add(i));
                i += 1;
            }
        }
        lo = hi;
    }
}
