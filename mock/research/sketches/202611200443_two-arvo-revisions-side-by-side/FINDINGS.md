# Sketch findings: cargo resolves two arvo revisions side by side in one workspace

**Date:** 2026-11-20
**Outcome:** WORKS
**Unblocks:** the port plan's job 2 to 5 may land one at a time, each hilavitkutin crate taking current arvo on `nightly-2026-05-28` while its neighbours still take the old revision. The plan's fallback (one long branch `work/port-hilavitkutin`) does not apply.

## Hypothesis

The port plan (`mock/research/202611200345_arvo-port-plan/README.md`, "What was not checked") did not know whether cargo takes the old arvo at `1b2796e9` and arvo `dev` at one pinned revision in one workspace under renamed dependency keys, and whether `dev` builds beside the old tree on `nightly-2026-05-28`. The question is settled by a workspace with one crate on each, built and tested by one `cargo test`.

Arvo `dev` was pinned at `6def883183ebe9b96c90da16cee148010eb89226`.

## What was tried, in order

The workspace has two members. `old-side` takes the old tree through the key `arvo-old` (`package = "arvo"`, rev `1b2796e9`), writes `arvo::USize(code)` and reads `.0`, as `hilavitkutin-linking/src/error.rs` does today. `new-side` takes `arvo-numeral` and `arvo-format` at `6def8831` through the keys `arvo-new-numeral` and `arvo-new-format`, declares `Code = USize<Adapt<HalfEven, Wrap>>` and enters and reads the code by `from_host` and `host`. Each crate has one test, and the manifests use the `ssh` spelling of the arvo url that hilavitkutin's `mock/Cargo.toml` uses.

First run, with the `[patch."ssh://git@github.com/orgrinrt/notko.git"]` section that hilavitkutin's `mock/Cargo.toml` carries:

```
   Compiling arvo v0.1.0 (ssh://git@github.com/orgrinrt/arvo.git?rev=1b2796e92ad2b8176c2177d72a9cffa161b7b92e#1b2796e9)
   Compiling arvo-numeral v0.0.0 (ssh://git@github.com/orgrinrt/arvo.git?rev=6def883183ebe9b96c90da16cee148010eb89226#6def8831)
   Compiling new-side v0.0.0 (...)
   Compiling old-side v0.0.0 (...)
    Finished `test` profile [unoptimized] target(s) in 14.32s
test tests::the_new_usize_carries_a_code_through_its_doors ... ok
test tests::the_old_usize_carries_a_code_through_its_field ... ok
```

Second run, with the patch section removed, which is the committed manifest. The lock then holds two `notko` entries at one commit, `git+ssh://git@github.com/orgrinrt/notko.git?branch=dev#33593901` and `git+https://github.com/orgrinrt/notko.git?branch=dev#33593901`, since the old revision names it by `ssh` and the new one by `https`:

```
Locking 1 package to latest compatible version
Adding notko v0.0.1 (ssh://git@github.com/orgrinrt/notko.git?branch=dev#33593901)
    Finished `test` profile [unoptimized] target(s) in 8.32s
test tests::the_new_usize_carries_a_code_through_its_doors ... ok
test tests::the_old_usize_carries_a_code_through_its_field ... ok
```

Control, to show the test can fail: `new-side`'s second assertion was changed to expect 1315 for the code 1314 and the run went red,

```
assertion `left == right` failed
  left: 1314
 right: 1315
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

and the assertion was put back and the run went green again.

## The result

One workspace under `nightly-2026-05-28` resolves, builds and runs a crate on `arvo::USize` of the old tree and a crate on `arvo_numeral::USize` of arvo `dev` `6def8831`, with no `[patch]` section. The renamed keys are enough, because the two revisions are two cargo sources. Both runs above are on the toolchain the repository pins, resolved from the working directory.

Where hilavitkutin's workspace keeps the patch for the `ssh` spelling of notko, the two sides still build with it (first run), so the patch can stay until the port's last job.

## What is NOT established

No value of a type of one side was passed to the other, and the two `USize` are not expected to unify, so a crate that takes both has to convert through the host integer, as the plan's private conversion modules do. The two notko entries in the lock are two copies of one commit, so a notko type of one is not a notko type of the other. That holds today because the old arvo's notko types do not cross into crates that take the new arvo. A crate taking both revisions through a notko type in its own signature would need the patch, and that was not built here.

Only `arvo-numeral` and its dependencies `arvo-format`, `arvo-contracts` and `arvo-placement` of arvo `dev` were built. `arvo-capacity`, `arvo-bits` and `arvo-digest` of `dev` were not built beside the old tree on this toolchain. The old tree has crates named `arvo-bits` and `arvo-strategy`, as `dev` does, and a crate taking either name from both revisions was not built, so that case is open.
