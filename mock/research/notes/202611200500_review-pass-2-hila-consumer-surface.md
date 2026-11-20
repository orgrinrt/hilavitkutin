# Review pass 2: `work/hila-consumer-surface` at c09e95ac

Canon gate: MISALIGNED. The canon gate stopped the review, so the ordinary review (unsafe, orderings, test-gate audit, public surface, commit messages, asm gate check 3) was not done. This is the second independent canon read. I wrote it without seeing the first, and the reconciling is owed by the dispatcher. If the two reads disagree, the call goes to op and not to a third seat.

## Verdict

DO NOT MERGE. Grounds 1 and 2 below each put the branch against the canon. Neither is a code defect that a third review pass can fix.

## Canon gate

Asked with `cargo mock canon check`, `related`, `find` and `already-said` against `canon_paths = ["mock/registry/*.toml", "mock/registry/tenet/*.toml"]`. The branch edits no tenet file (`git diff --stat origin/dev...HEAD -- mock/registry/tenet` is empty), so every row below binds on the branch tree exactly as it does on `dev`.

### 1. The branch deletes the design text a stamped row quotes, and states the opposite

Row `the_arena_column_storage_reserves_64_byte_aligned_blocks_through_the_memory_provider_and_its_release_is_a_no_op` (`mock/registry/tenet/memory-2.toml:4`) is provisional and stamped by the maintainer. Its `source` is `mock/crates/hilavitkutin-providers/DESIGN.md.tmpl:165-175`. Its quote:

> `release` is a no-op (the reader-count reclamation is a columnar-engine concern).

Under `mockspace.toml:524`, a provisional row "binds", and "a stamp is as good as the maintainer's word for now". The yszg doc changelist, item 2, deletes exactly that sentence. It is the `-` hunk in the providers design diff. In its place, `mock/crates/hilavitkutin-providers/DESIGN.md.tmpl:195` now reads "`release` gives the slot's block back through the provider". `crates/hilavitkutin-providers/src/storage.rs:135-137` ships that as `self.give_back(id)`.

Nothing on the branch addresses the row:
- no supersession;
- no new row;
- no question ticket to op;
- neither topic names it. The yszg topic's canon line (`202611200201-yszg_topic...md`, "Canon:" paragraph) lists five rows and leaves this one out.

The only licence cited is the workspace ticket `arenacolumnstorage_release_is_a_no_op_so_a_store_under_a_bum`. That ticket is `owner = "agent"` (`.shared/scripts/life/tickets-hilavitkutin.toml:3181-3186`), and an agent's ticket is not a ruling. The registry now asserts, at a binding standing, a design sentence that no longer exists. Per the chain, a design disagreeing with the tier above it is the design being wrong. It is not a design change the code may follow.

