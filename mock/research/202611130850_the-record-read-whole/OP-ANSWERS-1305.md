# What op answered on 13 November 2026 at 13:20 on the swap spec

This note holds the question put to the maintainer on 13 November 2026 at
13:20 on whether the resource swap spec may land, with the options it was
asked with and the answer he gave. The text below the line is copied line for
line from the capture kept in the workspace repository,
`.data/op-responses/202611131305_op-on-yeah-if-the-double-buffering-is-in-fact.md`,
lines 140 to 168, and nothing in it is reworded. The registry's rows on the
swap spec cite this note.

---

## Asked, answered at 13:20

### Swap spec

  ```text
  hilavitkutin resource swap (replace_value, replace_resource; spec S1 to S7): built, tested and benched in July, then held for your ratification, which never came. On dev today both methods discard their argument. It is finished and green on work/recover-landing-swap (154744e). The benches: installing a value costs 2 ns at 64 B, 85 ns at 1 KiB and 1.81 us at 64 KiB. A value swap pays only for the records it makes dirty; a swap that changes the plan pays the plan band once; neither leaks into the other. Two known gaps, ticketed: the plan-dirty bit is keyed by the store's position where S2 says the plan-affecting id, and a fused frame ignores the bit. Your last recorded word, 19 July, was 'revert it now', said of an earlier install. May it land?
  ```

Options offered:

1. Ratify and land

   S1 to S7 ratified as written; the swap branch lands on dev now and the two gaps stay as tickets.

2. Land as provisional

   Lands on dev now under a provisional stamp; ratification waits until the two gaps are fixed.

3. Ratify with changes

   Ratify once the gaps are fixed first: key the bit by the plan-affecting id and have fused frames honour it, then land.

4. Keep holding

   Stays parked on its branch; dev keeps discarding the argument.

Chosen:

- Land as provisional
