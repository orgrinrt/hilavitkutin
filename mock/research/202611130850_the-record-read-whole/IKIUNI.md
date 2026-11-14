# What ikiuni holds on buffering

ikiuni is the engine concept repository that weighed hilavitkutin against
bevy. It is a reference and not part of hilavitkutin's record. Op said on
13 November 2026 that the buffering design he recalls may have been
discussed there. The passages below are copied from `orgrinrt/ikiuni`,
branch `dev`, commit `634be3b`, by a command, each under the file and lines
it comes from. Rows that rest on them name ikiuni in their id and are marked
as ikiuni's.

The search was for double, triple and multi buffering, buffer swaps and
maintenance units, over `mock/PRINCIPLES.md.tmpl`, the canonical design
under `mock/research/0000_CANONICAL_DESIGN_FIRSTMOST_REFERENCE/` and the
imported design history under
`mock/research/202606221200_imported-design-history/`. Double buffering
occurs in 20 files. Triple buffering, multi buffering and a maintenance unit
occur in none.

## `mock/PRINCIPLES.md.tmpl`, lines 6 to 9

> - **Determinism is an invariant.** Authoritative state uses fixed-point and integer carriers; no floats
>   decide authoritative outcomes. Randomness is counter-keyed per substream. State changes apply at the
>   frame barrier through a double-buffer swap, one journal entry per change. A seed plus its journal
>   replays bit-identically.

## `mock/research/0000_CANONICAL_DESIGN_FIRSTMOST_REFERENCE/README.md`, lines 3 to 5

> This is a verbatim copy of the ikiuni engine's canonical design: the flat `NNN_` corpus, the oracle for
> what the engine is and should be. When code and design conflict, this design is the target and the code
> is the thing to advance.

## `mock/research/0000_CANONICAL_DESIGN_FIRSTMOST_REFERENCE/000_FUNDAMENTALS_INDEX.md`, lines 177 to 186

> A Mutation never takes effect in place. It applies at the next frame barrier: the frame a tick computes
> over is read-only for the whole of that computation, and every Mutation the tick produces is published
> together into the next frame by a double-buffer swap. A tick therefore reads the previous committed frame
> as an immutable input and writes the next frame as output, and no reader within a tick observes another
> reader's write. This application-timing law is what makes the order among a tick's own Mutations a pure
> within-frame resolution (the provenance leaf ordering, 100) rather than a read-write race. It holds
> whether the engine runs in one process or replicates the frame across many hosts: a committed frame plus
> an ordered set of Mutations determines the next frame identically wherever that set is applied. The law
> is intrinsic to the Mutation primitive, not a property of any distribution; the execution machinery that
> exploits it (replication, worker offload, replay) lives in 200.

## `mock/research/0000_CANONICAL_DESIGN_FIRSTMOST_REFERENCE/100_ENGINE_CORE_INDEX.md`, lines 635 to 638

> An
> Action's atomicity is emergent, not transacted: a foresighted Agent's Action is gated once up front, its
> Mutations are pure and total, and the tick's double-buffer swap publishes them together, so there is no
> write-buffer and no rollback to maintain. The impulsive, partly-completed case is not a rollback of an

## `mock/research/0000_CANONICAL_DESIGN_FIRSTMOST_REFERENCE/200_ENGINE_STD_INDEX.md`, lines 25 to 25

> | **Tick loop** | The deterministic advance of one frame: a reserved frame-open bracket, a dependency-ordered interior, a reserved frame-close bracket. Frame-open ingests the committed frame and delivers the clock and batch constants; the interior runs every system in the topological order of its declared data dependencies, with no fixed phase set; frame-close drains the Mutation queue, runs the conservation check, swaps the double buffer, and seals the frame. | fixed-timestep schedule |

## `mock/research/202606221200_imported-design-history/design-archive/2026-06-22-first_canonical_design_done/2026-06-19-settling-200-engine-std.md`, lines 51 to 57

> - **Frame-open and frame-close are reserved structural brackets, not systems.** They ARE the next-frame
>   keystone (000) made concrete, not nodes with empty dependency lists. Frame-open ingests the committed
>   frame, delivers the clock and batch constants (200:47), and faults in any queued deltas. Frame-close
>   drains the Mutation queue (the queue-during/drain-after seam, 200:23), runs the conservation check
>   (200:43), swaps the double buffer, and seals the frame. Content systems schedule only in the interior
>   BETWEEN the brackets and cannot, by construction, order themselves across a bracket. A mis-declared
>   dependency can only mis-order work within the frame, never move it across the barrier that defines the

## `mock/research/202606221200_imported-design-history/design-research/2026-06-19-replicated-state-stateless-compute-model.md`, lines 19 to 21

> - **Next-frame effect (double buffering).** All computation this tick reads the PREVIOUS committed
>   frame, immutable for the whole tick. Output is Mutations applied to the NEXT frame. The committed frame
>   is the only read source this tick, for every worker and the storyteller.

## `mock/research/202606221200_imported-design-history/design-archive/2026-06-22-first_canonical_design_done/2026-06-16-data-oriented-engine-direction.md`, lines 59 to 65

> ## ECS substrate: DECIDED = bevy_ecs (2026-06-16, user call)
>
> `bevy_ecs` used standalone, with rendering, GUI, windowing, and HID input ditched entirely. The ideal
> would be the project's own `hilavitkutin` + `arvo` + `notko` stack, but that is not ready, so the
> rebuild works from existing options. `bevy_ecs` is a proven staple for the user's PoC and ideation
> work; stripped of the render and input layers it is a lean, mature, parallel, archetypal-columnar ECS
> core, which is the one part needed here and the part bevy does almost perfectly.

## `mock/research/202606221200_imported-design-history/design-archive/2026-06-22-first_canonical_design_done/2026-06-16-data-oriented-engine-direction.md`, lines 74 to 79

> ## Contracts above bevy: bevy is one swappable engine impl (2026-06-16, user call)
>
> The models and contracts stay authoritative; `bevy_ecs` is one implementation that runs them, not the
> foundation. There is an abstract `runtime` / `engine` contract that bevy satisfies, so a later
> hilavitkutin-stack engine can satisfy the same contract and swap bevy out with no change to the models,
> the contracts, or the systems written against them. The contracts (traits) and models (concrete types
