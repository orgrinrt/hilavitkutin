# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (6.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 3.15 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if dominates: 100% faster than the next best (branch_cmov)

branch_if (3.15 us) leads branch_cmov (6.29 us) by 100%, a clear separation rather than a photo finish. CV 10.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### branch_if beats baseline by 50% (significant)

branch_if is -3.15 us (50%) faster than baseline branch_cmov, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: branch_if** at 3145.7 ns median (-50.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.00x (fastest 3145.7 ns, slowest 6290.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Branchful if/else vs branchless cmov/csel under data-dependent input |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x76543210fedcba98 |
| Counter frequency | 2112793168 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| branch_cmov | 8019ns | 8046ns | 7190ns | 8018ns | 8851ns | base |
| branch_if | 4835ns | 4878ns | 4328ns | 4835ns | 5344ns | -39.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 6303ns | 5650ns | 6955ns | base | 0.650 |
| branch_if | 3113ns | 2784ns | 3438ns | -50.61% | 1.316 |

## Performance model

- Peak throughput: **1.471 Gops/s** (branch_if; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.651 | 44.3% |
| branch_if | 1.302 | 88.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 8019ns | 8019ns | base |
| branch_if | 4835ns | 4835ns | -39.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 6290ns | base | --- | [5684, 6919] | --- | --- | --- | --- |
| branch_if | 3146ns | -3146.9ns (-50.0%) | [-3501, -2893]ns | [2817, 3415] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 5674ns | -51.1% |
| 1 | 2 | 5677ns | -39.8% |
| 1 | 3 | 5672ns | -50.7% |
| 1 | 4 | 6944ns | -59.8% |
| 1 | 5 | 6940ns | -50.6% |
| 1 | 6 | 6944ns | -59.4% |
| 1 | 7 | 5652ns | -39.5% |
| 1 | 8 | 6939ns | -50.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | 0.114 | ok |
| branch_if | -0.373 | moderate- |

**Consistency summary:**

- **branch_if**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 21.7ns | 6303.4ns | 0.3% |  |
| branch_if | 22.1ns | 3113.2ns | 0.7% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 5649.6-6955.1 ns)
   5649.6 |######################################
   5714.9 |
   5780.2 |
   5845.4 |
   5910.7 |
   5976.0 |
   6041.3 |
   6106.5 |
   6171.8 |
   6237.1 |
   6302.4 |
   6367.6 |
   6432.9 |
   6498.2 |
   6563.5 |
   6628.7 |
   6694.0 |
   6759.3 |
   6824.5 |
   6889.8 |########################################
  (12 below, 8 above range)

branch_if (n=160, range 2784.1-3437.6 ns)
   2784.1 |#########################
   2816.8 |##########
   2849.5 |
   2882.2 |
   2914.8 |
   2947.5 |
   2980.2 |
   3012.9 |
   3045.5 |
   3078.2 |
   3110.9 |
   3143.6 |
   3176.2 |
   3208.9 |
   3241.6 |
   3274.3 |
   3306.9 |
   3339.6 |
   3372.3 |
   3405.0 |########################################
  (14 below, 9 above range)

```
