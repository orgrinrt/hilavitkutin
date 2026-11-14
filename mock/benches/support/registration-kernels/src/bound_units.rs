//! Units and columns both come from the tables. The loop calls a unit's
//! body through the row's pointer once a morsel, and the body resolves its
//! columns and then runs a typed loop.
use super::*;

unsafe fn one(table: *const Binding, row: *const UnitRow, lo: usize, hi: usize) {
    // SAFETY: the caller hands a row of the unit table and a morsel inside
    // the columns.
    unsafe {
        let row = &*row;
        let a = column::<u32>(table, row.reads[0]) as *const u32;
        let b = column::<u32>(table, row.reads[1]) as *const u32;
        let t = column::<u32>(table, row.write);
        let mut i = lo;
        while i < hi {
            *t.add(i) = unit_one(*a.add(i), *b.add(i));
            i += 1;
        }
    }
}

unsafe fn two(table: *const Binding, row: *const UnitRow, lo: usize, hi: usize) {
    // SAFETY: as `one`.
    unsafe {
        let row = &*row;
        let t = column::<u32>(table, row.reads[0]) as *const u32;
        let c = column::<u32>(table, row.reads[1]) as *const u32;
        let out = column::<u32>(table, row.write);
        let mut i = lo;
        while i < hi {
            *out.add(i) = unit_two(*t.add(i), *c.add(i));
            i += 1;
        }
    }
}

pub fn bind(layout: &Layout) -> Tables {
    bind_units(layout, [one, two], [record_never, record_never])
}

#[inline(always)]
pub fn run(s: &Tables) {
    let mut lo = 0;
    while lo < s.records {
        let hi = (lo + MORSEL).min(s.records);
        let mut u = 0;
        while u < s.unit_count {
            // SAFETY: `u` is inside the unit table.
            unsafe {
                let row = s.units.add(u);
                ((*row).morsel)(s.columns, row, lo, hi);
            }
            u += 1;
        }
        lo = hi;
    }
}
