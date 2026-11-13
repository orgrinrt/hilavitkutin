//! Every bind refusal comes back as a value: no table is made, so no frame can run, and the store
//! is as it was.

use hilavitkutin_contracts::bind::{BindRefusal, Binding, Coefficient, Role, UnitId, bind};
use hilavitkutin_contracts::domain::{Cell, FormatId, Value, Word};
use hilavitkutin_contracts::kernel::{KernelId, ValueKernel};
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{Count, Index, Int, USize};
use hilavitkutin_contracts::{Maybe, Outcome};
use hilavitkutin_frozen_authors::{Diffuse, Extent, Plane};
use hilavitkutin_frozen_implementors::Columns;

type L = Int<16>;
type List = Cons<Diffuse<L>, Empty>;
type Store = Columns<Plane, 2, 4>;

fn store() -> Store {
    let shape = Extent {
        east:  Count::from_host(2),
        north: Count::from_host(2),
    };
    let Maybe::Is(mut store) = Store::new(shape, [L::FORMAT, L::FORMAT]) else {
        panic!("fits")
    };
    for at in 0 .. 4 {
        store.set(
            USize::from_host(0),
            Index::from_host(at),
            L::from_raw(at as i64 + 1).cell().word(),
        );
    }
    store
}

fn cells(store: &Store) -> [Word; 8] {
    core::array::from_fn(|i| store.cell(USize::from_host(i / 4), Index::from_host(i % 4)))
}

const fn role(column: usize) -> Role {
    Role {
        column: USize::from_host(column),
        format: <L as Value>::FORMAT,
    }
}

fn rate() -> [Coefficient; 1] {
    [Coefficient {
        format: L::FORMAT,
        word:   L::from_raw(1).cell().word(),
    }]
}

fn refused(bindings: &[Binding<'_>]) -> BindRefusal {
    let store = store();
    let before = cells(&store);
    let refusal = match bind::<Plane, List, Store>(bindings, &store) {
        Outcome::Ok(_) => panic!("the table binds"),
        Outcome::Err(refusal) => refusal,
    };
    assert_eq!(cells(&store), before);
    refusal
}

fn binding<'t>(unit: usize, roles: &'t [Role], params: &'t [Coefficient]) -> Binding<'t> {
    Binding {
        unit: UnitId::new(USize::from_host(unit)),
        kernel: Diffuse::<L>::ID,
        roles,
        params,
    }
}

#[test]
fn a_good_table_binds() {
    let roles = [role(0), role(0), role(0), role(0), role(0), role(1)];
    let params = rate();
    let bindings = [binding(0, &roles, &params)];
    assert!(
        matches!(bind::<Plane, List, Store>(&bindings, &store()), Outcome::Ok(b) if b.units() == Count::ONE)
    );
}

#[test]
fn unknown_kernel() {
    let roles = [role(0); 6];
    let params = rate();
    let mut r = binding(0, &roles, &params);
    r.kernel = KernelId::new(USize::from_host(99));
    assert_eq!(refused(&[r]), BindRefusal::UnknownKernel {
        binding: Index::FIRST,
        kernel:  r.kernel,
    });
}

#[test]
fn format_mismatch_against_the_kernel() {
    let mut roles = [role(0); 6];
    roles[2].format = FormatId::new(USize::from_host(0x0120));
    let params = rate();
    assert_eq!(
        refused(&[binding(0, &roles, &params)]),
        BindRefusal::FormatMismatch {
            binding: Index::FIRST,
            role:    Index::from_host(2),
        }
    );
}

#[test]
fn format_mismatch_against_the_store() {
    let roles = [role(0); 6];
    let params = rate();
    let shape = Extent {
        east:  Count::from_host(2),
        north: Count::from_host(2),
    };
    let Maybe::Is(store) = Store::new(shape, [Int::<32>::FORMAT, L::FORMAT]) else {
        panic!("fits")
    };
    let bindings = [binding(0, &roles, &params)];
    assert!(matches!(
        bind::<Plane, List, Store>(&bindings, &store),
        Outcome::Err(BindRefusal::FormatMismatch { binding, role }) if binding == Index::FIRST && role == Index::FIRST
    ));
}

#[test]
fn coefficient_format_mismatch() {
    let roles = [role(0); 6];
    let params = [Coefficient {
        format: FormatId::new(USize::from_host(0x0120)),
        word:   Word::ZERO,
    }];
    assert_eq!(
        refused(&[binding(0, &roles, &params)]),
        BindRefusal::CoefficientMismatch {
            binding: Index::FIRST,
            param:   Index::FIRST,
        }
    );
}

#[test]
fn unknown_column() {
    let mut roles = [role(0); 6];
    roles[4] = role(2);
    let params = rate();
    assert_eq!(
        refused(&[binding(0, &roles, &params)]),
        BindRefusal::UnknownColumn {
            binding: Index::FIRST,
            role:    Index::from_host(4),
        }
    );
}

#[test]
fn role_count_short_and_long() {
    let short = [role(0); 5];
    let long = [role(0); 7];
    let params = rate();
    assert_eq!(
        refused(&[binding(0, &short, &params)]),
        BindRefusal::RoleCount {
            binding: Index::FIRST,
        }
    );
    assert_eq!(
        refused(&[binding(0, &long, &params)]),
        BindRefusal::RoleCount {
            binding: Index::FIRST,
        }
    );
    let roles = [role(0); 6];
    assert_eq!(
        refused(&[binding(0, &roles, &[])]),
        BindRefusal::RoleCount {
            binding: Index::FIRST,
        }
    );
}

#[test]
fn duplicate_unit() {
    let a = [role(0), role(0), role(0), role(0), role(0), role(0)];
    let b = [role(1), role(1), role(1), role(1), role(1), role(1)];
    let params = rate();
    let unit = UnitId::new(USize::from_host(4));
    assert_eq!(
        refused(&[binding(4, &a, &params), binding(4, &b, &params)]),
        BindRefusal::DuplicateUnit {
            unit
        }
    );
}

#[test]
fn two_writers_of_one_column_across_rows() {
    let a = [role(0), role(0), role(0), role(0), role(0), role(1)];
    let b = [role(0), role(0), role(0), role(0), role(0), role(1)];
    let params = rate();
    assert_eq!(
        refused(&[binding(0, &a, &params), binding(1, &b, &params)]),
        BindRefusal::TwoWritersOfOneColumn {
            column: USize::from_host(1),
        }
    );
}

struct Pair;

impl ValueKernel for Pair {
    type Domain = Plane;
    type Reads = Empty;
    type Writes = Cons<
        hilavitkutin_contracts::domain::Own<hilavitkutin_frozen_authors::Heat<L>>,
        Cons<hilavitkutin_contracts::domain::Own<hilavitkutin_frozen_authors::Heat<L>>, Empty>,
    >;

    const ID: KernelId = KernelId::new(USize::from_host(3));

    fn at((): ()) -> (L, (L, ())) {
        (L::from_raw(1), (L::from_raw(2), ()))
    }
}

#[test]
fn two_writers_of_one_column_within_a_row() {
    let roles = [role(1), role(1)];
    let bindings = [Binding {
        unit:   UnitId::new(USize::from_host(0)),
        kernel: Pair::ID,
        roles:  &roles,
        params: &[],
    }];
    let store = store();
    let before = cells(&store);
    assert!(matches!(
        bind::<Plane, Cons<Pair, List>, Store>(&bindings, &store),
        Outcome::Err(BindRefusal::TwoWritersOfOneColumn { column }) if column == USize::from_host(1)
    ));
    assert_eq!(cells(&store), before);
}
