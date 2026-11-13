# m3: dropping a library while its occupant is installed

Spike, seat 01 reply, round 202611131951. Written to decide between merged call M3's options a and b
in `02_tiark-rompf.md` on the one axis seat 02 named for it: "whether unload safety belongs in the type
or in the host's discipline, shown by a spike that drops a library while its occupant is installed".
Uses the shipped `hilavitkutin-api`, `hilavitkutin`, `hilavitkutin-providers` and
`hilavitkutin-linking` at `ec13bc97`, nightly-2026-05-28, x86_64 linux glibc. One instance; nothing here
is a bench and no timing was taken.

The occupant is seat 02's s6 shape (a C-ABI kernel pointer held in an ordinary `Resource`, read by a
compiled host unit that calls it once a morsel), with one change: it is `Replaceable` and swapped with
`replace_value`, not `PlanAffecting`. The extension is a real cdylib (`ext/`), mapped so that the shipped
`Library` holds the only reference, so dropping the `Library` is the last `dlclose` and the image is
unmapped; each test checks `/proc/self/maps` to confirm it.

| Test | Result | What it says |
|---|---|---|
| `tests/unload.rs` `every_frame_after_the_drop_is_sound_with_the_occupant_left_installed` | red, on purpose | Library loaded after build, kernel installed, frame 2 runs it on all 20 000 records. The `Library` is dropped and the image unmapped with the occupant still installed. Frame 3 (clean) survives: the engine skips the host because none of its inputs changed (host runs 14 to 14). Frame 4, after `mark_dirty::<Column<A>>`, kills the child with signal 11. The hazard is latent until anything reruns the host. |
| `tests/unload.rs` `the_installer_swaps_before_it_drops_and_every_frame_after_is_sound` | green | A host-side owner (`Installed`, `src/lib.rs`) whose only release path hands a fallback occupant to the caller's swap first and drops the `Library` after. Image unmapped; frames 3 and 4 run the fallback on every record. |
| `tests/unload.rs` `an_installer_dropped_without_uninstall_keeps_the_image_and_stays_sound` | green | The owner's other exit: dropped without `uninstall`, it leaks the mapping. Image still mapped; a dirty frame runs the extension's kernel on every record. Leak, never dangle. |
| `tests/unload.rs` `an_occupant_swap_needs_a_dirty_seed_and_no_plan_band` | green | A `Replaceable` occupant swapped with `replace_value` reruns the host (7 to 14) and the next frame's records are the new kernel's, with the plan band shut. A kernel swap changes no plan input. |
| `tests/borrow.rs` `an_occupant_borrowing_its_library_is_a_resource` | does not compile | An occupant carrying `'lib` (as `Symbol<'lib, T>` does) registered as a resource: "argument requires that `'lib` must outlive `'static`". The type-level tie has no form on the trunk's store path. |
| `tests/owns.rs` `an_occupant_owning_its_library_is_a_resource` | does not compile | An occupant that owns its `Library` cannot be `Copy`, and `build` requires `Copy` of every staged resource (`OwningOccupant: Copy` required by `DrainStores`). An owning store kind is a store kind the trunk does not have. |

What it does not show: `run_parallel` (the swap takes `&mut self`, so no frame is in flight; the
parked-worker aliasing is ticketed at `hilavitkutin/src/scheduler/dirty.rs:62-65`); several occupants
per unit (a bin); a library dropped from another thread; macOS or Windows.

The closure-shaped `uninstall` is scaffolding: it takes the caller's swap as `FnOnce(Occupant)` because
naming the scheduler's full type in a spike is noise. The perimeter of its guarantee is the copy the
scheduler holds; `Occupant` is `Copy` with a public field, so any other copy the host keeps is outside
it. `Symbol::get` returns the bare function pointer by value with no lifetime
(`hilavitkutin-linking/src/symbol.rs:38-46`), which is how a kernel leaves the borrow that
`Symbol<'lib, T>` was meant to enforce; the crate's doc says the lifetime prevents "use-after-close at
compile time" (`:21-22`).

Run (copy `mock/Cargo.lock` beside the workspace `Cargo.toml` first):

    cargo +nightly-2026-05-28 build -p m3-ext
    cargo +nightly-2026-05-28 test -p m3-host --test unload --no-fail-fast -- --test-threads=1 --nocapture
    cargo +nightly-2026-05-28 test -p m3-host --test borrow
    cargo +nightly-2026-05-28 test -p m3-host --test owns

Outputs: `OUTPUT-unload.txt`, `OUTPUT-borrow-does-not-compile.txt`, `OUTPUT-owns-does-not-compile.txt`.
