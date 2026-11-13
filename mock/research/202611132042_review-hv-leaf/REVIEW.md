# Review of work/hv-leaf at 7dc08813, round one

Reviewer: a fresh `pr-reviewer-senior` sub-agent on opus, in a worktree at `7dc08813`, base `origin/dev` (`271909e5`). The report below is its own, kept whole.

Canon gate: passed. hilavitkutin declares no `canon_paths` (`.claude/rules/canon-design-code-chain.md`), so the `mock/crates/*/DESIGN.md.tmpl` designs stand in for the canon and the code was checked against them. The public-surface checklist `the-public-surface-is-checked-on-every-pr.md` is not in the repo tree; the sweep was run from the workspace rule's card.

## Verdict

MERGE AFTER FIXES

## Findings

1. `mock/crates/hilavitkutin-persistence/tests/archive_str.rs:326-336` [tests]: the 10,000-string collision test cannot fail for the reason it names. The keys come from `const_fnv1a` directly. Nothing goes through `evict_str` or the `StringTable`, and the width is hardcoded to 64 rather than read from what the crate stores. Its control runs at 16 bits, not at the 28 bits it replaced. On these exact keys, collisions per width, counted with a scratch rustc run (an ad-hoc spike, not committed): 16 bits gives 1224, 24 gives 72, 28 gives 0, 64 gives 0. Changing `64` to `28` in line 335 leaves the test green. So it passes whether the stored key is 28 bits or 64, and if `evict_str` went back to masking, this test would not notice. The design claim at `persistence/DESIGN.md.tmpl:59` ("counts collisions at the stored width") is false of this body. Fix: route the keys through `evict_str` on interned runtime handles and through `StringTable::lookup`. Use a key count or a control at the old 28-bit width that actually collides there. Either show the old width fails, or rename the test to what it does test.

2. `mock/crates/hilavitkutin-linking/src/backend/unix.rs:10` and `:32-39` [correctness]: the ticket's class is "a dlopen flag value differs per target", and only one instance was fixed. The backend runs on `cfg(unix)` (`backend/mod.rs:18`). On 32-bit Android (bionic), `RTLD_NOW` is 0 and 2 is `RTLD_GLOBAL` (libc 0.2.191, `android/b32/mod.rs:177-178`). So the hardcoded `RTLD_NOW = 2` loads every extension global on armv7 Android, which breaks the isolation the design says the trampolines rely on (`linking/DESIGN.md.tmpl:36`). `RTLD_LOCAL` is also not 0 on every other unix: NetBSD 0x200, QNX 0x200, AIX 0x80000 (`netbsd/mod.rs:1462`, `nto/mod.rs:1048`, `aix/mod.rs:590`). Those probably still load because LOCAL is their default, but that was not verified. Fix: set both constants per target (or take them from `libc`), and correct the design sentence and both changelists, which repeat it.

3. `mock/crates/hilavitkutin-sym/DESIGN.md.tmpl:385-388` against `src/generator.rs:105-108` [spec/drift]: the design says an out-of-range one-origin shape "does not compile with `single` in use". The code is a runtime `assert!` inside a `const fn`. It is a build error only when `single` is evaluated in a constant. A normal call, and every `Default::default()` (`generator.rs:121`), panics instead. The src changelist itself says "the check cannot be made at instantiation alone", so the locked design promises something the code is known not to do. Either fix the design text in a round, or make the check a real compile-time one. No test covers this failure path in either form.

4. `mock/crates/hilavitkutin-str/tests/section.rs` (`an_entry_hash_is_the_hash_of_its_string`) [tests]: `assert_eq!(e.hash, again.hash)` compares `StaticStrEntry::new("anchor-check")` with itself and is tautological. `assert_ne!(const_fnv1a(..), 0)` says nothing about the entry. The name claims the hash is derived from the string, and the body never compares it to `Str::__make(Bits::<28>::from_narrowed(const_fnv1a(..)))`. Delete the tautology and assert the derivation.

5. `mock/crates/hilavitkutin-persistence/src/archive_str.rs:57-65` [tests, edge case]: two `str_const!` strings that share 28 bits make one const handle ambiguous. `evict_str` then hashes whichever string the const table yields, which may be the wrong key. No test catalogues this case.

6. `mock/crates/hilavitkutin-persistence/src/string_table.rs:171-172` [rules]: rustfmt moved the `lint:allow` off the signature it justifies and into the body. Lints are still green (warn level), but the reason now sits on a line that has nothing to suppress. Put it back on the signature line.

7. `mock/crates/hilavitkutin-extensions-macros/tests/lto_smoke_fixture/Cargo.toml:27` [drift]: the comment still says "so `nm` can observe". `lto_smoke.rs:64-68` also counts undefined ELF symbols as "survived". Filter with `is_definition()`.

8. `mock/crates/hilavitkutin-linking/tests/not_yet_loaded.rs:24-37` [tests, minor]: the candidate paths are Debian multiarch only, so on Fedora or Arch the test panics. The negative control is sound.

## Verified

- Each crate re-run separately with `CARGO_INCREMENTAL=0 cargo test --manifest-path mock/Cargo.toml -p <crate>`. All green: str 30 tests, linking 11 (1 ignored is the doc block), sym 49, persistence 25 (3 cold_store ignored with tickets), extensions-macros 9 including trybuild. Providers `--test smoke` passes 7.
- `cargo mock check`: build, tests and strict lints green.
- str: the anchor plus the walker bounds check makes a link with no `str_const!` resolve. `the_section_holds_the_crates_own_empty_entry` and `the_empty_string_is_a_const_string` are real tests and would fail to link on dev.
- linking: the `RTLD_NOLOAD` control comes first and genuinely shows the premise.
- sym: the `compile_fail,E0271` doctest is pinned to the right error. The WideKind mint test asserts ids 0 and 1. Both one-origin shapes are covered.
- persistence: `evict_str` and `inject_str` match the design. The 28-bit lookalike test is real.
- lto_smoke: uses `object`, refuses unparsable files, has a negative control on the missing-symbol list, and the fixture is `lto = "fat"`.
- Job 6: trybuild fail fixtures do fail. The topic records this at line 24.
- The NEON sentence matches `mock/benches/ema_axis_h_n1024_findings.md`.
- No em- or en-dashes added. No bylines. Commit subjects are under 72 characters and correctly shaped. Tree clean.

## Not checked

- macOS, Windows and Android runtime behaviour.
- Whether NetBSD, QNX and AIX default to LOCAL.
- The Mach-O section start symbols when the section is absent.
- `-z start-stop-gc` retention of `str_const!` entries (that predates this branch).
- Regenerated `docs/*.md` read line by line; only grepped.
