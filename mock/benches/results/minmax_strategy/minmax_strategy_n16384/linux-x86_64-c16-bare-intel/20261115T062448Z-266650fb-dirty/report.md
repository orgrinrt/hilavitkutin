# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (minmax_branch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline minmax_branch has the worst median (720 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest minmax_branchless at 670 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### minmax_branchless is fastest but the noisiest (CV 64.5%)

minmax_branchless wins on median (670 ns) yet has the highest variance (CV 64.5%), while minmax_intrinsic is the steadiest (CV 60.5%, 720 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (50 ns) is smaller than the fastest variant's own run-to-run std-dev (432 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader minmax_branchless vs stability leader minmax_intrinsic (+7% speed for 1.1x steadier)

minmax_branchless is fastest (670 ns, CV 64.5%); minmax_intrinsic gives up 7.5% median for 1.1x lower variance (CV 60.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### minmax_branchless is inconsistent: worst-20% is 2.5x its best-20%

minmax_branchless's best 20% of batches run at 561 ns but its worst 20% at 1.43 us (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### minmax_intrinsic's edge over baseline is significant but tiny (-1 ns, 0.18%)

minmax_intrinsic differs from baseline minmax_branch by -1 ns (0.18%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: minmax_branchless** at 669.9 ns median (-7.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.08x (fastest 669.9 ns, slowest 720.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5d49a3f1b7c06e82 |
| Counter frequency | 2112457735 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 3177ns | 2602ns | 2559ns | 2599ns | 5528ns | base |
| minmax_branchless | 3031ns | 2553ns | 2099ns | 2545ns | 5423ns | -4.58% |
| minmax_intrinsic | 3073ns | 2600ns | 2312ns | 2598ns | 5260ns | -3.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 867ns | 678ns | 1510ns | base | 18.896 |
| minmax_branchless | 800ns | 561ns | 1427ns | -7.75% | 20.483 |
| minmax_intrinsic | 839ns | 613ns | 1433ns | -3.29% | 19.539 |

## Performance model

- Peak throughput: **29.211 Gops/s** (minmax_branchless; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 22.749 | 77.9% |
| minmax_branchless | 24.457 | 83.7% |
| minmax_intrinsic | 22.752 | 77.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 3177ns | 3177ns | base |
| minmax_branchless | 3031ns | 3031ns | -4.58% |
| minmax_intrinsic | 3073ns | 3073ns | -3.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 720ns | base | --- | [714, 722] | --- | --- | --- | --- |
| minmax_branchless | 670ns | -52.6ns (-7.3%) | [-60, -44]ns | [669, 672] | YES | 0.0000 | 0.0000 | 0 |
| minmax_intrinsic | 720ns | -1.3ns (-0.2%) | [-4, -1]ns | [713, 721] | YES | 0.0031 | 0.0031 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 686ns | -19.0% | -0.0% |
| 1 | 2 | 725ns | -8.9% | -0.4% |
| 1 | 3 | 711ns | -5.8% | +0.1% |
| 1 | 4 | 722ns | -21.7% | -0.1% |
| 1 | 5 | 718ns | -7.6% | +0.8% |
| 1 | 6 | 734ns | -7.3% | -0.5% |
| 1 | 7 | 673ns | -0.3% | -18.0% |
| 1 | 8 | 721ns | -2.7% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.439 | moderate- |
| minmax_branchless | -0.069 | ok |
| minmax_intrinsic | -0.252 | moderate- |

**Consistency summary:**

- **minmax_branchless**: won 8/8, lost 0/8
- **minmax_intrinsic**: won 3/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 29.8ns | 867.0ns | 3.4% |  |
| minmax_branchless | 28.9ns | 799.9ns | 3.6% |  |
| minmax_intrinsic | 28.8ns | 838.5ns | 3.4% |  |

## Distribution (algo ns)

```
minmax_branch (n=160, range 678.5-1509.6 ns)
    678.5 |########################################
    720.0 |########################################
    761.6 |
    803.1 |
    844.7 |
    886.3 |
    927.8 |
    969.4 |
   1010.9 |
   1052.5 |
   1094.0 |
   1135.6 |
   1177.1 |
   1218.7 |
   1260.2 |
   1301.8 |
   1343.4 |
   1384.9 |
   1426.5 |
   1468.0 |
  (16 below, 17 above range)

minmax_branchless (n=160, range 560.9-1426.8 ns)
    560.9 |#########
    604.2 |
    647.5 |########################################
    690.8 |########
    734.1 |
    777.4 |
    820.6 |
    863.9 |
    907.2 |
    950.5 |
    993.8 |
   1037.1 |
   1080.4 |
   1123.7 |
   1167.0 |
   1210.3 |
   1253.6 |
   1296.9 |
   1340.2 |
   1383.5 |
  (14 below, 15 above range)

minmax_intrinsic (n=160, range 613.1-1432.9 ns)
    613.1 |
    654.1 |##########
    695.1 |########################################
    736.1 |#
    777.1 |
    818.1 |
    859.1 |
    900.0 |
    941.0 |
    982.0 |
   1023.0 |
   1064.0 |
   1105.0 |
   1146.0 |
   1186.9 |
   1227.9 |
   1268.9 |
   1309.9 |
   1350.9 |
   1391.9 |
  (17 below, 14 above range)

```

## Diagnostics

- **minmax_branch**: CV=51.9% (high variance, measurements may be unstable)
- **minmax_branchless**: CV=54.0% (high variance, measurements may be unstable)
- **minmax_intrinsic**: CV=52.0% (high variance, measurements may be unstable)
