# Sketch findings: a change mark carried along a gather and a fold agrees with recomputation at every grain, and on loru's site the morsel grain reruns as much as the store grain

**Date:** 2026-11-20
**Outcome:** WORKS
**Unblocks:** the shape questions of mock/research/notes/202611200350_per-record-marks-expert-read.md. Nothing is priced. Records are counted.

## Hypothesis

A change mark can be carried across two reads while equalling from-scratch recomputation after every frame. The reads are a gather of a string by a key column and a fold of twenty contiguous nodes into a page. The mark varies over:
- grain: the store, a window of W records, or one record;
- direction: push through a reverse relation, or pull through the forward relation and a generation compare;
- cutoff: structural, or differs.

Demand frames are included. Four known failure modes must be caught by the same differential, or it proves nothing.

## What was tried, in order

1. src/lib.rs is a plain-Rust model with no engine dependency, reproducing loru-bench-data's site layout for one locale.
   - At record grain, the reverse relation of the key column is three columns: a head per string, and a previous and next link per node. They are fixed at build, and a rekey costs two link writes.
   - At coarser grains it is one bit per (string window, node window) pair, rebuilt per rekey from that window's keys.
   - The fold's reverse is range arithmetic.
   - Pull reads the reader window's keys and holds no reverse relation.
2. tests/differential.rs runs the matrix: grain (store, record, and windows 1, 2, 3, 7, 19, 20, 21, 40, 64, 1000), push and pull, structural and differs; at 1, 3, 7 and 13 pages; 24 seeds of 300 random steps each. The steps are string edits (a fifth of them rewrite the same value), rekeys, kind edits, demand frames and full frames. Every frame is held to recomputation. Passed: 1,152 runs.
3. Mutants (first catch of each in output/tests.txt):
   - Stale inverse after a rekey: caught under push at record grain (1 page, seed 0, step 75). Harmless at store grain and under pull, and the test asserts that too.
   - A demand frame that clears all marks: caught under push and under pull (3 pages, seed 0, step 26).
   - A fold reverse missing the last page: caught under push (1 page, seed 0, step 11).
   - A cutoff that drops every change: caught at every grain under both carries.
4. src/main.rs (binary counts) writes output/counts.tsv: records recomputed and relation entries touched, for six scenarios after a cold frame, at 300 and 3000 pages, with each row checked against recomputation.
5. The engine's catalogued red was run with --ignored (output/engine-red-host-import.txt): "only the window holding record 0 reruns, left: 4, right: 1".

## The result

At 300 pages, windows of 2304 and 4608 rerun 6000 nodes and 300 pages for one page-unique edit, the same as the store mark. Record grain under push reruns 1 and 1, and under pull it reruns 1 and 1 while reading 6000 key entries. A gather target's window inflates the cone (6912 nodes at W 2304 and 3000 pages). A demand frame needs forward relations only and must keep the marks of what it did not run. The differs cutoff stops a same-value rewrite at the node hash.

## What is NOT established

Nothing about hilavitkutin beyond the one red run. There is no time and no cost per record: a link write, a key read and a hash recompute are counted alike. Not modelled: several locales, run_parallel, differing windows across fibers, outputs of no fixed length, extent changes, and an index column written by a unit. The W values assume a 48 KiB L1 and 8 or 16 written bytes per record.

## Commands

```
cd mock/research/sketches/202611200350_per-record-marks
CARGO_INCREMENTAL=0 cargo test --release -- --nocapture --test-threads=1
CARGO_INCREMENTAL=0 cargo run --release --bin counts > output/counts.tsv
```

Toolchain: rustc 1.98.0-nightly (57d06900f 2026-05-27), the repository pin.
