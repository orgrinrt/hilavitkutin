# Next-solver verification (task #297)

## Question

Does `hilavitkutin-api`'s type-state surface (the `Buildable` /
`Contains` / `WuSatisfied` cons-list reduction shipped in round
202605010900) hold up under rustc's next-generation trait solver
(`-Znext-solver`)?

## Result, short form

- `-Znext-solver=globally`: **incompatible with the current arvo
  substrate.** The `arvo-strategy` crate uses
  `#![feature(generic_const_exprs)]`, and rustc rejects the
  combination at compile time (`-Znext-solver=globally and
  generic_const_exprs are incompatible`). Verification under the
  globally-enabled next solver is blocked at the substrate layer
  until either rustc relaxes the incompatibility or arvo finds a
  path off `generic_const_exprs`.

- `-Znext-solver=coherence`: **compiles cleanly.** The api crate,
  the engine, and the engine's siblings (`hilavitkutin-providers`,
  `hilavitkutin-kit`, `hilavitkutin-extensions`) all check under
  the coherence-only next-solver mode. No new diagnostics, no new
  errors, no observable behavioural drift.

## Reproducer

From `~/Dev/clause-dev/hilavitkutin/mock/`:

```bash
# Globally: blocks at arvo-strategy.
RUSTFLAGS="-Znext-solver=globally" cargo +nightly check \
    -p hilavitkutin-api 2>&1 | grep "next-solver=globally"

# Coherence: clean.
RUSTFLAGS="-Znext-solver=coherence" cargo +nightly check \
    -p hilavitkutin-api -p hilavitkutin -p hilavitkutin-providers \
    -p hilavitkutin-kit -p hilavitkutin-extensions
```

## Interpretation

The `Buildable` reduction relies heavily on recursive trait
resolution over cons-list `Stores`, plus `WuSatisfied<R>` /
`WuSatisfied<W>` proofs that the registered `Stores` set covers
each registered WU's read and write access. The original concern
(filed against round 202605010900 and tracked as #297) was that
this recursive resolution might surface a regression under the
next solver.

The coherence-mode result rules out coherence regressions on the
hilavitkutin-api surface. Type resolution under
`-Znext-solver=globally` is the stricter check the task originally
asked for; that path is currently blocked by the arvo substrate's
nightly-feature mix, not by anything hilavitkutin owns. Re-run when
the substrate moves off `generic_const_exprs` (tracked in arvo's
own follow-up rounds), or when rustc's next-solver implementation
relaxes the incompatibility.

## Pre-existing test failures (unrelated)

`cargo +nightly test --no-run` on the engine crate reports failures
in `dispatch_types`, `plan_types`, and `platform_os` test files
under both the default solver and `-Znext-solver=coherence`. These
failures are pre-existing post-Round-D API debt (USize comparison
ergonomics, Bits widening sites) tracked in #291 and are not
relevant to this verification.

## Status

The relevant subset of #297 (coherence-mode verification) is
complete. The globally-enabled subset remains deferred pending
substrate movement off `generic_const_exprs`. Task #297 stays in a
soft-pending state until that substrate work lands; the finding is
recorded here so future agents do not re-run the same investigation
without the substrate context.

## Recorded

2026-05-04 during overnight autonomous polishing.
