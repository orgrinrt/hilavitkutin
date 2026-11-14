//! The loop calls a unit's body through the row's pointer once a record,
//! and the body resolves its columns each time. This is what a table costs
//! when nothing hoists the lookup out of the record loop.
use super::*;

unsafe fn one(table: *const Binding, row: *const UnitRow, i: usize) {
    // SAFETY: the caller hands a row of the unit table and a record inside
    // the columns.
    unsafe {
        let row = &*row;
        let a = column::<u32>(table, row.reads[0]) as *const u32;
        let b = column::<u32>(table, row.reads[1]) as *const u32;
        let t = column::<u32>(table, row.write);
        *t.add(i) = unit_one(*a.add(i), *b.add(i));
    }
}

unsafe fn two(table: *const Binding, row: *const UnitRow, i: usize) {
    // SAFETY: as `one`.
    unsafe {
        let row = &*row;
        let t = column::<u32>(table, row.reads[0]) as *const u32;
        let c = column::<u32>(table, row.reads[1]) as *const u32;
        let out = column::<u32>(table, row.write);
        *out.add(i) = unit_two(*t.add(i), *c.add(i));
    }
}

pub fn bind(layout: &Layout) -> Tables {
    bind_units(layout, [morsel_never, morsel_never], [one, two])
}

#[inline(always)]
pub fn run(s: &Tables) {
    let mut lo = 0;
    while lo < s.records {
        let hi = (lo + MORSEL).min(s.records);
        let mut u = 0;
        while u < s.unit_count {
            let mut i = lo;
            while i < hi {
                // SAFETY: `u` is inside the unit table, `i` inside the
                // columns.
                unsafe {
                    let row = s.units.add(u);
                    ((*row).record)(s.columns, row, i);
                }
                i += 1;
            }
            u += 1;
        }
        lo = hi;
    }
}
