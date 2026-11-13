//! Every bind refusal comes back as a value, and no table is made, so no frame can run. The store
//! is lent to `bind` and comes back when it refuses.

use hilavitkutin_contracts::bind::{BindRefusal, Binding, Coefficient, Role, UnitId, bind};
use hilavitkutin_contracts::domain::{Cell, Column, Own, Value, Word};
use hilavitkutin_contracts::kernel::{KernelId, ValueKernel};
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{
    Additive,
    ColumnNo,
    Count,
    Identity,
    Index,
    Int,
    Ordinal,
    USize,
};
use hilavitkutin_contracts::{Maybe, Outcome};
use hilavitkutin_frozen_authors::{Diffuse, Extent, Heat, Plane};
use hilavitkutin_frozen_implementors::Columns;

type L = Int<16>;
type Wider = Int<32>;
type List = Cons<Diffuse<L>, Empty>;
type Store = Columns<Plane, 2, 4>;

const SHAPE: Extent = Extent {
    east:  Count::from_host(2),
    north: Count::from_host(2),
};

fn store_of(formats: [hilavitkutin_contracts::domain::FormatId; 2]) -> Store {
    let Maybe::Is(mut store) = Store::new(SHAPE, formats) else { panic!("fits") };
    for at in 0 .. 4 {
        store.set(
            ColumnNo::from_host(1),
            Index::from_host(at),
            L::from_raw(at as i64 + 1).cell().word(),
        );
    }
    store
}

fn store() -> Store {
    store_of([L::FORMAT; 2])
}

const fn role(column: usize) -> Role {
    Role {
        column: ColumnNo::from_host(column),
        format: <L as Value>::FORMAT,
    }
}

fn rate() -> [Coefficient; 1] {
    [Coefficient {
        format: L::FORMAT,
        word:   L::from_raw(1).cell().word(),
    }]
}

