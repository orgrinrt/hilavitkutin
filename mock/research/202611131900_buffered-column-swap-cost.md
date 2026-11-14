# What a Buffered Column Costs a Frame, Under Each Mode

**Date:** 2026-11-13
**Bench:** `swap_column` in `mock/benches/bench.toml`, arms `cs_plain`, `cs_snap`, `cs_snap_twin`, `cs_exch` under `mock/benches/variants/`
**Results:** `mock/benches/results/swap_column/`
**Asked by:** round `202611130617-h6mb`, ticket `bench_a_close_unit_s_copy_of_a_column_against_an_exchange_of` (ecb940c91d26)

The api design gives a buffered column two modes and takes `Snapshot` as the default with no number under it. Its own words were that which mode costs less at which size is a bench nobody has run. This note is that bench, run once on the branch `work/api-revision` with `dev` at `271909e` merged in, and it stands beside the two benches of a resource value's swap, `swap_cost` and `swap_band`, whose carrier crate it shares.

The question is what a frame pays for a column being buffered, and how much of that is the copy `Snapshot` makes at the boundary. It is asked of one shape only: a writer that writes every record every frame, which is the one shape where both modes are allowed and give the same values.

## What was measured

One timed frame is one call of `Scheduler::run` on one core. In it a producer writes every record of a middle column from a resource value and a column of slots, a reader turns the middle column into an output column, and the boundary runs. Every value is four bytes. The arms differ in the middle column alone. In `cs_plain` it is a plain `Column`, so the reader waits on the producer and reads in the same frame what was just written. In `cs_snap` and `cs_exch` it is a `Buffered` column, so the reader takes the frozen side, the producer writes the live side, and the boundary brings the live side over by a copy of the whole column under `Snapshot` or by turning the two sides under `Exchange`. `cs_snap_twin` is the `Snapshot` arm built a second time under another name, and the distance between the two is the noise of the run: no smaller difference between two arms counts.

Before every timed frame the resource value is installed again with `replace_value`, outside the timed region, so the producer is dispatched in every frame. Both writes are listed under `Written`, so a store counts as changed by the write itself and the reader of a buffered column is dispatched in every frame by the boundary of the frame before. Each frame of every arm therefore runs the producer once and the reader once over all records, and the three carriers settle on one output, which the harness checked byte for byte over 100 seeds at each size before timing.

The sizes 64, 1024 and 8192 each stand for that many times 1024 records: about 64 thousand, one million and eight million records, or 256 KiB, 4 MiB and 32 MiB a column. The profile is release with fat LTO and one codegen unit, each arm its own library in its own process, three passes of thirty runs of ten frames, the whole twice. The machine is a four-core x86-64 Linux virtual machine shared with other work, and its noise is large: read every number with the twin beside it.

## The numbers

Medians of one frame with the 95 % interval of the difference to `cs_plain`, from the statistical comparison table of each findings file.

| records | `cs_plain` | `cs_exch` | `cs_snap` | `cs_snap_twin` |
|---|---|---|---|---|
| 64 K | 22.8 us | 25.3 us, no significant difference | 49.9 us, +27.7 us [+19.3, +31.5] | 30.1 us, +6.5 us [+4.8, +16.2] |
| 1 M | 584.8 us | 725.3 us, +133.7 us [+108.4, +164.7] | 1093.2 us, +543.5 us [+490.5, +569.5] | 1008.5 us, +434.6 us [+385.5, +482.9] |
| 8 M | 10.16 ms | 11.54 ms, +1.27 ms [+0.76, +1.80] | 16.22 ms, +5.76 ms [+5.16, +6.38] | 15.61 ms, +5.23 ms [+4.69, +6.02] |

At one million and at eight million records the arms stand in one sequence: the plain column, then `Exchange`, then `Snapshot`, and the two builds of `Snapshot` stand closer to each other than either stands to `Exchange`. The twins are 85 us apart at one million and 0.6 ms apart at eight million, and every distance read below is larger than that.

`Snapshot` over `Exchange` is 283 to 368 us at one million records and 4.1 to 4.7 ms at eight million, taking each of the two `Snapshot` builds in turn. That is the 4 MiB and the 32 MiB of the column moved at about 11 to 14 GiB a second and at about 7 to 8, which is a speed a copy of memory can have on such a machine, so the copy at the boundary accounts for the distance and nothing else has to. In the `Snapshot` frame the copy is about a third of the time at one million records and about a quarter at eight million. The rest is the two units going over the column, which is what the plain frame is made of: 0.56 ns a record at one million and 1.21 ns at eight million.

`Exchange` over the plain column is 134 us at one million records and 1.27 ms at eight million, 23 % and 12.5 % of the plain frame. It moves no bytes at the boundary, so this part is not a copy.

At 64 thousand records the two `Snapshot` builds are 20 us apart, which is as far as either is from the plain column, and the harness marks the twin as inconsistent between its passes. That size separates nothing on this machine and no reading is taken from it.

The two benches of a resource value's swap were not run again here. A run rewrites their tracked results, and the note `202607201100_swap-semantics-ratification-evidence.md` quotes those. Their numbers are another machine's: on it a frame whose resource value was swapped cost 172.6 us at one million records and 2.61 ms at eight million, against about 10 us for a frame that skips, and the whole of that distance is the units that read the value running again. So a resource's swap costs a frame the work of its readers and no copy that grows with the records, where a buffered column under `Snapshot` costs the work of its readers and one copy of the column. The figures of the two machines are not to be laid side by side.

## What it decides

A writer that writes every record of its buffered column names `Exchange`. It gets back the copy, which here is 0.3 to 0.4 ms a frame for each million four-byte records and a quarter to a third of the frame, and it gets the same values.

`Snapshot` stays the default. It is the one mode that is right under any writer, and the bench gives no reason against that: what it adds is the price of the default, which the design did not have. The price is the copy of the whole column, so the copy for each changed morsel of ticket 8e2e2326d588 is where the default gets cheaper, and a writer that touches a small part of its column pays the most for the lack of it.

A column is buffered where its readers must see the frame before, and not as a kind to reach for first. Under `Exchange` too the frame was 12 to 23 % longer than over a plain column.

## What it does not decide

Only a total writer was measured. Under a writer that leaves records alone `Exchange` is not allowed at all, and `Snapshot` still copies the whole column where it could copy only the morsels that changed (ticket 8e2e2326d588). That copy is the larger part of what `Snapshot` costs here, so a copy per changed morsel would move these numbers for any writer that touches a part of its column, and this bench says nothing of by how much.

The writes were listed under `Written`. A column listed bare or under `Differs` compares before it stores, and a frame that writes back what was there brings nothing in at the boundary. That path pays a compare and saves the copy, and it was not timed.

The frame was run by `Scheduler::run` on one core. On `run_parallel` the boundary runs on the calling thread once every worker has parked again (`scheduler/run_parallel.rs`, the call of `bring_in_changes`), so the copy would stay one thread's work while more cores shorten everything else in the frame. Nothing here measures it.

Why `Exchange` costs more than the plain column is not shown. A reading that fits is that the reader of a plain column reads what the producer wrote a window ago and still holds in cache, where the reader of a buffered column reads the frozen side from memory, and that the two units fuse over a plain column and not over a buffered one. No arm separates these and no counter was read.

The values are four bytes. A wider value moves more bytes for each record in the copy and in the two units alike, and the ratio was not checked at another width. The run is one run on a shared virtual machine: the twin arms stand apart by the amounts in the table, and the ranking of the arms is the result, more than any one figure.
