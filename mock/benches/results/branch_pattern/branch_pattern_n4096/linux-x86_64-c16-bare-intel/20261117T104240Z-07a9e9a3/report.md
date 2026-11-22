# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (6.91 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 3.42 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if dominates: 102% faster than the next best (branch_cmov)

branch_if (3.42 us) leads branch_cmov (6.91 us) by 102%, a clear separation rather than a photo finish. CV 8.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### branch_if beats baseline by 51% (significant)

branch_if is -3.49 us (51%) faster than baseline branch_cmov, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### branch_if is fastest but the noisiest (CV 8.1%)

branch_if wins on median (3.42 us) yet has the highest variance (CV 8.1%), while branch_cmov is the steadiest (CV 7.9%, 6.91 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: branch_if** at 3417.2 ns median (-50.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.02x (fastest 3417.2 ns, slowest 6905.5 ns)

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
| Counter frequency | 2112670986 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| branch_cmov | 8405ns | 8793ns | 7195ns | 8663ns | 8839ns | base |
| branch_if | 5085ns | 5316ns | 4339ns | 5239ns | 5370ns | -39.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 6599ns | 5651ns | 6935ns | base | 0.621 |
| branch_if | 3269ns | 2787ns | 3448ns | -50.46% | 1.253 |

## Performance model

- Peak throughput: **1.470 Gops/s** (branch_if; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.593 | 40.4% |
| branch_if | 1.199 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 8405ns | 8405ns | base |
| branch_if | 5085ns | 5085ns | -39.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 6906ns | base | --- | [6902, 6910] | --- | --- | --- | --- |
| branch_if | 3417ns | -3491.8ns (-50.6%) | [-3497, -3477]ns | [3417, 3422] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 6911ns | -59.9% |
| 1 | 2 | 6913ns | -50.5% |
| 1 | 3 | 5658ns | -39.6% |
| 1 | 4 | 6921ns | -50.7% |
| 1 | 5 | 6914ns | -59.4% |
| 1 | 6 | 6907ns | -50.2% |
| 1 | 7 | 5651ns | -39.4% |
| 1 | 8 | 6916ns | -50.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.376 | moderate- |
| branch_if | -0.216 | moderate- |

**Consistency summary:**

- **branch_if**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 22.8ns | 6599.2ns | 0.3% |  |
| branch_if | 23.6ns | 3269.1ns | 0.7% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 5651.0-6934.6 ns)
   5651.0 |########
   5715.1 |
   5779.3 |
   5843.5 |
   5907.7 |
   5971.9 |
   6036.1 |
   6100.2 |
   6164.4 |
   6228.6 |
   6292.8 |
   6357.0 |
   6421.1 |
   6485.3 |
   6549.5 |
   6613.7 |
   6677.9 |
   6742.0 |
   6806.2 |
   6870.4 |########################################
  (17 below, 11 above range)

branch_if (n=160, range 2787.3-3447.8 ns)
   2787.3 |########
   2820.3 |
   2853.4 |
   2886.4 |
   2919.4 |
   2952.4 |
   2985.5 |
   3018.5 |
   3051.5 |
   3084.5 |
   3117.6 |
   3150.6 |
   3183.6 |
   3216.7 |
   3249.7 |
   3282.7 |
   3315.7 |
   3348.8 |
   3381.8 |#######
   3414.8 |########################################
  (20 below, 5 above range)

```
