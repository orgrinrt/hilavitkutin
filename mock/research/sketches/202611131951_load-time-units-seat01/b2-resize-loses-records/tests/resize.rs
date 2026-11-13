//! What the trunk's "resize" keeps.
//!
//! `growth_keeps_the_records_already_written` is the case that must hold for
//! `reserve` to be a growth operation, and it is expected RED against the
//! trunk: `ArenaColumnStorage::reserve` frees the prior block before it
//! allocates the new one (hilavitkutin-providers/src/storage.rs:68-77), so a
//! resize is a discard and a fresh reservation.
//!
//! `growth_moves_the_base_pointer` records the second half: the base moves,
//! so any pointer the scheduler recorded at build (resource bindings,
//! hilavitkutin/src/resource/bindings) dangles after a resize.

use arvo::USize;
use b2_resize_loses_records::ZeroedHeap;
use hilavitkutin_api::{ColumnStorage, StoreId};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

fn write_ramp(store: &ArenaColumnStorage<ZeroedHeap>, id: StoreId, n: usize) {
    let base = unsafe { store.column_ptr_mut::<u64>(id) };
    for i in 0 .. n {
        unsafe { base.add(i).write(1000 + i as u64) };
    }
}

fn read(store: &ArenaColumnStorage<ZeroedHeap>, id: StoreId, n: usize) -> Vec<u64> {
    let base = unsafe { store.column_ptr::<u64>(id) };
    (0 .. n).map(|i| unsafe { base.add(i).read() }).collect()
}

#[test]
fn growth_keeps_the_records_already_written() {
    let mut store: ArenaColumnStorage<ZeroedHeap> = ArenaColumnStorage::new(ZeroedHeap);
    let id = StoreId(USize(0));
    assert!(matches!(
        store.reserve::<u64>(id, USize(8)),
        Outcome::Ok(())
    ));
    write_ramp(&store, id, 8);
    assert_eq!(
        read(&store, id, 8),
        (0 .. 8).map(|i| 1000 + i).collect::<Vec<u64>>()
    );

    // Grow from 8 to 16 records: the contract calls this a resize.
    assert!(matches!(
        store.reserve::<u64>(id, USize(16)),
        Outcome::Ok(())
    ));
    assert_eq!(store.count(id), USize(16));

    // A growth keeps the first 8 records. Expected red on the trunk.
    assert_eq!(
        read(&store, id, 8),
        (0 .. 8).map(|i| 1000 + i).collect::<Vec<u64>>(),
        "a resize through ColumnStorage::reserve kept none of the records written before it"
    );
}

#[test]
fn growth_moves_the_base_pointer() {
    let mut store: ArenaColumnStorage<ZeroedHeap> = ArenaColumnStorage::new(ZeroedHeap);
    let id = StoreId(USize(0));
    assert!(matches!(
        store.reserve::<u64>(id, USize(8)),
        Outcome::Ok(())
    ));
    let before = unsafe { store.column_ptr::<u64>(id) } as usize;
    // Hold a second allocation live so the allocator cannot hand the freed
    // block straight back at the same address and hide the move.
    let other = StoreId(USize(1));
    assert!(matches!(
        store.reserve::<u64>(other, USize(8)),
        Outcome::Ok(())
    ));
    assert!(matches!(
        store.reserve::<u64>(id, USize(4096)),
        Outcome::Ok(())
    ));
    let after = unsafe { store.column_ptr::<u64>(id) } as usize;
    assert_ne!(before, after, "the base pointer of a grown column moved");
}
