//! The units stay typed and monomorphised. Their columns are declared from
//! data, and each unit resolves its three through the table once a morsel.
use super::*;

pub struct Bound {
    pub table:   *const Binding,
    pub units:   [UnitDecl; UNITS],
    pub records: usize,
}

pub fn bind(layout: &Layout) -> Bound {
    Bound {
        table:   bind_columns(layout),
        units:   declared_units(),
        records: layout.records,
    }
}

#[inline(always)]
pub fn run(s: &Bound) {
    let mut lo = 0;
    while lo < s.records {
        let hi = (lo + MORSEL).min(s.records);
        // SAFETY: the table holds `COLUMNS` bindings of `records` values.
        unsafe {
            let one = &s.units[0];
            let a = column::<u32>(s.table, one.reads[0]) as *const u32;
            let b = column::<u32>(s.table, one.reads[1]) as *const u32;
            let t = column::<u32>(s.table, one.write);
            let mut i = lo;
            while i < hi {
                *t.add(i) = unit_one(*a.add(i), *b.add(i));
                i += 1;
            }
            let two = &s.units[1];
            let t = column::<u32>(s.table, two.reads[0]) as *const u32;
            let c = column::<u32>(s.table, two.reads[1]) as *const u32;
            let out = column::<u32>(s.table, two.write);
            let mut i = lo;
            while i < hi {
                *out.add(i) = unit_two(*t.add(i), *c.add(i));
                i += 1;
            }
        }
        lo = hi;
    }
}
