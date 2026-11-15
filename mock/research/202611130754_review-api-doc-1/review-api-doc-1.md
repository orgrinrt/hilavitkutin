Source: kaski channel (coord/kaski of clause-dev), briefs/fixup/review-api-doc-1.md, first posted 2026-11-13 07:54 UTC, read 2026-11-15.

# Review of the doc changelist, round 202611130617-h6mb, hilavitkutin `work/api-revision` at `8af52de8`

Read only. Nothing built or run. Read in full: the changelist, the diffs of the two designs, the map, the brief, his answers 2, 5, 6, 8. Read in part: the sketch's api and author crates and the head and tail of its FINDINGS; the api source on `origin/dev` (`record_op.rs`, `work_unit.rs`, `builder_input.rs`, `context.rs`, `lib.rs` features). No suite is leaned on; the sketch's fixtures are cited as the sketch's claim. "CL" is `mock/design_rounds/202611130617-h6mb_changelist.doc.md` at `8af52de8`. Design lines are `origin/dev` `c42d707` unless marked "base" (`26e926b`, the branch's merge base).

## Verdict

Open the source window after fixes. Nothing is wrong at the root: every form is hung on an existing concept (store marker, access set, `RecordOp`, router, snapshot), no second crate or contract appears, formats stay generic, no data-bound table enters, no overflow rule is added, and nothing presumes head-and-tail or the N-way split. What stops the window today is one staleness fault, a set of silently changed design lines, and one shape of the value form that would break unit authors when corrected later.

## Gate one: what is refused as written

1. The CL's quotations are of the base, and `origin/dev` has moved two commits since, both on what the CL changes. Code: `git merge-tree origin/dev 8af52de8` reports conflicts in both design templates and both `docs/` files. `c42d707` landed the overflow work the CL calls "not landed on the trunk" (CL:249), and `27abd8a` landed the branded-index sketch the CL says "is on `origin/work/sketch-branded-index` and not on this branch" (CL:151, :259). The design now holds an `AccumWriterApi` block with `append` and `len` and a section on `AccumRefused` that the CL's own `AccumWriterApi` block (no `len`, `append` with an index) collides with. A changelist that quotes lines that are no longer the design's cannot be the record of what it changed.
2. The silent lines of section 2 below. Each is cured by naming, quoting and reasoning it; none needs the form withdrawn.
3. "Declared per column" narrowed to the frame-buffered kind (CL:123). Not silent: the helper flags it. It is his to read, so it is held as question Q1 and the in-place sentence is not written as design until he answers.

## 1. The forms, one by one

Record domains (C3). Extension of the column marker in its own terms: `Column<T, D = Records>`, default keeps `Column<T>`. In the types: the domain, the axes order, one domain per unit (refused at the `impl` for the value form through `Along`), a missing extent refused at `build`. Contract: `Domain`, `Grid`, the extent store. Leaves for the plan: a count per domain, single-domain fibers, one contiguous window. Not looser. No std or alloc. Two faults. `Extent<D>` is called "a singleton store" and is added to `StoreBundle`, while the section says "Five store kinds" (CL:39): it is a sixth kind or it is a resource, and the CL must say which and why it is not `Resource<Extent<D>>` with writes refused. And "a unit whose members name two is refused where it is declared" holds for the value form only; a block-form unit's domain is "folded from its sets" in `Declares`, so its refusal is at registration.

