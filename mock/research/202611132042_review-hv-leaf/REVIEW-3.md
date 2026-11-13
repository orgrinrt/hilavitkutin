# Review of work/hv-leaf at cd6922e1, round 3

Reviewer: the round 2 reviewer, resumed to check its own findings against commits eb71f2e4..cd6922e1. The report is kept whole, as returned. This is the last of three rounds; what it leaves open is ticketed, and the branch lands with it.

## Verdict

MERGE AFTER FIXES. One defect is left, and it is a single text sweep: finding 4 was fixed in two places and left standing in five others. Nothing else blocks. If the branch lands instead, the sweep is ticketed.

## Findings

1. Finding 4 is only half fixed [spec/drift]. The design now says the section has no leading dot (`str/DESIGN.md.tmpl:106-108`), and `entry.rs:9` was fixed. But `.hilavitkutin_strings` still appears in five places: `mock/crates/hilavitkutin-str/DESIGN.md.tmpl:123` and `:170`, which render to `docs/STR.md:132,179`; `src/macros.rs:3`, the rustdoc of the public `str_const!`; `src/section.rs:1` and `:20`. `BACKLOG.md.tmpl:23` still names `getsectiondata` for macOS. So the design now contradicts itself. Fix: one commit replacing all six, then regenerate the docs. Ticket `8eb14617754f`.

2. The removed trampoline row describes a gap that still exists [drift, minor]. `mock/catalogue.md` no longer has the row "trampoline survival under fat LTO is unchecked off unix". The changelist (`202611132059_changelist.src.lock.md:23`) is accurate that the symbol reader now exists and that no ignored test names the row. But the Mach-O and PE reads have still never run, so the check is still effectively missing off unix. That gap now lives only in ticket `2529323efd1f`, which no file in the repo names except the round topic. Acceptable as is; the ledger no longer records it.

3. One new subject repeats finding 9 of round 2 [rules, minor]. `89b87d4b` "docs: open the second review fixes round with its design text" says how the change came about rather than what changed. The other five subjects and the `3b5771de` body are clean, with no bylines and no tool adverts.

## Status of the round 2 findings 1 to 10

1. Fixed. `entry.rs:26` has `#[doc(hidden)]`, the doc says it is not for callers, and `str/DESIGN.md.tmpl:123-126` now names both hidden items.
2. Catalogued, not fixed (fixing needs a design decision). `persistence/tests/archive_str.rs:239-254` uses a real pair: both hashes end in `0x7b6fb9a`. The ignore reason follows the ledger's format, and `catalogue.md:18` has the matching 60-character slug. The changelist maps that slug to ticket `462b061b94ed`. Run with `--include-ignored` it fails at `:249`: the second string evicts to `0x8129587f57b6fb9a`, the first string's key.
3. Fixed. `generator.rs:139` (`compile_fail,E0080`) and `:168` (the positive control).
4. Partly fixed. See finding 1 above.
5. Fixed. `str/tests/str_const.rs:177-181` checks that the macro's handle equals the entry's hash, and pins `0x073967E8`, the low 28 bits of the reference FNV-1a value for "foobar", `0x85944171f73967e8`.
6. Ticketed as `f6f3d3d62f1c`.
7. Left, because it is a locked, closed changelist.
8. Fixed. `persistence/DESIGN.md.tmpl:97-99` no longer names the sha. No `1b2796e9` remains in docs or templates.
9. Left, because the subject is already pushed.
10. Partly fixed. `lto_smoke.rs:140,238` now run `env!("CARGO")`. The `RTLD_LOCAL` isolation test and the Mach-O and PE runs are ticket `2529323efd1f`.

## What makes each new test fail

- E0080 doctest (`generator.rs:139`): the failure it observes is the range check's. With index 1 and Standard's widths, `origin_base` is `1<<28` and `origin_ceiling` is `0x1FFF_FFFF`, with no overflow. `Uint::from_raw` goes down to arvo `Bits::from_raw`, which is `Self(raw)` with no check (`arvo-storage/src/bits.rs:86-87`). So the `assert!` at `generator.rs:212-215` is the only thing that can panic during const evaluation. Remove it and the doctest compiles, so the doctest fails. The control at `:168` differs only in returning index 0, and it builds. rustdoc checks the error code, not the message, so this conclusion is by elimination rather than by matching text.
- `the_macro_handle_is_the_entrys_and_the_low_28_bits_of_the_hash`: fails if the macro returns anything other than the entry's hash, or if the truncation changes (a different mask, the high bits, or the const flag folded in).
- `two_registered_strings_sharing_28_bits_evict_to_their_own_keys`: red now, and turns green only when two colliding const strings evict to their own keys. If the section order flips, the first assertion fails instead, so it stays red either way.

## Verified sound

- `cargo test --doc -p hilavitkutin-sym` on the worktree: 12 doctests pass, including both `single` doctests.
- `hilavitkutin-str` passes, the new test included.
- `persistence --test archive_str` passes 12, with the catalogue test red when run.
- `lto_smoke` passes 8.
- No added em-dashes, en-dashes or bold. The only matches are the re-swept flat topic copies, which come from the round tooling.
- The generated `docs/` match their templates on the sampled passages.