This is the textbook undeclared upward mutation: the leaf (loru's need) rewrote the design, and the canon row was left behind. A stamped row does not get overruled by deleting the paragraph it was lifted from. The route is a registry change superseding the row, with op asked by question ticket, or stamped by the provisional-stamp process while he is away. That change comes before the design, never after.

### 2. `StaticFreeListMemory` is a no_os platform implementation shipped inside the family

`the_no_os_tier_is_trait_contracts_only_and_the_consumer_provides_every_platform_implementation` (ratified, `mock/registry/tenet/memory.toml:37`):

> `no_os` = trait contracts only; consumer provides all platform impls.

The yszg topic states the branch's purpose in its own words: `StaticBumpMemory` is "the one memory provider the stack ships for a consumer with no operating system", and the round adds a second one. `crates/hilavitkutin-providers/src/memory/free_list.rs:48`, `:188` is a `MemoryProviderApi` implementation for the no_os tier, with an allocator algorithm and a lock. It is authored by the stack, not by the consumer.

The one ratified ruling op made on where a platform implementation lives is `op_ruled_that_the_consumer_supplies_the_os_thread_executor_and_the_engine_ships_only_the_executor_contract` (ratified, `memory-2.toml:143`). It puts the implementation "in the consumer, or in a crate outside the hilavitkutin* family". `hilavitkutin-providers` is inside the family.

The licence the topic leans on does not say what the topic needs:
- `clock_and_libc_unix_memory_provider_defaults_land_in_their_own_rounds_when_consumer_demand_surfaces` (provisional, `future.toml:380`) reads "The providers crate's scope at this stage is the interner and the arena column storage backing. The clock default and the memory provider default for libc and Unix each land in their own round". That licenses a libc/Unix memory default and nothing else. A free-list allocator over a consumer's buffer for targets with no OS is neither.
- `later_modules_of_the_providers_crate_add_column_storage_clock_and_memory_defaults_per_consumer_demand` is `held`, and binds nothing (`mockspace.toml:524`).

The spin lock deepens it. The canon puts cross-thread guarding of the stack's inline stores on the consumer: `the_memory_arena_is_not_sync_by_default_and_multi_thread_access_needs_a_consumer_mutex_shim_or_a_future_sync_arena` (provisional, `future.toml:360`). The branch has the stack keep its own lock (`free_list.rs:52` FIXME, `:124`, `:170-173`), and tickets it as a gap rather than asking whether it should exist.

To read this as aligned, "consumer provides all platform impls" has to mean "consumer picks an opt-in crate of the family". Neither row says that, and op's executor ruling refused exactly that reading for threads.

The same defect already sits on `dev` as `StaticBumpMemory` (landed 266650fb). Its round, `202611140512_topic.sound-memory-provider.md:39`, cited the same held row and the same libc/Unix row. Pre-existing drift is not a licence. The branch doubles it, and adds a lock.

Rows read and found consistent with the branch, for the record:
- `every_crate_is_no_std_with_no_allocation_and_the_consumer_provides_the_memory_provider` (ratified): no_std and no alloc hold, and the buffer is consumer-given.
- `a_column_s_lifetime_is_told_to_the_consumer_as_an_advisory_release` (ratified): release stays advisory to the consumer, and the conflict is with the stamped row in ground 1, not with this one.
- `hilavitkutin_does_not_use_the_alloc_crate_and_dynamic_storage_goes_through_slab_columns_or_stack_arrays` (ratified).
- `the_memory_arena_is_an_inline_storage_interner_with_const_generic_byte_and_entry_caps_and_overflow_returns_a_sentinel_id` (provisional): the arena still answers the sentinel, and refusing it as `Maybe::Isnt` one level up, in `StringInterner`, contradicts nothing. The 2^28 bound keeps issued ids inside the 28-bit field.
- `container_capacity_lives_at_type_level_and_a_consumer_needing_more_picks_the_bigger_type` (provisional): not crossed.

Open for the second read, not decided here: `the_platform_contracts_are_memory_provider_thread_pool_and_clock_traits_in_the_api_crate` (ratified) names a provider that "allocates, deallocates and protects". `allocate_zeroed` and a fifth promise extend a ratified contract the canon is silent on. Silence is not permission. I did not settle whether a provided method counts as an append or a change.

## Findings

1. `mock/crates/hilavitkutin-providers/DESIGN.md.tmpl:195` and `crates/hilavitkutin-providers/src/storage.rs:135` [drift, blocker]: they contradict the stamped row `the_arena_column_storage_..._release_is_a_no_op`, which still binds and was neither superseded nor asked of op. The registry change comes first, through op or a proper stamp. Until then the design and the code under it are wrong.
2. `crates/hilavitkutin-providers/src/memory/free_list.rs` (whole file) and `DESIGN.md.tmpl:265-320` [drift, blocker]: a no_os platform implementation inside the family, against ratified `the_no_os_tier_is_trait_contracts_only_...` read with op's ratified locus ruling `op_ruled_that_the_consumer_supplies_the_os_thread_executor_...`. Its existence and its locus are a question for op (`--holder op`). It does not stand on an agent's reading of a held row.
3. `mock/crates/hilavitkutin-api/DESIGN.md.tmpl` `allocate_zeroed` [drift, needs the second read]: it extends a ratified three-method contract. Say whether that is an append before it lands.
4. Tooling gap [follow-up]: no lint or `canon check` noticed that a binding row's `source` text was deleted from the design it cites. `canon: read 853, nothing to report.` came back on this branch. That is a gotcha that belongs in a lint. File it, since this seat is read-only.
5. Process [follow-up]: open PR #4 (`recover/unlanded-rounds-and-benches` into `dev`, "land the recovered rounds, benches and the scheduler rewrite") is outside this set. It touches the engine crate this branch also edits (`crates/hilavitkutin/src/platform/os.rs`). The brief names round `202611200133-yszg`, and the tree has `202611200201-yszg`.

## Notes

Not done, because the gate refused: the unsafe and SAFETY audit of `memory/` and `storage.rs`, the orderings, the test-gate read of `static_free_list_memory.rs`, `column_storage.rs`, `interner_law.rs` and the str and persistence tests, the public surface, the commit-message sweep, and asm gate check 3. No suite was run. Once the canon question is answered, a fresh pass owes all of it.

Two rounds and a review pass went into polishing an allocator and a release semantics that never checked the one row that quoted the paragraph it was deleting. The loru demand is real. The answer to it was op's to give, or a stamp's, and the branch took it anyway.