fn refused<K: hilavitkutin_contracts::kernel::Kernels<Plane, P>, P>(
    bindings: &[Binding<'_>],
) -> BindRefusal {
    let mut store = store();
    match bind::<Plane, K, Store, P>(bindings, &mut store) {
        Outcome::Ok(_) => panic!("the table binds"),
        Outcome::Err(refusal) => refusal,
    }
}

fn binding<'t>(unit: usize, roles: &'t [Role], params: &'t [Coefficient]) -> Binding<'t> {
    Binding {
        unit: UnitId::new(USize::from_host(unit)),
        kernel: Diffuse::<L>::ID,
        roles,
        params,
    }
}

const FIRST: Ordinal = Ordinal::FIRST;

#[test]
fn a_good_table_binds() {
    let roles = [role(1)];
    let params = rate();
    let bindings = [binding(0, &roles, &params)];
    let mut store = store();
    let Outcome::Ok(bound) = bind::<Plane, List, Store, _>(&bindings, &mut store) else {
        panic!("the table binds")
    };
    assert_eq!(bound.units(), Count::ONE);
    assert_eq!(
        bound.store().cell(ColumnNo::from_host(1), Index::FIRST),
        L::from_raw(1).cell().word()
    );
}

#[test]
fn unknown_kernel() {
    let roles = [role(1)];
    let params = rate();
    let mut r = binding(0, &roles, &params);
    r.kernel = KernelId::new(USize::from_host(99));
    assert_eq!(refused::<List, _>(&[r]), BindRefusal::UnknownKernel {
        binding: FIRST,
        kernel:  r.kernel,
    });
}

#[test]
fn a_kernel_id_too_wide_for_the_list_is_only_unknown_as_data() {
    let roles = [role(1)];
    let params = rate();
    let mut r = binding(0, &roles, &params);
    r.kernel = KernelId::new(USize::from_host(usize::MAX));
    assert_eq!(refused::<List, _>(&[r]), BindRefusal::UnknownKernel {
        binding: FIRST,
        kernel:  r.kernel,
    });
}

#[test]
fn format_mismatch_against_the_kernel() {
    let mut roles = [role(1)];
    roles[0].format = Wider::FORMAT;
    let params = rate();
    assert_eq!(
        refused::<List, _>(&[binding(0, &roles, &params)]),
        BindRefusal::FormatMismatch {
            binding: FIRST,
            role:    FIRST,
        }
    );
}

#[test]
fn format_mismatch_against_the_store() {
    let roles = [role(1)];
    let params = rate();
    let mut store = store_of([L::FORMAT, Wider::FORMAT]);
    let bindings = [binding(0, &roles, &params)];
    assert!(matches!(
        bind::<Plane, List, Store, _>(&bindings, &mut store),
        Outcome::Err(BindRefusal::FormatMismatch { binding, role }) if binding == FIRST && role == FIRST
    ));
}

#[test]
fn coefficient_format_mismatch() {
    let roles = [role(1)];
    let params = [Coefficient {
        format: Wider::FORMAT,
        word:   Word::ZERO,
    }];
    assert_eq!(
        refused::<List, _>(&[binding(0, &roles, &params)]),
        BindRefusal::CoefficientMismatch {
            binding: FIRST,
            param:   FIRST,
        }
    );
}

#[test]
fn unknown_column() {
    let roles = [role(2)];
    let params = rate();
    assert_eq!(
        refused::<List, _>(&[binding(0, &roles, &params)]),
        BindRefusal::UnknownColumn {
            binding: FIRST,
            role:    FIRST,
        }
    );
}

#[test]
fn role_count_short_and_long_and_coefficients_short() {
    let long = [role(1), role(0)];
    let params = rate();
    for roles in [&[][..], &long[..]] {
        assert_eq!(
            refused::<List, _>(&[binding(0, roles, &params)]),
            BindRefusal::RoleCount {
                binding: FIRST,
            }
        );
    }
    let roles = [role(1)];
    assert_eq!(
        refused::<List, _>(&[binding(0, &roles, &[])]),
        BindRefusal::RoleCount {
            binding: FIRST,
        }
    );
}

#[test]
fn duplicate_unit() {
    let a = [role(1)];
    let b = [role(0)];
    let params = rate();
    assert_eq!(
        refused::<List, _>(&[binding(4, &a, &params), binding(4, &b, &params)]),
        BindRefusal::DuplicateUnit {
            unit: UnitId::new(USize::from_host(4)),
        }
    );
}

#[test]
fn two_writers_of_one_column_across_bindings() {
    let roles = [role(1)];
    let params = rate();
    assert_eq!(
        refused::<List, _>(&[binding(0, &roles, &params), binding(1, &roles, &params)]),
        BindRefusal::TwoWritersOfOneColumn {
            column: ColumnNo::from_host(1),
        }
    );
}

/// Writes its one column twice.
struct Pair;

impl ValueKernel for Pair {
    type Domain = Plane;
    type Reads = Empty;
    type Roles = Cons<Heat<L>, Empty>;
    type Writes = Cons<Own<Heat<L>>, Cons<Own<Heat<L>>, Empty>>;

    const ID: KernelId = KernelId::new(USize::from_host(3));

    fn at((): ()) -> (L, (L, ())) {
        (L::from_raw(1), (L::from_raw(2), ()))
    }
}

#[test]
fn two_writers_of_one_column_within_a_binding() {
    let roles = [role(1)];
    let bindings = [Binding {
        unit:   UnitId::new(USize::from_host(0)),
        kernel: Pair::ID,
        roles:  &roles,
        params: &[],
    }];
    assert_eq!(
        refused::<Cons<Pair, List>, _>(&bindings),
        BindRefusal::TwoWritersOfOneColumn {
            column: ColumnNo::from_host(1),
        }
    );
}

/// A second column of the tests' own.
struct Flux;

impl Column for Flux {
    type Domain = Plane;
    type Value = L;

    const OUTSIDE: L = <L as Identity<Additive>>::IDENTITY;
}

/// Writes two columns of its own.
struct Split;

impl ValueKernel for Split {
    type Domain = Plane;
    type Reads = Empty;
    type Roles = Cons<Heat<L>, Cons<Flux, Empty>>;
    type Writes = Cons<Own<Heat<L>>, Cons<Own<Flux>, Empty>>;

    const ID: KernelId = KernelId::new(USize::from_host(4));

    fn at((): ()) -> (L, (L, ())) {
        (L::from_raw(1), (L::from_raw(2), ()))
    }
}

/// Reads one column and writes another.
struct Carry;

impl ValueKernel for Carry {
    type Domain = Plane;
    type Reads = Cons<Own<Heat<L>>, Empty>;
    type Roles = Cons<Heat<L>, Cons<Flux, Empty>>;
    type Writes = Cons<Own<Flux>, Empty>;

    const ID: KernelId = KernelId::new(USize::from_host(5));

    fn at((heat, ()): (L, ())) -> (L, ()) {
        (heat, ())
    }
}

fn of<'t>(kernel: KernelId, roles: &'t [Role]) -> Binding<'t> {
    Binding {
        unit: UnitId::new(USize::from_host(0)),
        kernel,
        roles,
        params: &[],
    }
}

#[test]
fn two_written_roles_on_one_stored_column_are_two_writers() {
    let roles = [role(1), role(1)];
    assert_eq!(
        refused::<Cons<Split, Empty>, _>(&[of(Split::ID, &roles)]),
        BindRefusal::TwoWritersOfOneColumn {
            column: ColumnNo::from_host(1),
        }
    );
}

#[test]
fn a_read_role_and_a_written_role_may_share_a_stored_column() {
    let roles = [role(1), role(1)];
    let bindings = [of(Carry::ID, &roles)];
    let mut store = store();
    assert!(matches!(
        bind::<Plane, Cons<Carry, Empty>, Store, _>(&bindings, &mut store),
        Outcome::Ok(bound) if bound.units() == Count::ONE
    ));
}
