# Review of work/hv-leaf at eb71f2e4, round 2

Reviewer: fresh `pr-reviewer-senior`, opus, worktree detached at eb71f2e4. The report is kept whole, as returned.

## Verdict

MERGE AFTER FIXES

Gate: hilavitkutin has no `canon_paths`, so the code was checked against the five touched `mock/crates/*/DESIGN.md.tmpl` files. This is the first expert read against the designs, so a second read is still owed. The suites for str, sym, persistence and linking were run, plus `lto_smoke` and `trybuild` in extensions-macros, at eb71f2e4. All are green, including str in `--release`. The body of every new or changed test was read and no tautological test was found; the one the first review flagged is gone. `cargo mock check` was not run.

## Findings

1. `StaticStrEntry::new` is a new public way to make const handles. `mock/crates/hilavitkutin-str/src/entry.rs:23` [public-surface/spec]
   - It is a documented, non-hidden `pub const fn`. `StaticStrEntry::new("x").hash` gives a const handle for a string that was never registered, and `resolve` returns `Isnt` for it.
   - This contradicts `str/DESIGN.md.tmpl:77-78`: "`Str::__make` is not public, `str_const!` is the only way to create const handles". `__make` is at least `#[doc(hidden)]` (`handle.rs:31`).
   - Fix: hide it the same way (`#[doc(hidden)]`, named `__new`), or change the design in a round.

2. Two const strings sharing 28 bits still produce the wrong key, and nothing records it. [tests/drift]
   - Scenario: two `str_const!` literals with equal low 28 bits. `lookup_const_by_value` hands back the same handle for both (`interner.rs:124-126`), and `lookup_const_by_handle` returns whichever entry comes first (`interner.rs:134-137`). So `evict_str` (`archive_str.rs:44-48`) can return the other string's 64-bit key.
   - That is the exact silent wrong-string-on-disk failure that `persistence/DESIGN.md.tmpl:33-36` says the 64-bit key removes.
   - The round topic (`202611132042_topic...md:9`) says this is "catalogued as a ticket" but gives no ticket id, and there is no test.
   - Fix: add an `#[ignore = "<ticket>: ..."]` catalogue test that finds a const pair the way `strings_sharing_28_bits_evict_to_different_keys` does. The design does not say how this should resolve; that is a question for the designer.

3. The claimed build error has no compile-fail test. `sym/DESIGN.md.tmpl:385-387` [tests]
   - The design says `single` in a constant is a build error for an out-of-range one-origin shape. Line 222 of the same design says "Every refusal this document claims is a compile-fail test."
   - Only the runtime panic is pinned (`tests/generator.rs:154,160`).
   - Fix: a `compile_fail,E0080` doctest with `const G: Generator<NoRoomGen> = Generator::single();`.

4. The `str_const!` snippet and platform lines in the str design are still wrong. `str/DESIGN.md.tmpl:60,74-75` [spec]
   - This round rewrote the snippet but left `#[link_section = ".hilavitkutin_strings"]` with a leading dot. A dot-prefixed name gets no `__start_`/`__stop_` symbols, so the snippet built as written would not link, which is the defect the anchor exists to fix.
   - Line 75 says macOS uses `getsectiondata` on `__DATA,hilavitkutin_strings`. The code uses `__DATA,__hvkstr` and the `section$start$` symbols (`section.rs:91-95,160-163`).
   - Fix both in a doc changelist.

5. The entry-hash test only restates the constructor. `str/tests/section.rs:292-304` [tests, minor]
   - The expected value is spelled with the same formula as `new` itself, there is no fixed known-answer value, and `str_const!` is never compared with `new`.
   - Fix: add `assert_eq!(str_const!("anchor-check"), StaticStrEntry::new("anchor-check").hash)` and a pinned 28-bit hex value.

6. A language-limit claim has no committed evidence. `sym/DESIGN.md.tmpl:388-389` and `202611132009_changelist.src.lock.md:31` [drift]
   - Both say a `const` block and an associated constant cannot see the `[const]` bound. No committed sketch backs this, and it is the reason the check is a runtime panic rather than a compile-time error.
   - Fix: commit a sketch, or try the trait-and-associated-const decomposition first.

7. A changelist line does not match the code. `202611132009_changelist.src.lock.md:15` [CL accuracy] says "Each platform's `imp` places one anchor entry". There is one anchor, at module level outside `imp` (`section.rs:149-164`).

8. Workspace-internal provenance in a public doc. `persistence/DESIGN.md.tmpl:79-83` renders into `docs/PERSISTENCE.md:108` [public-surface/drift]
   - It names arvo sha `1b2796e9` and says "a revision arvo's own `dev` no longer carries". That is internal history in a public document.
   - It is also a real risk: the workspace is pinned to an orphaned arvo revision, which should be a ticket.

9. One commit subject says how, not what. e7397b77 [rules]: "fix: act on the first review of the leaf crate fixes" describes how the change came about, not what changed. The other 15 subjects and the one body have no bylines and no workflow leakage.

10. Smaller test notes [tests, minor]
    - `lto_smoke.rs:140` uses `Command::new("cargo")` rather than `env CARGO`.
    - The Mach-O and PE branches of `read_symbols` (`:78-106`) have never run. Only ELF ran here, although the design says "every host".
    - `not_yet_loaded.rs` is gnu-only.
    - No test anywhere checks the `RTLD_LOCAL` isolation that `linking/DESIGN.md.tmpl:10` says the trampolines rely on. This gap was already there before the branch.

## Verified

- str: the anchor and the bounds check in `between` are sound. `the_section_holds_the_crates_own_empty_entry` would fail to link without the anchor. `the_empty_string_is_a_const_string` would fail without it too, since `is_const` would be false.
- linking: the flags now come from `libc`, and `libc` is already a dependency (`Cargo.toml:15`). The `RTLD_NOLOAD` control runs first, then `Library::load`. The test fails if the flag goes back to 4.
- sym: the `[const]` bound on `single` and the plain bound on `Default` are correct. The `compile_fail,E0271` SixteenMinters doctest passes. The `should_panic` tests pin the exact message. Both shipped one-origin shapes (`Standard`, `WideKind`) are covered.
- persistence: `ContentHash = Bits<64, Hot>` at the pin (arvo `aliases.rs:12`). The evict and inject code matches the design. The old `.unwrap()` panic is gone. The lookalike, sharing-28-bits and 10k tests each fail under 28-bit truncation, the 10k one through its per-key `assert_eq`. The design's collision figures check out (about 17 percent at 28 bits, about 3e-12 at 64).
- lto_smoke: only defined symbols are counted, unparsable files are refused, the missing-trampoline control is real, and the metadata parse has four unit tests.
- trybuild: all four fail fixtures fail to compile, which matches the not-reproducing record (`202611132004_topic...md:24`).
- docs: `docs/*.md` match their templates on the sampled new passages.
- Style: no em- or en-dashes and no new bold in the added lines. The only bold added is the round metadata lines, which are exempt.
- Flat topics: both are re-filed flat at HEAD. Copying them into each closed round is a pattern older than this branch (it is on `dev` five times) and comes from the round tooling.

## Notes

The first review's findings 1, 2, 3, 4, 6, 7 and 8 were acted on. Finding 5 was only waved at, and it is the one that undercuts what the 64-bit-key job claims (finding 2 above). The public checklist `the-public-surface-is-checked-on-every-pull-request.md` exists only as the workspace card, not in the repo tree; the sweep ran from that card.