The value form (C4). Extension of `RecordOp`, as the map's row 1 says. In the types: read list, write list, total return, no handle, no engine type. Contract: `RecordOp` through `RecordDispatch` and `Declares`. Leaves the engine the record sequence, the thread and composition. Three things are nearer a second vocabulary than the CL admits. (a) `ReadList` and `WriteList` stand beside `AccessSet` as lists of "members" projecting to stores; the map's row 11 classed exactly that pair of names (`ReadMember`, `WriteMember`, `Reads`, `Writes`) as SAME as `AccessSet`, and shape A says `Near` is "added as a member of a read set". The CL does not say why `AccessSet` is not widened. (b) `Declares` restates `Plannable`'s three members and adds two; design api:372-375 says "`Plannable` is that half" (the declaration). The CL gives the overlap reason for a blanket `Plannable` but not why the router type itself cannot be the `Plannable` (`impl Plannable<S> for RecordDispatch<U, S, H>`), which would keep one declaration trait. (c) Schedule and hint sit on the router for one form and on the trait parameter and an associated type for the other. Judgement: (a) and (b) are fixable in the text or by folding; (c) is his (Q4). One real loss: the table gives a resource read as "its `T`" by value, and the sketch has `impl<T: Copy + 'static> Reads for Resource<T>` (sketch `api/src/lib.rs:366`). Code today: `fn resource<T: 'static, I>(&self) -> &T` (`context.rs:183`). So the primary form cannot read a `Seq` or `Map` resource, which is the map's own charge against the branch (reverse map, resource row).

