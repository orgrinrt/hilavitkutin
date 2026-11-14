//! As `bound_units`, and the body does not assume a column is packed: it
//! steps by the stride the binding declares. This is the addressing a
//! column needs when it is a member of a wider shaped store.
use super::*;

unsafe fn one(table: *const Binding, row: *const UnitRow, lo: usize, hi: usize) {
    // SAFETY: the caller hands a row of the unit table and a morsel inside
    // the columns.
    unsafe {
        let row = &*row;
        let (Some((a, sa)), Some((b, sb)), Some((t, st))) = (
            resolve_strided::<u32>(table, row.reads[0]),
            resolve_strided::<u32>(table, row.reads[1]),
            resolve_strided::<u32>(table, row.write),
        ) else {
            refuse()
        };
        let mut i = lo;
        while i < hi {
            let x = *(a.add(i * sa) as *const u32);
            let y = *(b.add(i * sb) as *const u32);
            *(t.add(i * st) as *mut u32) = unit_one(x, y);
            i += 1;
        }
    }
}

unsafe fn two(table: *const Binding, row: *const UnitRow, lo: usize, hi: usize) {
    // SAFETY: as `one`.
    unsafe {
        let row = &*row;
        let (Some((t, st)), Some((c, sc)), Some((out, so))) = (
            resolve_strided::<u32>(table, row.reads[0]),
            resolve_strided::<u32>(table, row.reads[1]),
            resolve_strided::<u32>(table, row.write),
        ) else {
            refuse()
        };
        let mut i = lo;
        while i < hi {
            let x = *(t.add(i * st) as *const u32);
            let y = *(c.add(i * sc) as *const u32);
            *(out.add(i * so) as *mut u32) = unit_two(x, y);
            i += 1;
        }
    }
}

pub fn bind(layout: &Layout) -> Tables {
    bind_units(layout, [one, two], [record_never, record_never])
}

#[inline(always)]
pub fn run(s: &Tables) {
    super::bound_units::run(s)
}
