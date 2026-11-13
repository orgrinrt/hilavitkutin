# c7: a packed column filled from a queue at the close

Spike, seat 01 reply, round 202611131951. Checks one sentence of seat 02's reconciliation (attack 8 and
merged call M11 option a, `02_tiark-rompf.md`): "A packed column written only through a queue drained
at the close has one writer by construction, so the split rule is needed only for a unit writing a
packed column directly." Op's description of the queue is "queued up ops that drain verctoeised and
optimized at the close" (op-answers-25, item 3). A drain that is vectorised and spread over cores is a
parallel writer of the packed words, so the question is what the drain's own split needs.

A 1-bit column of 136 records (three 64-bit words). Each close, a queue holds one "set record r" op
for every record, in a fixed pseudo-random order no drainer chose. Two threads start together on the
same queue; the word write is c2's model, a relaxed load then a relaxed store, except in the
`fetch_or` arm. Every test asserts that no close lost a write. 200 000 closes per arm, release build,
nightly-2026-05-28, x86_64 linux, 4 hardware threads. Pure std; no hilavitkutin crate is involved,
because the trunk has no packed path and no drain to race (as with c2, this proves that the hazard
exists, not that the engine has it).

| Test | Result | Closes that lost a write |
|---|---|---|
| `a_single_drainer_loses_nothing` | green | 0 |
| `two_drainers_split_by_queue_position_lose_nothing` | red | 79522 (43603 and 132462 on earlier runs) |
| `two_drainers_split_by_record_at_68_lose_nothing` | red | 155965 (166523 and 148774 on earlier runs) |
| `two_drainers_split_by_record_at_64_lose_nothing` | green | 0 |
| `two_drainers_split_by_queue_position_with_fetch_or_lose_nothing` | green | 0 |

Reading: the queue removes the hazard only when one core drains it. A drain split by queue position
races on every word, so no record rounding helps it; it needs an atomic word write. A drain split by
record range needs the same word-multiple rule as a unit writing directly (68 is 4-aligned like the
trunk's `& !3` window, 64 is word-aligned). So the rule moves to whoever writes the word in parallel;
it does not go away. The `fetch_or` arm holds for a 1-bit set because OR commutes; a field wider than
one bit written with overwrite semantics would need a compare-and-swap loop and an order between ops
on the same record, which this spike does not test.

Run: `cargo +nightly-2026-05-28 test --release --no-fail-fast -- --test-threads=1 --nocapture` in
this directory. Output: `OUTPUT-tests.txt` (the third run; the counts vary run to run, the zeros do
not).
