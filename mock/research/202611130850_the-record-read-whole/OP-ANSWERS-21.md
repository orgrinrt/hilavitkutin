# Op answers 21, 13 November about 12:25 UTC: the engine benches so far are a baseline, and no call rests on them

Op, verbatim, following op-answers-20:

"And to add, those answers don't yet include any of the actual headline features like fusing and the adaptive trunk/fiber/morsel crunching on the datadlow-ish schedule. It's bare bones, barely lowers to anything interesting yet. So no calls should be made based on those yet, they are a baseline to compare against then. I mean the benches that bench hila against a hand-written optimized path and just regular old rust on no_std and on std etc whatever we have written so far (probably very limited comparison still) and the threaded variants of each"

Intent, in coord's words:

- The benches that compare hilavitkutin against a hand-written optimised path, plain Rust on no_std and on std, and the threaded variants of each, measure an engine that has none of its headline features yet: no fusing, no adaptive trunk, fiber and morsel crunching on the dataflow schedule.
- No call is made on those numbers yet: no design decision, no verdict on the engine, no ruling.
- They are the baseline the engine is compared against once those features exist.

Kept for provenance, not an intent: "probably very limited comparison still", "whatever we have written so far".
