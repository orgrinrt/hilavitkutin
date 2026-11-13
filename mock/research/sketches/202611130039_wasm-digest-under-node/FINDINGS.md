# Sketch findings: a no_std loop with a growable model runs under node through a fixed-capacity array, and gives the native digests

Date: 2026-11-13

Outcome: WORKS. The fixed-capacity route runs under node 22 and prints the same 1701 lines as the native binary. The negative control, a digest that absorbs the line length as a pointer-width integer, is red on the first line.

Unblocks: a `#![no_std]`, no-alloc reference kernel with a growable model can have its wasm32 digests checked against native ones by a script that exits non-zero on the first differing line. The alloc route is closed for hilavitkutin crates and is only a note below.

## Hypothesis

A `no_std` kernel with one integer loop gives the same digest after every step on `x86_64-unknown-linux-gnu` and on `wasm32-unknown-unknown` under node, provided nothing in a digested value depends on the width of a pointer. A growable model fits in a fixed-capacity array, and a full array refuses with a value instead of truncating. What would settle it: native and node printing line-identical output, and a variant that does absorb a pointer-width value turning the comparison red.

## What was tried, in order

Environment, as run: `rustc 1.98.0-nightly (57d06900f 2026-05-27)` under the repository's pin, node `v22.23.4` at `/opt/node22/bin/node`. The target was already installed, so none was added: `rustup target list --installed` printed `wasm32-unknown-unknown` and `x86_64-unknown-linux-gnu`.

The kernel (`kernel/src/lib.rs`, `#![no_std]`, `#![forbid(unsafe_code)]`, no dependency) holds the whole pass. The digest function, in full:

1. `acc` starts at `0xCBF29CE484222325`.
2. The length of the line enters first, as a 64-bit integer: `acc = fold(acc, len as u64)`.
3. Then every cell, by ascending index, as its two's complement 64-bit value: `acc = fold(acc, cell as u64)`.
4. `fold(acc, x)` is `let m = ((acc ^ x) as u128) * 0x9E3779B97F4A7C15u128; (m as u64) ^ ((m >> 64) as u64)`.

The pass is one three-tap weighted sum per step over `i64` cells: a Q16 weight triple `[0x9000, -0x4000, 0x9000]` (summing to 0.875, with a gain of 1.375 on a line that alternates in sign) applied to left, middle and right cell, summed as `i128`, shifted down by 16 with the remainder rounded half to even, then saturated to the `i64` range. The two end cells repeat themselves as their missing neighbour. Each step starts with a small wrapping nudge to one cell (`s % len`), so the line does not settle into a saturated cycle. The loop runs 1000 steps and keeps one digest per step.

The growable model: the line starts with 54 cells and gains one every 100 steps. Case 0 holds it in an array of 64, which fills at step 1000 and never overflows. Case 1 holds the same run in an array of 60, which is full when step 700 asks for a 61st cell. The refusal is the value `Refusal::Cells { step: 700, needed: 61, cap: 60 }`, the 699 digests before it are kept, and no cell is dropped. Both front ends print it as `c1 end written=699 refusal=cells step=700 needed=61 cap=60`.

Front ends, both thin:

1. `native/` is a binary that prints `c<case> s<step> <16 hex digits>` per digest and one `c<case> end ...` line per case.
2. `wasm/` is a `cdylib` for `wasm32-unknown-unknown`. It exports `run_case_export`, `out_ptr` and `status_ptr`, and leaves the digests and the report in two statics. `run.mjs` loads the module in node with an empty import object, reads the statics with a `DataView`, and prints the same lines.

`run.sh` builds both, runs both, and compares line by line.

```
$ ./run.sh
SAME: 1701 lines, native and node agree
```

That is 1001 lines for case 0 and 700 for case 1. The expected lines are committed as `digests-native.txt`; `output-main.txt` is the run above with its exit code. Since the review, `run.sh` also diffs its lines against `digests-native.txt` and fails on an empty output, so two front ends that drifted together or printed nothing are red too. The drift check was shown failing by changing the last digit of line 5 of `digests-native.txt` by hand and restoring it; `output-golden-drift.txt` holds that run, and the hand edit itself is not committed.

The kernel's own tests pin ties to even in both signs, saturation, order and length sensitivity of the digest, the refusal of a full array with its cells kept, the case 1 stop at step 700, the refusal of a short digest buffer, and that the main case does reach a saturated cell. `probes.sh` runs them and records the output in `output-tests.txt`: 7 passed, 0 failed, exit 0.

