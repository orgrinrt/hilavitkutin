# Swap semantics panel, memo 1: soundness and correctness

**Date:** 2026-07-19
**Author:** Hans-Kristian Arntzen (lens: correctness, soundness, what must be established versus assumed)
**Scope:** what `replace_value` / `replace_resource` must do under the bench-decided storage model (`202606210600_resource-storage-model-canonical-addendum.md`), what the spec must state verbatim, and where a real fork remains.

A swap is a write into a live system, not a value assignment on an inert struct. The scheduler holds the only handle, worker threads hold raw pointers into the same memory, and the plan-recompute machinery watches for the write's aftermath. Every claim below traces to a named constraint. Where I could not derive an answer from the constraints, I name it as a fork and the evidence that would close it, not a pick.

## 1. Forced by constraints

**The write path is a `Selector`-witnessed whole-blob write, never a decomposed leaf copy.** The storage addendum settled the layout: one-record contiguous blob per resource, `Decompose` scoped to the size fold and the collection ptr+len, never per-member columns (addendum lines 18 to 28). `engine_ctx.rs:733` shows the canonical read of that blob: `Selector<T, I>::get(self)` yields the backcast pointer, then `core::ptr::read`. A2-5 confirms the "member-by-member copy" language in the shipped `replace_value` FIXME (`scheduler/mod.rs:1235`) traces to a tier-4 input memo whose layout the bench refuted, not to canon. So the sound write is the mirror of the sound read: the same `Selector` witness, `ptr::write` in place of `ptr::read`. A fresh pointer derived any other way breaks the provenance argument the noalias invariant depends on (addendum item 5).

**The old value needs no drop.** `engine_ctx.rs:717` states resource values are `T: ColumnValue` (`Copy`), so overwriting the blob in place is sound with no destructor obligation and no double-free risk from an in-flight duplicate.

**A swap can never race a live-streamed read, because it can never occur mid-frame.** `replace_value` / `replace_resource` take `&mut self`. Worker threads dereference the scheduler only through the retained `*const Scheduler` inside `run()` / `run_fused()`, per the pinned park-between-frames invariant. Rust's aliasing rule forces the caller to hold exclusive access to call either function, and the only place that access exists is between `run()` calls, when every worker is parked. This is not a design choice: it is what `&mut self` plus the parked-workers invariant together entail. Consequently a swap is observable only starting at the next `run()`, never mid-frame; there is no other window for it to land in.

**Seq/Map members never move their base pointer, and the type system enforces the capacity bound.** `Seq<T, N: Cap>` / `Map` are const-sized (expert-architect memo line 20); the "new" value passed to a swap shares the exact same `Seq<T, N>` type as the old, so its live length is always `<= N` by construction. Columns are sized once at plan time and never resized (schedule-once-reuse). So the sound write for a collection member is: keep the ptr+len view's base pointer unchanged, write up to `N'` elements into the existing column region, then update the length. A swap that supplies a different base pointer, or writes past `N`, is unsound by construction and the spec must forbid it outright.

**`replace_value` marks store-dirty only; `replace_resource` must mark store-dirty and plan-dirty, and today it does not do the second half.** `scheduler/mod.rs:1394` shows `run()` reads `self.plan_dirty` only to suppress an unused-field warning (`let _ = (&self.plan_dirty, &self.plan_cache);`); the actual plan-dirty gate is computed solely from `first_frame` (`:1389`). `replace_resource`'s body (`:1217-1221`) never sets a bit in `self.plan_dirty` either. The domain-22 recompute half is unwired end to end. This is a shipped gap, not a spec question: the spec must state that a `replace_resource` swap sets a plan-dirty bit consumed by the next `run()` to force the leading plan band regardless of `first_frame`, and today's code does not do this.

## 2. Genuine forks

**Can a type implement both `PlanAffecting` and `Replaceable`?** Nothing in the trait definitions (`store.rs:270`, A2-3) prevents it, and if it happens, a caller can call `replace_value` on a plan-affecting resource and silently skip the recompute mark: a soundness hole with no compiler signal. Deciding this needs one of: a supertrait bound (`PlanAffecting: Replaceable` or the reverse, foreclosing the ambiguity structurally), a `negative_impls`-based mutual exclusivity (WATCH tier per `unstable-features.md`, sound but with an open coherence gap, #133556, that may or may not bite here, untested), or a mockspace lint scanning for the double impl. Evidence needed: a sketch of the `negative_impls` bound against the current nightly pin to see whether the coherence gap actually blocks it.

**Whether a mutable write accessor into a collection column exists at all is unresolved**, and the answer decides how much of `replace_value` for Seq/Map-bearing types can be spec'd now. `engine_ctx.rs:722` FIXME states resources are read-only through the Context; no write-back path is built. Evidence: does #344's live-stream wiring include a write side, or only read.

## 3. Soundness obligations the spec must state verbatim

1. A swap writes through the same `Selector<T, Index>` witness the drain and the read path use; never a pointer re-derived by any other route.
2. `replace_value` / `replace_resource` may be called only when the caller holds `&mut Scheduler`; the spec names this as depending on the parked-between-frames invariant, not merely on the borrow checker, because worker raw pointers are outside its jurisdiction.
3. A collection member's swap never changes the ptr+len view's base pointer and never writes past the type's `Cap` bound.
4. `replace_resource` on a `PlanAffecting` type sets a plan-dirty bit consumed at the very next `run()`, independent of `first_frame`.

## 4. Test surface

Byte-identity of the blob address across a swap (proves the same-witness-write claim). A collection-bearing swap test asserting pointer stability and correct length update. A currently-unwritable but catalogue-worthy red test: a `replace_resource` swap of a `PlanAffecting` type causes the next `run()` to enter the plan band on a non-first frame (blocked on the `plan_dirty` wiring gap in section 1). A double-swap-before-run test: only the last value is ever observed. A ZST-resource swap test: dirty is set, no byte write occurs, no UB. A swap-of-an-unread-resource test: dirty is set and cleared next frame, no unit ever re-runs, no soundness break.
