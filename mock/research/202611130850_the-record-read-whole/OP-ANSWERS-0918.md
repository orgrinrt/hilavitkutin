# What op answered on 18 September 2026 on the std tier and persistence

This note holds the two questions put to the maintainer on 18 September 2026
about where hilavitkutin's OS-thread code lives and what hilavitkutin-persistence
builds on, each with the options it was asked with and the answer he gave. The
text below the line is copied line for line from the capture kept in the
workspace repository, `.data/op-responses/202609181645_hila-std-tier-persistence.md`,
lines 11 to 43, and nothing in it is reworded. The registry's rows that rest on
these answers cite this note, so that a clone of hilavitkutin on its own can
open them.

---

# Std tier and persistence deps in hilavitkutin's design round

## What was going on

A design round moving hilavitkutin onto arvo's current dev was blocked by hilavitkutin's own lint gate, on conflicts found inside hilavitkutin's own design. Two conflicts were put to op.

## Asked

> hilavitkutin's engine has an OS-thread tier (`platform/std_tier.rs`: `#![no_std]` plus `extern crate std`, and `thread::spawn`/`OnceLock`), which the workspace's no-extern-crate-std rule forbids. Its lint config says every `hilavitkutin*` crate is no_std and no_alloc. Where does the code that spawns OS threads live?

Options offered:

1. **Consumer supplies it.** hilavitkutin ships only the executor contract (a trait). The OS-thread executor lives in the consumer, or in a crate outside the hilavitkutin* family, gated per target and feature.
2. **hilavitkutin-platform-std crate.** A separate hilavitkutin crate that openly needs std, not claiming no_std, and exempted from rule-0 in the lint config by name. The engine stays no_std.
3. **Both, as arms.** The contract in the engine, plus a first-party std crate as one arm; a consumer may bring its own instead.

## Answered

> Consumer supplies it

## Asked

> `hilavitkutin-persistence`: its DESIGN lists `hilavitkutin-str` and a shared `MemoryProvider` from the api as dependencies, but the lint config (rule-4) says persistence is a standalone crate and forbids importing `hilavitkutin_str` and `hilavitkutin_api`. Which one holds?

Options offered:

1. **Standalone, lint holds.** Persistence depends on arvo and notko only; the str and MemoryProvider uses move behind its own traits and the DESIGN is corrected.
2. **Builds on str and api.** The DESIGN holds; rule-4's scope for persistence is corrected to allow `hilavitkutin_str` and `hilavitkutin_api`, still forbidding the engine.
3. **Str yes, api no.** Persistence may use hilavitkutin-str (a sibling ecosystem crate) but not the api; MemoryProvider comes through its own trait.

## Answered

> Builds on str and api
