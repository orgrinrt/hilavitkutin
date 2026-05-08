**Date:** 2026-05-05
**Phase:** TOPIC
**Scope:** Records the corrected framing for kit visibility / kit-private state enforcement that the doc CL must use. Consolidates audit C2/M4 plus the S5 visibility-axis-collapse finding into a single source the doc CL can paraphrase directly. Frozen on commit; supersedes any "substrate enforces kit-private state" claim from earlier topics.
**Source:** Audit topic `202605042200_topic_round_4_audit.md` findings C2 / M4. S5 FINDINGS at `mock/research/sketches/202605050615_kit_taxonomy/FINDINGS.md`, "Visibility finding" section.

# Topic: round-4 layered enforcement of kit-private state

## Why this topic exists

Topic 3's "two committed scoping axes" framing implied the substrate enforces kit-private state via visibility. The audit C2 corrected that to "the substrate enforces nothing; Rust's standard visibility rules do all the work." The S5 sketch revealed the picture is even more constrained: under the chosen `pub Kit { type Owned: StoreBundle }` shape, Rust's visibility rules cannot enforce per-Owned-type kit-internalness either, because E0446 forbids less-visible types in the associated-type position of a more-visible trait impl.

This topic captures the corrected framing in a single frozen artefact so the doc CL writeup can paraphrase from it without rebuilding the analysis.

## What the substrate does NOT do

The doc CL must NOT claim or imply any of the following:

- "Substrate-enforced kit-private state."
- "The typestate prevents one kit from accessing another kit's Owned types."
- "`pub(crate)` wrapping makes Owned types kit-internal at the substrate level."

None of these is true. The typestate sees every type in the accumulated `Stores` list, regardless of where it was registered or what visibility modifier its declaration carried. ContainsAll proofs route through the trait solver, which has no knowledge of module boundaries or Rust visibility.

## What the substrate DOES do

The doc CL CAN claim:

- The typestate proves `Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>` at `.build()`. This is correctness only: every read and every write must correspond to a registered store. It is not an access-control mechanism.
- `Replaceable` is the one substrate-enforced annotation. The static `T: Replaceable` bound on `replace_resource::<T>(...)` rejects override attempts on types that did not opt in. This works regardless of visibility.

## Where visibility lives (the layer below the substrate)

Rust's visibility rules govern which paths a kit's WU declarations can name. The substrate observes the paths the consumer writes; it does not enforce visibility. The visibility model the substrate is compatible with:

1. **Whole-kit pub.** Kit struct, Owned types, WorkUnit types are all `pub`. Cooperative-public is the only mode. "Kit-internal" is documentation convention applied by the kit author keeping non-Replaceable types out of the kit's public re-export surface. Other kits in the same crate can name them; consumers in other crates can name them via the kit's full path.
2. **Whole-kit pub(crate) within the kit's own crate.** Kit struct, Owned types, WorkUnit types all `pub(crate)`. Consumers register the kit via a pub helper-fn that consumes a builder and returns one without naming the kit. Kit-internal types are then unreachable across crate boundaries. This is the genuine kit-private mode.

The substrate cannot mix levels in a single Kit impl. E0446 forbids associated-type positions less visible than the impl's effective publicity.

## The layered picture

The doc CL describes correctness, access control, and convention as three separate layers, each operating independently:

| Layer | Mechanism | What it enforces |
|-------|-----------|------------------|
| Substrate (typestate) | `Stores: ContainsAll<...>` at `.build()` | Every WU's reads and writes have a registered store. Not access control. |
| Substrate (replaceability) | `T: Replaceable` bound on `replace_resource` | Apps can override only opt-in types. |
| Rust language (visibility) | E0446 + module visibility | Kit-as-a-whole publicity; cross-crate name reachability. |
| Convention (documentation) | Kit author's rustdoc + public re-export choices | Kit-internal vs cooperative; entirely soft signal. |

Each layer is independent. None subsumes any other. None is a per-Owned-type substrate axis.

## What this means for the round-4 plan

Round-4 ships:

- One substrate-enforced annotation: `Replaceable` (opt-in).
- One per-kit structural choice: whole-kit pub vs whole-kit pub(crate)-in-own-crate.
- No per-Owned-type visibility knob.
- No substrate enforcement of kit-private state. Documentation conventions and crate-boundary visibility are the only available levers, and both live below the substrate.

The doc CL writeup paraphrases this section's framing rather than proposing a "two-axis substrate annotation" as topic 3 did.

## Cross-references

- `mock/design_rounds/202605042200_topic_round_4_audit.md`. Audit topic, findings C2 / M4.
- `mock/research/sketches/202605050615_kit_taxonomy/FINDINGS.md`. S5 visibility finding (the empirical evidence behind this framing).
- `mock/research/sketches/202605050615_kit_taxonomy/sketch.rs`. The compile experiments that surfaced E0446 on `pub(crate)` Owned types.
- `mock/design_rounds/202605042200_topic_kit_trait_split.md`. Topic 3, locked Kit shape (still defines `pub trait Kit { type Owned: StoreBundle }`; the trait shape itself does not change, only the framing around it).
