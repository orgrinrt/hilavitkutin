//! The kernels run through the reference runner and are checked against a
//! plain array model of the same rule, per morsel.

use core::num::NonZeroUsize;

use author::{BelowMax, Double, Previous};
use engine::{BlockKernel, Morsel, Unequal, run};

const RECORDS: usize = 10;
const MORSEL: NonZeroUsize = NonZeroUsize::new(4).unwrap();
const READ: [i64; RECORDS] = [5, -3, 8, 1, 0, 7, -2, 9, 4, 6];

fn ran<K: BlockKernel<i64, i64>>(kernel: &K) -> [i64; RECORDS] {
    let mut write = [0; RECORDS];
    run(kernel, &READ, &mut write, MORSEL).unwrap();
    write
}

/// The model: the rule applied to each morsel's records as a plain slice.
fn model(rule: fn(&[i64], &mut [i64])) -> [i64; RECORDS] {
    let mut write = [0; RECORDS];
    for (read, write) in READ
        .chunks(MORSEL.get())
        .zip(write.chunks_mut(MORSEL.get()))
    {
        rule(read, write);
    }
    write
}

#[test]
fn double_writes_each_record_doubled() {
    let want = model(|read, write| {
        for record in 0 .. read.len() {
            write[record] = read[record] * 2;
        }
    });
    assert_eq!(ran(&Double), want);
}

#[test]
fn below_max_takes_each_morsels_own_largest() {
    let want = model(|read, write| {
        let max = *read.iter().max().unwrap();
        for record in 0 .. read.len() {
            write[record] = read[record] - max;
        }
    });
    assert_eq!(ran(&BelowMax), want);
    // The morsels are [5 -3 8 1] [0 7 -2 9] [4 6], so the largest differs per morsel.
    assert_eq!(want, [-3, -11, 0, -7, -9, -2, -11, 0, -2, 0]);
}

#[test]
fn previous_keeps_an_index_across_records_of_one_morsel() {
    let want = model(|read, write| {
        for record in 0 .. read.len() {
            write[record] = read[if record == 0 { 0 } else { record - 1 }];
        }
    });
    assert_eq!(ran(&Previous), want);
}

#[test]
fn columns_of_unequal_length_are_refused() {
    let mut write = [0; RECORDS - 1];
    assert_eq!(
        run(&Double, &READ, &mut write, MORSEL),
        Err(Unequal {
            read:  RECORDS,
            write: RECORDS - 1,
        })
    );
}

/// The hole the brand does not close: the sequence `indices` yields in is the
/// engine's, and an author can count along it. `nth` hands back the index at
/// a position, which is an integer reaching an index by another road.
struct Third;

impl BlockKernel<i64, i64> for Third {
    fn block(&self, mut morsel: Morsel<'_, i64, i64>) {
        morsel.each(|mut block| {
            if let Some(third) = block.indices().nth(2) {
                block.write(third, 1);
            }
        });
    }
}

#[test]
fn counting_along_the_indices_reaches_a_position() {
    assert_eq!(ran(&Third), [0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
}
