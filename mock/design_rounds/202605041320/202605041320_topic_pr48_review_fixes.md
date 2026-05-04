---
round: 202605041320
phase: TOPIC
status: frozen
---

# Topic: PR #48 review fixes (round 202605041257 follow-up)

## Frame

Round 202605041257 shipped hilavitkutin-providers v0 and was
closed. PR #48 went out for review. The reviewer flagged three
findings:

1. **Soundness (load-bearing).** `arena_intern` writes through
   `(*self.bytes.get()).as_mut_ptr()` and `&mut *self.entries.get()`,
   creating `&mut [u8; BYTES]` and `&mut [Entry; ENTRIES]`
   reborrows over the `UnsafeCell` contents. Stacked Borrows /
   Tree Borrows admit overlapping `&self` calls (a caller can
   hold a `&str` returned by a previous `arena_resolve` while
   issuing a new `arena_intern`). Creating a `&mut` over the
   same allocation while a shared borrow into it is live is UB
   even single-threaded. Asymmetric with `arena_resolve`, which
   already uses raw-pointer-only access for the same reason
   (the `dangerous_implicit_autorefs` lint caught it). Fix is
   the symmetric pointer-arithmetic-only write pattern.

2. **Em-dash in module doc comment.** `lib.rs:1` opened with the
   crate name followed by an em-dash separator before the
   tagline. The workspace `writing-style.md` rule bans em-dashes
   anywhere. Replace with a colon.

3. **CL claim drift.** SRC CL of round 202605041257 claimed a
   "const-table short-circuit" smoke test. Landed `tests/smoke.rs`
   did not include it; the four tests covered round-trip, byte
   overflow, entry overflow, and the default constructor.

## Decisions

### Decision 1: round shape

Open a new mockspace round on the same feature branch
(`feat/hilavitkutin-providers-v0`) per the workspace
branch-pr-flow rule's "multiple sequential rounds per branch"
shape. The fixes ride in this round's SRC CL. PR #48 picks up
the fix commits before merge.

### Decision 2: soundness fix is the symmetric pointer pattern

`arena_intern` mirrors `arena_resolve`'s raw-pointer-only access:

```rust
// Before (unsafe, &mut reborrow over UnsafeCell contents):
let buf_ptr = (*self.bytes.get()).as_mut_ptr().add(cursor);
core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf_ptr, len);

// After (raw pointer arithmetic, no &mut reborrow):
let buf_ptr = (self.bytes.get() as *mut u8).add(cursor);
core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf_ptr, len);
```

Symmetric pattern for the entry-table write:

```rust
// Before:
let entries = &mut *self.entries.get();
entries[count] = Entry { offset: USize(cursor), len: USize(len) };

// After:
let entries_ptr = self.entries.get() as *mut Entry;
entries_ptr.add(count).write(Entry { offset: USize(cursor), len: USize(len) });
```

Append-only allocator invariant + `!Sync` invariant + cursor /
count bounds checks remain the SAFETY justification. Comments
are updated to call out the Stacked Borrows reasoning explicitly.

### Decision 3: scope down nit 3 to a soundness test, not a const-table test

The reviewer offered two options for nit 3: add the const-table
short-circuit test the SRC CL claimed, or strike the bullet.
Neither is the most useful fix. The actually-load-bearing test
this round needs is one that exercises the soundness invariant
finding 1 fixes: hold a `&str` from `arena_resolve` live across
a subsequent `arena_intern` and verify the borrow is not
invalidated. v0 ships that test
(`resolve_borrow_survives_subsequent_intern`); it earns its
keep by demonstrating the shape that would fail under Stacked
Borrows if the symmetric fix from finding 1 were not applied.
The SRC CL of this round records the new test.

A const-table short-circuit test remains worthwhile for
`StringInterner` end-to-end coverage, but it belongs in
hilavitkutin-str rather than hilavitkutin-providers. Tracked
informally; not in this round's scope.

## Sketches

None. Three mechanical fixes against verified-correct surface.

## Cross-references

- Round 202605041257: closed predecessor that PR #48 reviews.
- PR #48 reviewer report (chat transcript): origin of the three
  findings.
- `.claude/rules/writing-style.md`: em-dash rule.
- `.claude/rules/cl-claim-sketch-discipline.md`: CL-claim
  discipline that drove the choice to record the new test in
  this round's SRC CL.
