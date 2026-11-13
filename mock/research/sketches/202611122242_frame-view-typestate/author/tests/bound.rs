//! The data-bound twin of fixtures 05 and 06: columns minted at bind, the
//! second writer refused at bind as a value.

use engine::{
    BoundKernel,
    Checked,
    Column,
    ColumnStaged,
    ColumnView,
    Columns,
    Overlap,
    Step,
    Undeclared,
};

const ROWS: usize = 4;
const COLS: usize = 3;

type Store = Columns<i64, COLS, ROWS>;

struct Drift {
    pos: Column<i64>,
    vel: Column<i64>,
}

impl BoundKernel<i64, COLS, ROWS> for Drift {
    fn run(
        &self,
        view: ColumnView<'_, i64, COLS, ROWS>,
        mut out: ColumnStaged<'_, i64, COLS, ROWS>,
    ) {
        for row in view.rows() {
            out.set(
                self.pos,
                row,
                view.get(self.pos, row) + view.get(self.vel, row),
            )
            .unwrap();
        }
    }
}

struct Cool {
    heat: Column<i64>,
}

impl BoundKernel<i64, COLS, ROWS> for Cool {
    fn run(
        &self,
        view: ColumnView<'_, i64, COLS, ROWS>,
        mut out: ColumnStaged<'_, i64, COLS, ROWS>,
    ) {
        for row in view.rows() {
            out.set(self.heat, row, view.get(self.heat, row) / 2)
                .unwrap();
        }
    }
}

fn store() -> (Store, Column<i64>, Column<i64>, Column<i64>) {
    let mut store = Store::new(0);
    let pos = store.bind().unwrap();
    let vel = store.bind().unwrap();
    let heat = store.bind().unwrap();
    let Step {
        view,
        staged: [mut seed],
    } = store.begin(&Checked::bind([&[vel, heat][..]]).unwrap());
    for row in view.rows() {
        seed.set(vel, row, 3).unwrap();
        seed.set(heat, row, 64).unwrap();
    }
    store.land();
    (store, pos, vel, heat)
}

#[test]
fn a_second_writer_of_one_column_is_refused_at_bind() {
    let (_, pos, vel, heat) = store();
    let refused = Checked::bind([&[pos, vel][..], &[heat, pos][..]]);
    assert_eq!(
        refused.err(),
        Some(Overlap {
            column: 0,
            first:  0,
            second: 1,
        })
    );
}

#[test]
fn one_kernel_naming_one_column_twice_is_refused_at_bind() {
    let (_, pos, _, heat) = store();
    let refused = Checked::bind([&[pos, heat, pos][..]]);
    assert_eq!(
        refused.err(),
        Some(Overlap {
            column: 0,
            first:  0,
            second: 0,
        })
    );
}

#[test]
fn disjoint_write_sets_bind_and_step_together() {
    let (mut store, pos, vel, heat) = store();
    let checked = Checked::bind([&[pos][..], &[heat][..]]).unwrap();
    store.step2(
        &checked,
        &Drift {
            pos,
            vel,
        },
        &Cool {
            heat,
        },
    );
    store.step2(
        &checked,
        &Drift {
            pos,
            vel,
        },
        &Cool {
            heat,
        },
    );
    assert_eq!(store.committed(pos), &[6; ROWS]);
    assert_eq!(store.committed(vel), &[3; ROWS]);
    assert_eq!(store.committed(heat), &[16; ROWS]);
}

#[test]
fn a_write_to_a_column_the_set_does_not_name_is_refused() {
    let (mut store, pos, vel, _) = store();
    let checked = Checked::bind([&[pos][..]]).unwrap();
    let Step {
        view,
        staged: [mut out],
    } = store.begin(&checked);
    let row = view.rows().next().unwrap();
    assert_eq!(
        out.set(vel, row, 9),
        Err(Undeclared {
            column: 1,
        })
    );
    assert_eq!(out.set(pos, row, 9), Ok(()));
}

#[test]
fn a_column_past_the_capacity_is_refused_at_bind() {
    let (mut store, ..) = store();
    assert!(store.bind().is_none());
}

#[test]
fn the_staged_columns_of_two_kernels_run_on_two_scoped_threads() {
    let (mut store, pos, vel, heat) = store();
    let checked = Checked::bind([&[pos][..], &[heat][..]]).unwrap();
    let drift = Drift {
        pos,
        vel,
    };
    let cool = Cool {
        heat,
    };
    let Step {
        view,
        staged: [a, b],
    } = store.begin(&checked);
    std::thread::scope(|scope| {
        scope.spawn(|| drift.run(view, a));
        scope.spawn(|| cool.run(view, b));
    });
    store.land();
    assert_eq!(store.committed(pos), &[3; ROWS]);
    assert_eq!(store.committed(heat), &[32; ROWS]);
}