Neighbour reads (C5). Extension of the read set. In the types: store, offset, edge value, reach checked at the type, committed kind only. Contract: `Near`, `Edge`, `Committed`. Leaves halos and key stepping to the plumbing; this revision computes key and position per read (the sketch's FINDINGS says one division per axis per record and a key rebuilt per neighbour, unmeasured). Boundary at the read as asked. Not looser: tighter than `read(i)` at any index. `Shift<A, const N: isize>` uses the const-generic exception, which the rule grants "only where the arvo type is truly painful"; the CL asserts the exception without saying why.

The frame-buffered kind (C6). A sibling of `Column` as `Accum` was; the relation to `ResourceSnapshot` is stated in both directions. In the types: kind, change rule, one writer, block-form write unresolvable. Contract: `Buffered` through `PerRecord`, `Committed`, `Changed`. Leaves the plan free of an edge. Costs a second column. The soundness sentence leans on an engine promise the types do not carry: the staged side is whole only while the writer is applied to every record of every pass it runs in, so the sentence "a finer one (per morsel, per record) changes nothing in a declaration" is true of declarations and false of the swap, which would then expose two-pass-old values. Say so.

Two writers of a `Buffered` store (C7). Named, quoted, reasoned, and marked as his. Correct against the map's row 21. Goes to him as Q2.

Safe accessors (C8). Correction of `unsafe fn read / write`. In the types: a branded index. Contract: accessor traits provided over the `Resolve*` bridges. Tighter than today. Three faults. "It has no public constructor" and "minted through `Index::mint`, an `unsafe fn`" cannot both hold across the crate border: `mint` is public and unsafe, and the text should say that. The index must be `Copy` to read two columns at one record, so it proves no at-most-one-append; on dev the per-binding capacity check is what makes an append safe, and the CL's "the `Index` a write takes carries that proof" is true of `write` and not of `append`. And `batch` loses the position as a number with only an idea ticket: the block form exists "where the body needs the whole morsel", and a body that cannot stride, pair or key its records has lost most of that. His (Q3).

The shared list (C2). Correction, row 38, consuming his parked topic with three differences stated. `Holds` and `HoldsAll` are api traits over the shared ones, for the message only. Not a second vocabulary. One leftover: api:989-993 and :1154-1158 still describe the `build` bound as `#[marker]` resolution after the bound became `HoldsAll`, which is not a marker trait.

Formats (C9), registration (C10). Hold his words. Nothing added.

## 2. Design lines changed or contradicted without being named

Left side `origin/dev`, right side the CL or the branch's design.

- `mock/DESIGN.md.tmpl:86`, "Per-morsel vs per-record WU dispatch inferred from AccessSet shape: column-shaped WUs receive a `Morsel<T>` window; resource-only WUs dispatch per-record." Against CL:93 (the form "is read off the router") and CL:145 (`Block<'b>`). Not in the CL's scope line (CL:6) at all.
- api:73 and :88, "the four router shapes", "The four router structs sit beside `Provider`", and the table at :91-96. Against CL:193, "`RecordDispatch` is one more router".
- api:327, "`type Hint: SchedulingHint = (Immediate, Atomic, Normal);`". Against CL:107, "the api does not enable defaults for associated types". Code agrees with the CL (`work_unit.rs:61`, no default; `lib.rs:13-16`, no such feature), so the design line is the one that is wrong, and the CL reasons from source past it without naming it.
- api:445 (base), "`// full morsel slice, non-fusible`" becomes "the whole morsel"; CL:145 names `Block` and does not quote the line.
- engine:104-110, the incremental skip "seeds a dirty set from per-root change signals: a `PlanAffecting` resource swapped via `replace_resource`, a non-plan-affecting value swapped via `replace_value`, an input the consumer marked changed, or the cold first frame". Against CL:163 and the new engine section ("whether a flip marks the store dirty for the next pass"): a fifth seed, carried across passes, for a store that by design has no edge to propagate over.
- engine:165-168, "four fold traits, `ResourceBundleOf`, `ColBundleOf`, `AccumBundleOf`, and `VirtBundleOf` ... (the contributing kind conses its node, the other three pass the tail)". Against the new engine section ("A read projects the committed pointer, a write projects the other"): a fifth kind in a four-way dispatch, unsaid.
- engine:80-84, step 9, "columns at type-native stride ... accumulators and virtuals zero". Against CL:177, "Morsel sizing: unchanged per fiber": a written `Buffered` store has no arm in the size fold, and a neighbour read touches lines the L1 window does not count.
- `hilavitkutin-kit/DESIGN.md.tmpl:41-43`, "`Units` is a cons-list of `WorkUnit` declarations ...; `Owned` is a cons-list of store markers (`Resource<T>` / `Column<T>` / `Virtual<T>`)", and `mock/DESIGN.md.tmpl:21`. Against CL:193, "a kit's unit bundle may hold units of either form". Also kit:175, "`ContainsAll<Wus::AccumRead>` proof at `.build()`", against `HoldsAll`.
- The CL's claim "A column written in place keeps the rule it has, that a store counts as changed when a unit that writes it ran" (CL:123, api design new text). The design states no rule about stores: engine:110-112 propagates over units, "`dirty[j] |= dirty[i]` for every edge". The CL words a new store-level rule as an existing one.

## 3. What must keep working

- Read-after-write, DAG: stated, right. The no-edge kind is the one exception and is said.
- Planner steps: stated ("grouping gains one rule"). Right as far as it goes; steps 9 and 12 change too (above) and are not said.
- Fusion: stated and right against `record_op.rs:34-42`: the fold bounds on one-member lists. The map's open point stands: no committed number shows the value form at parity with `each` or a hand loop, and the CL makes it primary. A bench is owed under the harness, as a ticket at least.
- Morsels: window per domain stated; size fold not (above).
- Memory provider, `ColumnStorage`: stated, right; two reservations through `reserve`.
- Resource model: "nothing" (CL:183) is wrong for the value form (by-value `Copy` read, section 1).
- Virtuals, `On<V>`, `OnMeta<V>`: stated. Right against `work_unit.rs:186-189` (`ScheduleGate` covers all three). A per-record `Bool` folded to one fire is new behaviour and is said.
- Kits: stated, but against unquoted kit lines (above).
- `Plannable` and a hand-declared type: stated, and it holds in code only if that type also implements `HasSchedule` by hand (`work_unit.rs:191-199`); say so.
- Persistence: "nothing" (CL:197) is unexamined. A `Buffered` store is two columns and a flag saying which is committed; what `evict` and `inject` see is not said, and `TableMeta` holds one `record_count` (persistence design:166).
- Accumulators: `append` gains an index and the design block drops `len`, which dev's design now has; must be reconciled on the merge.
- Extensions, `-ctx`, `-str`, `-sym`, `-build`: "nothing"; I found nothing against it and did not read those crates.

## 4. Implementability, as predictions

- The two `Declares` impls, `Along`, `Join`, `Holds` with `do_not_recommend`, `Near`'s where-clause on the reach, `Shift` equality under `Contains`: predicted to compile; the sketch shows all but `Declares` on the real traits. None needs `generic_const_exprs` or `specialization`.
- The second-writer refusal "by a const assertion ... which fails compilation of the `build` call": predicted to compile, and predicted to be a post-monomorphisation error. Such an error does not fire under `cargo check`, so a compile-fail twin run by a check-only harness would pass falsely. Either force the constant through a type position or confirm the harness builds.
- Sizing each column from its domain's `Extent<D>` value inside the build walk: a lookup of a value by type in a heterogeneous list cannot dispatch on "head equals `Extent<D>`" without overlap, so it needs an inferred position witness per column, threaded as a witness list through `build`. Predicted feasible (the accessors already infer `I`), unshown (the sketch's `plumb` uses `Vec` and `std`), and the likeliest place for "type annotations needed" where one domain's extent is registered twice.
- Const masks restricted to `Buffered` positions need a store's position as a constant; `Contains` is a marker trait and carries none, so the same witness is needed there.
- `AccessSet`, `WorkUnitBundle`, `Grid` implemented for the shared `Cons`: local traits on foreign types, allowed. Any inherent or foreign-trait impl the api had on its own `Cons` is lost; I did not search for one.
- `Vals<L>` and `rec!` over `notko-hlist`'s `Project` and `PCons`: shown by the sketch.

