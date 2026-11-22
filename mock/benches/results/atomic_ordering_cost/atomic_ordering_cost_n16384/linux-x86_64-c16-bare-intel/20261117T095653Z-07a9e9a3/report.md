# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (14.27 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_seqcst at 8.54 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_relaxed beats baseline by 40% (significant)

atomic_relaxed is -5.73 us (40%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (atomic_seqcst, atomic_relaxed) are a dead heat (<1%)

atomic_seqcst (8.54 us) and atomic_relaxed (8.55 us) differ by 0.14%, inside the noise, even though the wider field spreads 67.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### atomic_acquire is inconsistent: worst-20% is 3.1x its best-20%

atomic_acquire's best 20% of batches run at 14.25 us but its worst 20% at 44.44 us (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: atomic_seqcst** at 8538.7 ns median (-40.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.67x (fastest 8538.7 ns, slowest 14271.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x0ddc0ffee055b011 |
| Counter frequency | 2112683200 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| atomic_acquire | 22828ns | 15816ns | 15794ns | 16246ns | 49608ns | base |
| atomic_relaxed | 10884ns | 10097ns | 10069ns | 10563ns | 12662ns | -52.32% |
| atomic_seqcst | 10332ns | 10085ns | 10070ns | 10087ns | 11330ns | -54.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 20524ns | 14251ns | 44437ns | base | 0.798 |
| atomic_relaxed | 9202ns | 8525ns | 10652ns | -55.17% | 1.781 |
| atomic_seqcst | 8746ns | 8525ns | 9584ns | -57.39% | 1.873 |

## Performance model

- Peak throughput: **1.922 Gops/s** (atomic_seqcst; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.148 | 59.7% |
| atomic_relaxed | 1.916 | 99.7% |
| atomic_seqcst | 1.919 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 22828ns | 22828ns | base |
| atomic_relaxed | 10884ns | 10884ns | -52.32% |
| atomic_seqcst | 10332ns | 10332ns | -54.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 14271ns | base | --- | [14264, 14293] | --- | --- | --- | --- |
| atomic_relaxed | 8551ns | -5733.7ns (-40.2%) | [-5744, -5728]ns | [8544, 8573] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 8539ns | -5725.8ns (-40.1%) | [-5736, -5724]ns | [8536, 8541] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 14264ns | -40.2% | -40.1% |
| 1 | 2 | 18104ns | -52.8% | -52.9% |
| 1 | 3 | 14501ns | -41.1% | -40.8% |
| 1 | 4 | 14262ns | -40.1% | -40.1% |
| 1 | 5 | 14267ns | -40.1% | -40.2% |
| 1 | 6 | 14266ns | -40.1% | -40.1% |
| 1 | 7 | 14254ns | -26.8% | -40.1% |
| 1 | 8 | 17428ns | -40.2% | -51.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.198 | ok |
| atomic_relaxed | 0.457 | moderate+ |
| atomic_seqcst | -0.139 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 29.6ns | 20524.5ns | 0.1% |  |
| atomic_relaxed | 21.0ns | 9201.8ns | 0.2% |  |
| atomic_seqcst | 19.6ns | 8746.0ns | 0.2% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 14250.7-44436.6 ns)
  14250.7 |########################################
  15759.9 |#
  17269.2 |##########
  18778.5 |
  20287.8 |
  21797.1 |
  23306.4 |
  24815.7 |
  26325.0 |
  27834.3 |
  29343.6 |
  30852.9 |
  32362.2 |
  33871.5 |
  35380.8 |
  36890.1 |
  38399.4 |
  39908.7 |
  41418.0 |
  42927.3 |
  (15 below, 8 above range)

atomic_relaxed (n=160, range 8525.1-10651.8 ns)
   8525.1 |########################################
   8631.4 |
   8737.7 |####
   8844.1 |
   8950.4 |
   9056.8 |
   9163.1 |
   9269.4 |
   9375.8 |
   9482.1 |
   9588.4 |
   9694.8 |
   9801.1 |
   9907.5 |
  10013.8 |
  10120.1 |
  10226.5 |
  10332.8 |##############
  10439.1 |#####
  10545.5 |
  (13 below, 6 above range)

atomic_seqcst (n=160, range 8525.0-9583.8 ns)
   8525.0 |########################################
   8577.9 |#
   8630.9 |#
   8683.8 |
   8736.7 |
   8789.7 |
   8842.6 |
   8895.6 |
   8948.5 |
   9001.4 |
   9054.4 |
   9107.3 |
   9160.3 |
   9213.2 |
   9266.1 |
   9319.1 |
   9372.0 |
   9425.0 |
   9477.9 |
   9530.8 |
  (15 below, 8 above range)

```

## Diagnostics

- **atomic_acquire**: CV=122.2% (high variance, measurements may be unstable)
- **atomic_acquire**: worst_20/best_20 = 3.1x (possible bimodal distribution)
