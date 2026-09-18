# GATE-2 Deviation 5: the Aliasing Audit Confirms a Hole

**Date:** 2026-07-19
**Status:** the #689 audit, delivered as the deviation-5 evidence under
evidence-then-bless (A2-4); seed governance item 5. Independent
reviewer audit over `scheduler/mod.rs` and `thread/frame.rs`, findings
verified against source before acceptance.

## The verdict

The discipline-sound aliasing claim does NOT hold as shipped. The
frame protocol's happens-before is real (the publish/await and
shutdown/exit pairs are genuine Release/Acquire, not comment-asserted),
and `Drop` joins workers before any field drops. The hole is one level
deeper than the ledger's framing: `worker_main` derives one
`&Scheduler` shared reference and holds it live across every park for
the worker's whole life, while the public between-frames surface
(`mark_dirty`, `replace_value`, `replace_resource`, `run`, `run_fused`)
takes ordinary `&mut self`. The INTENDED driver pattern,
`run_parallel(..); replace_value(..); run_parallel(..)`, therefore
materialises a `&mut Scheduler` on the main thread while a worker's
`&Scheduler` is live. That is an aliasing-model violation (exclusivity
is asserted over the whole allocation for the reference's lifetime)
independent of timing and independent of which fields carry interior
mutability; the inline SAFETY comment only forbids raw `*mut` writes
and never considered the `&mut` receivers. The risk is
miscompilation-class (noalias codegen licences), not a data race the
frame protocol could order.

## The structural fix is the plane relocation

No in-place guard repairs this without breaking the intended API: a
spawned-check panic on the `&mut` surface would outlaw the
between-frames swap contract the swap spec just ratified into shape,
and converting the receivers to raw-pointer discipline does not help
because the `&mut` receiver itself asserts the exclusivity. The shape
that makes the API legal is the deviation 1+6 arena-plane relocation
(record `202607201400`, sketch `202607201300` WORKS): workers hold
pointers only into a separate provider-allocated plane, so a
`&mut Scheduler` on the handle never overlaps any worker-held
reference. The audit therefore upgrades that proposed ruling from
canonical-plus-ergonomics to SOUNDNESS-REQUIRED: deviations 1, 5, and 6
resolve in one relocation round, and blessing the shipped shape would
bless undefined behaviour.

## Catalogued

The failing pattern is catalogued as an ignored test naming the
interleaving and the resolution (`tests/replace_swap.rs`); it is not
runtime-observable natively (the violation is a compile-model fact a
Miri run would flag), so the catalogue entry documents rather than
fails, and unignores as a Miri-gated check when the relocation lands.
