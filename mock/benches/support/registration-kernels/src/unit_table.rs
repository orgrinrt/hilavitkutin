//! The unit table of the three arms whose units are declared from data.

use super::*;

/// A unit body called once per morsel.
pub type MorselBody = unsafe fn(*const Binding, *const UnitRow, usize, usize);

/// A unit body called once per record.
pub type RecordBody = unsafe fn(*const Binding, *const UnitRow, usize);

/// One row of the unit table. Which of the two body fields a run uses is the
/// arm's choice; a row is filled for both.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnitRow {
    pub morsel: MorselBody,
    pub record: RecordBody,
    pub reads:  [u32; 2],
    pub write:  u32,
}

/// A run-time table of units over a run-time table of columns.
pub struct Tables {
    pub columns:    *const Binding,
    pub units:      *const UnitRow,
    pub unit_count: usize,
    pub records:    usize,
}

pub(crate) fn bind_units(
    layout: &Layout,
    morsel: [MorselBody; UNITS],
    record: [RecordBody; UNITS],
) -> Tables {
    let columns = bind_columns(layout);
    // The unit table follows the binding table in the same arena region.
    // SAFETY: the region is `TABLE_BYTES` long, which holds both tables.
    let units = unsafe { (layout.tables as *mut Binding).add(COLUMNS) } as *mut UnitRow;
    let decls = declared_units();
    for u in 0 .. UNITS {
        let body = decls[u].body as usize % UNITS;
        // SAFETY: inside the table region, aligned for a row.
        unsafe {
            *units.add(u) = UnitRow {
                morsel: morsel[body],
                record: record[body],
                reads:  decls[u].reads,
                write:  decls[u].write,
            };
        }
    }
    Tables {
        columns,
        units: units as *const UnitRow,
        unit_count: black_box(UNITS),
        records: layout.records,
    }
}

pub(crate) unsafe fn record_never(_: *const Binding, _: *const UnitRow, _: usize) {
    refuse()
}

pub(crate) unsafe fn morsel_never(_: *const Binding, _: *const UnitRow, _: usize, _: usize) {
    refuse()
}
