//! A role list naming one column twice, bound with its places written out: the read of `Heat` is
//! placed on the first role and the write on the second, so one column's members fill two roles
//! and read one stored column while writing another. One role per column should refuse this.
use hilavitkutin_contracts::bind::{Binding, Role, UnitId, bind};
use hilavitkutin_contracts::domain::{Own, Value};
use hilavitkutin_contracts::kernel::{KernelId, ValueKernel};
use hilavitkutin_contracts::list::{Cons, Empty, Here, There};
use hilavitkutin_contracts::num::{ColumnNo, Count, Int, USize};
use hilavitkutin_contracts::{Maybe, Outcome};
use hilavitkutin_frozen_authors::{Extent, Heat, Plane};
use hilavitkutin_frozen_implementors::Columns;

type L = Int<16>;

struct Split;

impl ValueKernel for Split {
    type Domain = Plane;
    type Reads = Cons<Own<Heat<L>>, Empty>;
    type Roles = Cons<Heat<L>, Cons<Heat<L>, Empty>>;
    type Writes = Cons<Own<Heat<L>>, Empty>;

    const ID: KernelId = KernelId::new(USize::from_host(7));

    fn at(reads: (L, ())) -> (L, ()) {
        reads
    }
}

type Places = (((Here, ()), (There<Here>, ())), ());

const fn role(column: usize) -> Role {
    Role {
        column: ColumnNo::from_host(column),
        format: <L as Value>::FORMAT,
    }
}

fn main() {
    let shape = Extent {
        east:  Count::from_host(2),
        north: Count::from_host(2),
    };
    let Maybe::Is(mut store) = Columns::<Plane, 2, 4>::new(shape, [L::FORMAT; 2]) else {
        return;
    };
    let roles = [role(0), role(1)];
    let bindings = [Binding {
        unit:   UnitId::new(USize::from_host(0)),
        kernel: Split::ID,
        roles:  &roles,
        params: &[],
    }];
    let bound = bind::<Plane, Cons<Split, Empty>, _, Places>(&bindings, &mut store);
    assert!(matches!(bound, Outcome::Ok(_)));
}
