use super::*;

fn seeded(len: usize, seed: u64) -> Vec<u8> {
    let mut x = seed | 1;
    (0 .. len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x as u8
        })
        .collect()
}

fn all_arms(input: &[u8]) -> [[u8; 8]; 5] {
    let l = layout(input);
    typed::run(&typed::bind(&l));
    let typed = checksum(&l);
    let l = layout(input);
    bound_columns::run(&bound_columns::bind(&l));
    let columns = checksum(&l);
    let l = layout(input);
    bound_units::run(&bound_units::bind(&l));
    let units = checksum(&l);
    let l = layout(input);
    bound_strided::run(&bound_strided::bind(&l));
    let strided = checksum(&l);
    let l = layout(input);
    bound_record::run(&bound_record::bind(&l));
    let record = checksum(&l);
    [typed, columns, units, strided, record]
}

#[test]
fn every_arm_writes_the_same_column() {
    for &len in &[4usize, 2044, 4096, 16384, 65536] {
        for seed in 1 .. 6u64 {
            let sums = all_arms(&seeded(len, seed));
            assert!(
                sums.iter().all(|s| *s == sums[0]),
                "len {len} seed {seed}: {sums:?}"
            );
        }
    }
}

#[test]
fn the_checksum_follows_the_input() {
    let a = all_arms(&seeded(4096, 1))[0];
    let b = all_arms(&seeded(4096, 2))[0];
    assert_ne!(a, b);
}

#[test]
fn the_typed_arm_matches_a_plain_scalar_pass() {
    let input = seeded(4096, 9);
    let l = layout(&input);
    let mut expect = Vec::new();
    for i in 0 .. l.records {
        // SAFETY: the columns hold `records` values.
        let (a, b, c) = unsafe {
            (
                *l.column[A as usize].add(i),
                *l.column[B as usize].add(i),
                *l.column[C as usize].add(i),
            )
        };
        expect.push(unit_two(unit_one(a, b), c));
    }
    typed::run(&typed::bind(&l));
    assert_eq!(checksum(&l), fnv1a_column(expect.as_ptr(), expect.len()));
}

#[test]
fn a_binding_of_another_shape_is_refused() {
    let l = layout(&seeded(64, 3));
    let table = bind_columns(&l);
    // SAFETY: the table holds `COLUMNS` bindings.
    unsafe {
        assert!(resolve::<u32>(table, A).is_some());
        assert!(resolve::<u64>(table, A).is_none());
        assert!(resolve::<u16>(table, A).is_none());
        assert!(resolve::<u32>(table, COLUMNS as u32).is_none());
    }
}

#[test]
fn a_wider_stride_is_refused_as_packed_and_taken_as_strided() {
    let l = layout(&seeded(64, 4));
    let table = bind_columns(&l) as *mut Binding;
    // SAFETY: the table holds `COLUMNS` bindings.
    unsafe {
        (*table.add(B as usize)).stride = 12;
        assert!(resolve::<u32>(table, B).is_none());
        assert_eq!(resolve_strided::<u32>(table, B).map(|(_, s)| s), Some(12));
        (*table.add(B as usize)).stride = 2;
        assert!(resolve_strided::<u32>(table, B).is_none());
    }
}
