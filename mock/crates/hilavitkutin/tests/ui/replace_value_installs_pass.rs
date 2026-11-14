// `Scheduler::replace_value` as a consumer writes it: the value type opts into
// `Replaceable`, the registered value is replaced, a frame runs and the new
// value is read back. The methods take the value type, as the kit design opts
// in, so the consumer names a value to install and writes the impl on its own
// type. It runs from `a_replaced_value_is_read_by_the_next_run` in
// `tests/ui.rs`. While the methods took the store marker `Resource<InA>` this
// file did not compile (ticket `147e3897011f`).

use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::store::{Replaceable, Resource};
use hilavitkutin_providers::{ArenaColumnStorage, StaticBumpMemory};
use notko::Outcome;

#[derive(Copy, Clone)]
struct InA(u32);

impl hilavitkutin_api::footprint::ResourceFootprint for InA {
    const L1_BYTES: USize = USize(0);
}

impl Replaceable for InA {}

fn store(provider: StaticBumpMemory) -> ArenaColumnStorage<StaticBumpMemory> {
    ArenaColumnStorage::new(provider)
}

fn main() {
    let provider = StaticBumpMemory::new(std::vec![MaybeUninit::new(0); 8192].leak());
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(InA(1)))
        .build(store(provider), USize(4))
        .unwrap_or_else(|_| panic!("build should succeed"));
    scheduler.replace_value(InA(2));
    // No unit is registered, so nothing names the projection witness list;
    // the empty carrier accepts any, and `()` is the one spelled here.
    assert!(matches!(scheduler.run::<(), _>(), Outcome::Ok(())));
    // SAFETY: the one registered resource is the bindings head, and its blob
    // holds an initialised `InA`.
    let read: InA = unsafe { *scheduler.__bindings().__ptr().as_ptr() };
    assert_eq!(read.0, 2, "the next run reads the replaced value");
}