## 5. His to decide

Q1. When the engine tracks which stores changed in a frame, which stores declare what "changed" means? (a) only the frame-buffered kind; a column written in place always counts as changed when its writer ran. (b) every per-record kind declares it, default differs, which asks that every column value can be compared. (c) every per-record kind declares it; default differs for the frame-buffered kind and written for a column written in place. (d) the writing unit declares it, not the store.
Q2. Two units write one frame-buffered store. (a) refused at build, at compile time. (b) resolved by priority as for every other kind, the lower writer being dead work. (c) refused now, with a later form where two writers declare disjoint record sets.
Q3. Once writes are safe, what does a block-form body know of the record it is at? (a) an opaque index only; a key arrives later as a declared readable member. (b) an opaque index that can be read as a number but never built from one. (c) the safe accessors, with the old numeric `unsafe` pair kept beside them.
Q4. Where does a value-form unit say when it runs and how it is priced? (a) on the router it registers with, `RecordDispatch<Self, On<V>, Hint>`. (b) on the unit, as `WorkUnit` does, with a hand-written schedule impl. (c) associated types of `RecordOp`, every unit writing both.
Q5. Does a value-form unit read a resource by value (plain `Copy` values only, collections through the block form) or by reference (any resource, the read product carrying a lifetime)?
Q6. Names of the list's cell and end (the helper's ticket, four options). Blocks nothing.
Q7. Is a domain's extent fixed for the life of a built scheduler, or replaceable between passes like a value swapped with `replace_value`?

Blocks the source window: Q5 (it sets `apply`'s signature for every author). Q1 blocks only the in-place sentence. The rest can be built on the CL's choice and overturned.

## 6. Fixes, blocking first

1. Merge `origin/dev` `c42d707`, consolidate both designs by hand, and re-anchor every quotation and line number on the merged text; correct the two "not landed" statements; reconcile `AccumWriterApi` (`len`, the appender-slot obligation, the capacity check) with the indexed `append`.
2. Name, quote and reason each line of section 2, and widen the scope line to the root and kit designs.
3. Settle the resource read in the value form (Q5) or state the limit and correct "the resource model: nothing".
4. State the incremental-skip change as a change: the cross-pass seed, what a `Written` store whose writer reads it means for rest, and that a finer skip must keep the staged side whole.
5. Say whether `Extent<D>` is a sixth kind or a resource, and correct the count.
6. Give the reasons `AccessSet` is not widened for `Near` and `Plannable` is not the router's trait, or fold them.
Not blocking: the post-monomorphisation twin; the `isize` reason; the wording on `Index::mint`; the step 9 size fold; persistence of the committed side; the marker sentences left on `HoldsAll`; the block-form refusal site; a bench ticket for the value form and the neighbour read against a hand loop.
