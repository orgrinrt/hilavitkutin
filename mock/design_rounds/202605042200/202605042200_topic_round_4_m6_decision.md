**Date:** 2026-05-05
**Phase:** TOPIC
**Scope:** Records the audit M6 decision after the M2 dedup investigation closed. Settles a single open question; not a substantive design topic. Captures the decision in a frozen artefact so the doc CL writeup picks it up directly.
**Source:** Audit topic `202605042200_topic_round_4_audit.md`, finding M6. M2 sketch dir `mock/research/sketches/202605050800_dedup_concat/`, FINDINGS.md.

# Topic: round-4 audit M6 decision

## Why this topic exists

Audit M6 named two options for `Kit::requirements_doc()`:

1. Drop the claim entirely. Document "what a kit requires" in prose (the kit author lists required Resources in the Kit's rustdoc).
2. Build the type-level Difference operator. Survives only if M2's dedup exploration shows Difference is feasible.

The M2 sketches (`mock/research/sketches/202605050800_dedup_concat/`) showed the Difference operator hits the same E0119 coherence wall as round-3's NotIn shape: two impls of `Difference<R> for L` distinguished only by where-clause guards (`L: Contains<H>` vs `L: NotContains<H>`) collide. NotContains cannot be soundly encoded under coherence with the workspace's nightly-feature constraints. Option 2 is therefore not viable in round-4.

## Decision

**Option 1: drop the `requirements_doc()` claim entirely.**

The substrate's correctness gate is `Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>` at `.build()`. The compile error that fires when a kit's required Resource is not registered already names the missing marker (S1b's depth-5 sketch confirmed: `Empty: Contains<Clock>` appears on the first line of the diagnostic). That error is the canonical "what a kit requires" answer.

Kit authors document required external Resources in the Kit's rustdoc, in prose, the same way any Rust API documents its bounds. The substrate does not provide a compile-time-derivable list.

## What this means for the doc CL

The doc CL writeup omits any reference to `Kit::requirements_doc()` or a type-level Difference operator. The "Required" concept in topic 3 stays as a documentation-level mental model ("the kit requires Resources X, Y, Z which the app provides"), not a substrate feature.

If a future round reopens type-level set difference, options to explore include:

- Specialisation, when `feature(specialization)` matures past current restrictions.
- A coherence-clean alternative encoding not yet discovered.
- Const-generic-string-keyed sets (depending on `feature(adt_const_params)` extensions).

None of these is round-4 work.

## Cross-references

- `mock/design_rounds/202605042200_topic_round_4_audit.md`. Audit topic; finding M6 named the two options.
- `mock/research/sketches/202605050800_dedup_concat/FINDINGS.md`. M2 investigation that ruled out option 2.
- `mock/research/sketches/registrable-not-in-202605051200/FINDINGS.md`. Round-3 NotIn proof; the underlying coherence issue.
- `mock/design_rounds/202605042200_topic_kit_trait_split.md`. Topic 3; the locked Kit shape (still has no `Required` field).
