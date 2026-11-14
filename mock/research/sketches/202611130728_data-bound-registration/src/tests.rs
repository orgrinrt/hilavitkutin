//! The sketch's checks: declared units over declared columns against the
//! typed result, and each refusal the table makes.

use super::*;

const A: Slot = Slot(0);
const B: Slot = Slot(1);
const C: Slot = Slot(2);
const T: Slot = Slot(3);
const OUT: Slot = Slot(4);
const RECORDS: usize = 1500;
const MORSEL: usize = 512;

fn one(a: u32, b: u32) -> u32 {
    a.wrapping_mul(2654435761).wrapping_add(b.rotate_left(7))
}

fn two(t: u32, c: u32) -> u32 {
    (t ^ c.wrapping_mul(2246822519)).rotate_left(13)
}

fn body_one(table: &Table<'_, 8>, unit: &UnitDecl, w: Window) -> Result<(), Refused> {
    let a = table.column::<u32>(unit.reads[0])?;
    let b = table.column::<u32>(unit.reads[1])?;
    let t = table.column::<u32>(unit.write)?;
    for i in w.lo .. w.hi {
        // SAFETY: the window is inside the columns, and the plan gave
        // this unit the only write of its column in this phase.
        unsafe { t.set(i, one(a.get(i), b.get(i))) };
    }
    Ok(())
}

fn body_two(table: &Table<'_, 8>, unit: &UnitDecl, w: Window) -> Result<(), Refused> {
    let t = table.column::<u32>(unit.reads[0])?;
    let c = table.column::<u32>(unit.reads[1])?;
    let out = table.column::<u32>(unit.write)?;
    for i in w.lo .. w.hi {
        // SAFETY: as `body_one`.
        unsafe { out.set(i, two(t.get(i), c.get(i))) };
    }
    Ok(())
}

const BODIES: Bodies<8, 2> = Bodies([body_one, body_two]);

fn bytes(words: &mut [u32]) -> &mut [u8] {
    // SAFETY: `u32` has no padding and any bytes are `u32` values.
    unsafe { core::slice::from_raw_parts_mut(words.as_mut_ptr().cast::<u8>(), words.len() * 4) }
}

struct Store {
    a:   Vec<u32>,
    b:   Vec<u32>,
    c:   Vec<u32>,
    t:   Vec<u32>,
    out: Vec<u32>,
}

fn store() -> Store {
    let a: Vec<u32> = (0 .. RECORDS as u32)
        .map(|i| i.wrapping_mul(40503) ^ 0x9E37)
        .collect();
    let b: Vec<u32> = a.iter().map(|x| x.rotate_left(11) ^ 7).collect();
    let c: Vec<u32> = a.iter().map(|x| x.wrapping_add(0x85EB_CA6B)).collect();
    Store {
        a,
        b,
        c,
        t: vec![0; RECORDS],
        out: vec![0; RECORDS],
    }
}

fn expected(s: &Store) -> Vec<u32> {
    (0 .. RECORDS)
        .map(|i| two(one(s.a[i], s.b[i]), s.c[i]))
        .collect()
}

fn declare_all<'s>(s: &'s mut Store) -> Table<'s, 8> {
    let mut table = Table::<8>::new();
    let u = ColumnDecl::packed::<u32>();
    assert_eq!(table.declare(u, bytes(&mut s.a)), Ok(A));
    assert_eq!(table.declare(u, bytes(&mut s.b)), Ok(B));
    assert_eq!(table.declare(u, bytes(&mut s.c)), Ok(C));
    assert_eq!(table.declare(u, bytes(&mut s.t)), Ok(T));
    assert_eq!(table.declare(u, bytes(&mut s.out)), Ok(OUT));
    table
}

const UNITS: [UnitDecl; 2] = [
    UnitDecl {
        body:  0,
        reads: [A, B],
        write: T,
    },
    UnitDecl {
        body:  1,
        reads: [T, C],
        write: OUT,
    },
];

#[test]
fn declared_units_over_declared_columns_compute_the_typed_result() {
    let mut s = store();
    let want = expected(&s);
    let table = declare_all(&mut s);
    let (reads, writes) = masks(&UNITS);
    let phase = phases(&reads, &writes).unwrap();
    assert_eq!(phase, [0, 1]);
    run(&table, &BODIES, &UNITS, &phase, RECORDS, MORSEL).unwrap();
    drop(table);
    assert_eq!(s.out, want);
}

#[test]
fn the_order_of_declaration_does_not_change_the_result() {
    let mut s = store();
    let want = expected(&s);
    let table = declare_all(&mut s);
    let turned = [UNITS[1], UNITS[0]];
    let (reads, writes) = masks(&turned);
    let phase = phases(&reads, &writes).unwrap();
    assert_eq!(phase, [1, 0]);
    run(&table, &BODIES, &turned, &phase, RECORDS, MORSEL).unwrap();
    drop(table);
    assert_eq!(s.out, want);
}