To show the tie test can fail, the kernel has a feature `mutant-half-up` that rounds ties up (`let up = r >= half;`) instead of to even. `probes.sh` runs the same tests under it and records `output-mutant-half-up.txt`:

```
test tests::ties_round_to_even_in_both_signs ... FAILED
assertion `left == right` failed: 0.5 goes to 0
  left: 1
 right: 0
test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
exit=101
```

## The negative control

`./run.sh --control` builds both front ends with the kernel feature `control-usize-len`. The only change is the length entering the digest as `len.to_ne_bytes()`, one byte per fold, instead of `len as u64`. A `usize` is 8 bytes on the native target and 4 on wasm32.

```
$ ./run.sh --control
DIFFER at line 1
  native: c0 s1 92a39b69ea5ef7a3
  node:   c0 s1 924f4fd6e3e01fe1
exit=1
```

The output is committed as `output-control.txt`. The script is red on the first line, and green again without the feature.

## What the cdylib front end needed

The pair below differs in the offending token only: the feature `omit-panic-handler` removes the `#[panic_handler]` function. The failing half is recorded in `output-omit-panic-handler.txt`:

```
$ cargo build --release -p wasm-digest-wasm --target wasm32-unknown-unknown --features omit-panic-handler
error: `#[panic_handler]` function required, but not found
error: could not compile `wasm-digest-wasm` (lib) due to 1 previous error
exit=101
```

With the feature off the same command builds, which is the module `output-wasm.txt` measures. The handler body is `core::arch::wasm32::unreachable()`.

Panic strategy: the workspace profile sets `panic = "abort"`. Overriding it with `--config 'profile.release.panic="unwind"'` also built, recorded in `output-panic-unwind.txt` (`Finished release profile`, `exit=0`), so nothing here depends on the setting on this target. Only the build was checked, not what a panic does at run time.

Exports: `#[unsafe(no_mangle)] pub extern "C" fn` for the three functions. The module's exports, read by node (`inspect.mjs`, output in `output-wasm.txt`), are `memory`, `out_ptr`, `run_case_export` and `status_ptr`, and its imports are an empty list. No allocator, no `main`, no `#![no_main]`, no linker flag. The two statics are `static mut` arrays written through raw pointers (`addr_of_mut!` and `copy_nonoverlapping`), which is the only `unsafe` in the sketch; the kernel stays safe code. A bare `cargo build` of the workspace would try to build this `no_std` `cdylib` for the host, so `default-members` leaves it out.

The `.wasm` is 3157 bytes (`bytes: 3157` in `output-wasm.txt`, release profile, fat LTO, one codegen unit, stripped).

## Does a u128 intermediate compile on wasm32, and agree

Yes to both, for this kernel. The product of the tap is an `i128` and the digest fold is a `u128` multiply. Both compiled for `wasm32-unknown-unknown`, and the 1701 lines from node equal the native ones, so the widened product, the half to even rounding and the 128 bit fold agree on both targets.

## The result

A `no_std` loop with a growable model runs under node 22 through a fixed-capacity array with no allocator, and its digest after each of 1000 steps equals the native one, line for line. A full array refuses with a value that both front ends print identically. The comparison script is able to fail, shown by the control.

## What is NOT established

1. The alloc route was not built or run. Noted only: it needs an allocator in the wasm front end (a `#[global_allocator]` over a static arena, or a bump allocator), and `mockspace.toml` forbids `std` and `alloc` for every `hilavitkutin*` crate, which the reference crate will be. It would give a growable list without a capacity, at the cost of a failure path inside the allocator instead of a value at the call.
2. Only one node version (22.23.4) and one native target (`x86_64-unknown-linux-gnu`) were run. A big-endian target, or a node that lacks 64-bit `BigInt`, was not tried.
3. The kernel uses raw primitives (`i64`, `u64`, `usize`, `u128`). It is a sketch under `mock/research/sketches/` and not a stack crate; a reference crate would take arvo types there.
4. A pointer-width value that does not enter a digest was not examined, for example an index used only inside the loop. The control shows that one entering the digest is caught.
5. The cell values are one seeded stream. Other seeds, other lengths, other weights and other step counts were not run.
6. Nothing here says anything about speed.
