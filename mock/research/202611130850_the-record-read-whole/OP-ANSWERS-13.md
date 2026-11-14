# Op's answers, batch 13, 13 November 2026 (about 10:35 UTC)

Asked after `hv-canon-registry` posted ANSWERS.md (the eight questions answered from hilavitkutin's record, read at 14 of 50 batches). Each entry: the question, the options, his words verbatim, the coordinator's reading (not his words).

## 1. Who rotates a buffered store

Asked: who rotates a buffered store in hilavitkutin (a store read as the value it held at frame start while the next value is written)? The founding record gives read and write only and leaves double and triple buffering to loimu's layer as two or three declared stores; it names a store wrapper in the access set as the way a new access form would enter, and rates loimu's swap buffer as directly portable.

Options: [a: Wrapper in the access set: `hilavitkutin-api` gains a store wrapper naming a store as buffered two or three deep; the schedule knows frozen and live, puts no edge between reader of old and writer of new, rotates at the pass boundary] [b: Consumer rotates: hilavitkutin stays read and write only, the previous and next value are two declared stores the consumer swaps between passes] [c: Opt-in crate over the api: the core stays read and write only, a separate opt-in crate offers the buffered wrapper and rotation built from two declared stores].

Answer, verbatim: "I am almost sure this was researched and spiked and even benched perhaps, for multibuffering and swaps, and the hilavitkutin should also have specs for the maintenance work units already that do this kind of shit and adaptation and all. I am now wondering if the inlander branch or a few of them contain all this work entirely unlanded so you guys just have only stale shit at hand?"

Reading: no option chosen. He recalls research, sketches and perhaps benches on multi-buffering and swaps, and specs for maintenance work units that do rotation and adaptation, and suspects that work sits on unlanded branches so that `dev` is stale ("inlander" read as unlanded). Checked the same minute, facts about the repository and not his words: `origin/recover/unlanded-rounds-and-benches` (13 commits of 30 August 2026, 592 files, 38615 insertions over `dev`, 96 conflicts against `dev` today) was never landed and carries swap benches (`mock/benches/results/swap_band`, `swap_cost`, variants `sb_none`, `sb_plan`, `sb_value`), 14 design round files, 19 research files, sketches, 41 engine tests and source under `plan/` and `dispatch/`. On `dev` itself the round `202605101036` (topics adapt-subsystem, plan-algorithm-chain, resource-resolution, thread-pool) specifies the meta work unit `AdaptWu` and a flipped-index double buffer for its published snapshot (plan-algorithm-chain 1181-1208); the registry read had not reached these when ANSWERS.md was written. The question is not re-asked: it is answered from that work once read, and the recovery branch is landed.

## 2. Record sets of different counts

Asked: how do record sets of different counts live together in hilavitkutin (beings, tiles, items in one frame)? The founding record has one count per schedule and separate schedules per set of columns held by the consumer; it has no passage on one schedule holding several counts.

Options: [a: Several sets in one schedule, each contiguous with its own count as a plan parameter, a fiber belonging to one set, a cross-set reach a declared read the plan orders] [b: One schedule per set, as the founding round says, crossings through resources or accumulators the consumer carries] [c: Bench both first: sketch and bench both shapes on kaski-like data before the api takes either].

Answer, verbatim: "Bench both first"

Reading: option c. The api revision takes neither shape until a sketch and a bench of both exist.

## 3. Tie-break between two writers of one store

Options: [a: Hints, dependents, registration: his earlier answer names only the fallback the spec left open, the most-dependents step stays] [b: Hints, registration: his earlier answer is the whole rule].

Answer, verbatim: "Hints, dependents, registration"

Reading: option a. Scheduling hints, then most downstream dependents, then registration order.

## 4. Whether the accumulator is a fourth store kind

Options: [a: A column with an append write: three kinds stand, the accumulator is a column whose write form is append, marked by a wrapper in the access set] [b: A fourth store kind: resource, column, accumulator, virtual].

Answer, verbatim: "Whichever makes most sense typestate and plumbing wise"

Reading: no preference between the two as such; the api revision decides by which gives the cleaner typestate and plumbing, and says why. Not returned to him as a question.