#[test]
fn a_plan_that_ignores_the_edge_computes_something_else() {
    // The case that must fail: with both units in one phase and the
    // reader declared first, the reader sees the column before its
    // writer ran. This is what `phases` is there to prevent.
    let mut s = store();
    let want = expected(&s);
    let table = declare_all(&mut s);
    let turned = [UNITS[1], UNITS[0]];
    run(&table, &BODIES, &turned, &[0, 0], RECORDS, MORSEL).unwrap();
    drop(table);
    assert_ne!(s.out, want);
}

#[test]
fn the_typed_plan_and_the_table_plan_are_one_function() {
    let (reads, writes) = masks(&UNITS);
    assert_eq!(phases(&reads, &writes), TYPED_PLAN);
    assert_eq!(TYPED_PLAN, Ok([0, 1]));
}

#[test]
fn two_writers_and_a_cycle_are_refused() {
    let twice = [UNITS[0], UnitDecl {
        body:  0,
        reads: [A, C],
        write: T,
    }];
    let (r, w) = masks(&twice);
    assert_eq!(phases(&r, &w), Err(PlanRefused::TwoWriters));
    let ring = [
        UnitDecl {
            body:  0,
            reads: [A, OUT],
            write: T,
        },
        UnitDecl {
            body:  1,
            reads: [T, C],
            write: OUT,
        },
    ];
    let (r, w) = masks(&ring);
    assert_eq!(phases(&r, &w), Err(PlanRefused::Cycle));
}

#[test]
fn a_view_of_another_shape_is_refused() {
    let mut s = store();
    let table = declare_all(&mut s);
    assert!(table.column::<u32>(A).is_ok());
    assert_eq!(
        table.column::<u64>(A).err(),
        Some(Refused::Shape {
            declared: Shape::of::<u32>(),
            asked:    Shape::of::<u64>(),
        })
    );
    assert_eq!(
        table.column::<[u8; 4]>(A).err(),
        Some(Refused::Shape {
            declared: Shape::of::<u32>(),
            asked:    Shape::of::<[u8; 4]>(),
        })
    );
    // Same alignment, another size: the case a check of the alignment
    // alone lets through to the stride check.
    assert_eq!(
        table.column::<[u32; 2]>(A).err(),
        Some(Refused::Shape {
            declared: Shape::of::<u32>(),
            asked:    Shape::of::<[u32; 2]>(),
        })
    );
    assert_eq!(
        table.column::<u32>(Slot(5)).err(),
        Some(Refused::NoSuchSlot)
    );
    assert_eq!(
        table.column::<u32>(Slot(99)).err(),
        Some(Refused::NoSuchSlot)
    );
}

#[test]
fn a_shape_cannot_tell_two_types_of_one_layout_apart() {
    // Not a refusal, and recorded as a limit: `f32` has the shape of
    // `u32`, so the table hands the view out. Both are plain, so nothing
    // is unsound, and the meaning of the bits is whatever the reader
    // takes it to be. Telling them apart takes a name in the
    // declaration, which a shape does not carry.
    let mut s = store();
    let table = declare_all(&mut s);
    assert!(table.column::<f32>(A).is_ok());
}

#[test]
fn a_bad_declaration_is_refused() {
    let mut words = [0u32; 8];
    let region = bytes(&mut words);
    let mut table = Table::<1>::new();
    let odd = &mut region[1 ..];
    assert_eq!(
        table.declare(ColumnDecl::packed::<u32>(), odd),
        Err(Refused::Misaligned)
    );
    let mut words = [0u32; 8];
    assert_eq!(
        table.declare(
            ColumnDecl {
                shape:  Shape::of::<u32>(),
                stride: 2,
            },
            bytes(&mut words)
        ),
        Err(Refused::StrideUnderSize)
    );
    let mut one = [0u32; 8];
    let mut two = [0u32; 8];
    assert!(
        table
            .declare(ColumnDecl::packed::<u32>(), bytes(&mut one))
            .is_ok()
    );
    assert_eq!(
        table.declare(ColumnDecl::packed::<u32>(), bytes(&mut two)),
        Err(Refused::Full)
    );
}

#[test]
fn a_strided_column_is_not_handed_out_as_packed() {
    let mut words = [0u32; 12];
    let mut table = Table::<1>::new();
    let wide = ColumnDecl {
        shape:  Shape::of::<u32>(),
        stride: 12,
    };
    let slot = table.declare(wide, bytes(&mut words)).unwrap();
    assert_eq!(table.column::<u32>(slot).err(), Some(Refused::NotPacked));
}

#[test]
fn a_unit_naming_no_body_is_refused() {
    let mut s = store();
    let table = declare_all(&mut s);
    let stray = [UnitDecl {
        body:  7,
        reads: [A, B],
        write: T,
    }];
    assert_eq!(
        run(&table, &BODIES, &stray, &[0], RECORDS, MORSEL),
        Err(Refused::NoSuchBody)
    );
}
