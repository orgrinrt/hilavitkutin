# s4: growth against the trunk's storage and os-tier provider

Spike, seat 02, round 202611131951. Uses the shipped `hilavitkutin-api`, `hilavitkutin`
(`OsMemoryProvider`) and `hilavitkutin-providers` (`ArenaColumnStorage`) at `271909e5`.

| Test | Result | What it says |
|---|---|---|
| `growth.rs` `re_reserving_a_column_keeps_the_records_it_held` | red | the contract says "Re-reserving an existing `id` resizes it" (`hilavitkutin-api/src/storage.rs:46-47`); the shipped store frees and maps afresh (`hilavitkutin-providers/src/storage.rs:68-80`), so four written records read back `[0, 0, 0, 0]`. The base pointer came back at the same address, so a context still holding it reads zeros rather than faulting: silent loss |
| `growth.rs` `a_capacity_far_above_the_live_count_costs_address_space_not_memory` | red | the shipped provider maps `PROT_READ | PROT_WRITE` with no `MAP_NORESERVE` (`hilavitkutin/src/platform/os.rs:47-56`); under `overcommit_memory = 0` on a 16 GiB, no-swap host a 32 GiB capacity is refused |
| `reserve_commit.rs` `a_reservation_committed_as_it_grows_keeps_its_base_and_its_records` | green | the record's mechanism, reserve with `PROT_NONE | MAP_NORESERVE` and commit each pass's growth with `mprotect`: 32 GiB reserved, one million records grown in sixteen passes, resident set grew 8.0 MiB for 8 MiB touched, base unchanged, records intact. This is what the os tier's `protect` would do; its body is empty under a FIXME (`os.rs:80-83`) |
| `reserve_commit.rs` `the_shipped_provider_grants_a_capacity_under_physical_memory_and_touch_decides_residency` | green | a 4 GiB capacity through the shipped provider: granted, resident set unchanged by the reservation, grows only by what is touched |

What it does not show: anything on a target without virtual memory (wasm32, no_os), where
the record's fallback is linked pages (loimu storage research `:61-69`) and where several
columns cannot all grow contiguously in place. No timing; nothing here is a bench.

Run: `env -C <this dir> CARGO_INCREMENTAL=0 cargo test --no-fail-fast -- --nocapture --test-threads=1`.
`OUTPUT-tests.txt` is the run.
