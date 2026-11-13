//! A runner on several threads built from the bound table alone: every unit's staged part is cut
//! in two by position, and each half runs on a thread of its own, all at once. No author and no
//! store is changed for it, and its digests equal the one-thread reference's.

mod common;

use std::thread::{Scope, scope};

use common::probe::{Honest, Probe};
use common::{COLUMNS, Cells, FRAMES, Input, List, MORSELS, Table, bound, digest_stream, loaded};
use hilavitkutin_contracts::Outcome;
use hilavitkutin_contracts::bind::{StagedFn, Staging, StepFn, Units};
use hilavitkutin_contracts::domain::Word;
use hilavitkutin_contracts::kernel::{Kernels, ValueKernel};
use hilavitkutin_contracts::num::{Count, Index};
use hilavitkutin_contracts::store::{Read, Stage};
use hilavitkutin_frozen_authors::Plane;
use hilavitkutin_frozen_implementors::{Columns, Forward, Records};
use hilavitkutin_laws::{Law, Verdict, morsel_split_and_order};
use hilavitkutin_reference::digest;

type One = Columns<Plane, COLUMNS, { common::CELLS }>;
type Two = Records<Plane, COLUMNS, { common::CELLS }>;

fn fill<K: ValueKernel<Domain = Plane>, W: Stage<Plane>, R: Read<Plane>>(
    mut staging: Staging<'_, K, W, R>,
) {
    let mut at = staging.low();
    while at.lt(staging.high()).get() {
        let values = K::at(staging.unit().gather(at));
        assert!(matches!(staging.scatter(at, values), Outcome::Ok(())));
        at = at.next();
    }
}

struct Spread<'scope, 'env> {
    scope: &'scope Scope<'scope, 'env>,
}

impl<'scope, 'env, 'v: 'scope, R: Read<Plane> + Sync + 'v, W: Stage<Plane> + 'scope>
    StagedFn<'v, Plane, W, R> for Spread<'scope, 'env>
{
    fn unit<K: ValueKernel<Domain = Plane>>(&mut self, staging: Staging<'v, K, W, R>) {
        let middle = Index::from_host((staging.low().host() + staging.high().host()) / 2);
        let (below, from) = staging.split(middle);
        self.scope.spawn(move || fill(below));
        self.scope.spawn(move || fill(from));
    }
}

struct Threads;

macro_rules! threads_over {
    ($($store:ty),*) => {$(
        impl StepFn<Plane, $store> for Threads {
            fn step<'v, 's, L: Kernels<Plane, P>, P>(
                &mut self,
                mut units: Units<'v, 's, Plane, L, $store, P>,
            ) where
                $store: 's,
            {
                scope(|scope| {
                    let mut spread = Spread {
                        scope,
                    };
                    while units.next(&mut spread).get() {}
                });
            }
        }
    )*};
}

threads_over!(One, Two, Probe<Honest>);

fn threaded<S: Cells>() -> [Word; FRAMES]
where
    Threads: StepFn<Plane, S>,
{
    let table = Table::new();
    let bindings = table.bindings();
    let mut store = loaded::<S>(Input::Scramble);
    let mut bound = bound(&bindings, &mut store);
    core::array::from_fn(|f| {
        bound.step(&mut Threads);
        bound.swap();
        digest::<Plane, List, S, _>(&bound, Count::from_host(f)).value()
    })
}

#[test]
fn a_runner_on_threads_equals_the_reference_over_every_store() {
    let reference = digest_stream::<One, _>(&Forward, MORSELS[0]);
    let runs = [threaded::<One>(), threaded::<Two>(), threaded::<Probe<Honest>>()];
    let mut streams: Vec<&[Word]> = vec![&reference];
    streams.extend(runs.iter().map(|r| r.as_slice()));
    assert_eq!(
        morsel_split_and_order(&streams),
        Verdict::Holds(Law::MorselSplitAndOrder)
    );
}
