# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (minmax_branch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline minmax_branch has the worst median (709 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest minmax_branchless at 671 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### minmax_branchless is fastest but the noisiest (CV 67.4%)

minmax_branchless wins on median (671 ns) yet has the highest variance (CV 67.4%), while minmax_branch is the steadiest (CV 59.6%, 709 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (39 ns) is smaller than the fastest variant's own run-to-run std-dev (452 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader minmax_branchless vs stability leader minmax_branch (+6% speed for 1.1x steadier)

minmax_branchless is fastest (671 ns, CV 67.4%); minmax_branch gives up 5.8% median for 1.1x lower variance (CV 59.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### minmax_branchless is inconsistent: worst-20% is 2.7x its best-20%

minmax_branchless's best 20% of batches run at 564 ns but its worst 20% at 1.52 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### minmax_branchless's edge over baseline is significant but tiny (-6 ns, 0.89%)

minmax_branchless differs from baseline minmax_branch by -6 ns (0.89%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: minmax_branchless** at 670.7 ns median (-5.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 670.7 ns, slowest 709.3 ns)

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
| Counter frequency | 2112833092 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 2921ns | 2589ns | 2097ns | 2486ns | 5048ns | base |
| minmax_branchless | 3107ns | 2554ns | 2105ns | 2551ns | 5777ns | +6.37% |
| minmax_intrinsic | 2963ns | 2580ns | 2125ns | 2566ns | 4993ns | +1.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 794ns | 561ns | 1376ns | base | 20.625 |
| minmax_branchless | 819ns | 564ns | 1518ns | +3.13% | 19.999 |
| minmax_intrinsic | 814ns | 586ns | 1390ns | +2.52% | 20.118 |

## Performance model

- Peak throughput: **29.226 Gops/s** (minmax_branch; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 23.099 | 79.0% |
| minmax_branchless | 24.430 | 83.6% |
| minmax_intrinsic | 23.583 | 80.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2921ns | 2921ns | base |
| minmax_branchless | 3107ns | 3107ns | +6.37% |
| minmax_intrinsic | 2963ns | 2963ns | +1.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 709ns | base | --- | [674, 714] | --- | --- | --- | --- |
| minmax_branchless | 671ns | -6.3ns (-0.9%) | [-30, -3]ns | [670, 672] | YES | 0.0055 | 0.0055 | 0 |
| minmax_intrinsic | 695ns | +5.3ns (+0.7%) | [+1, +12]ns | [685, 721] | YES | 0.0009 | 0.0004 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 561ns | -1.1% | +21.9% |
| 1 | 2 | 721ns | -8.3% | +0.7% |
| 1 | 3 | 710ns | -5.6% | -17.9% |
| 1 | 4 | 560ns | +23.3% | +5.8% |
| 1 | 5 | 597ns | +11.1% | +22.1% |
| 1 | 6 | 741ns | -8.1% | +0.4% |
| 1 | 7 | 674ns | -0.4% | +0.1% |
| 1 | 8 | 723ns | -20.7% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.119 | ok |
| minmax_branchless | 0.031 | ok |
| minmax_intrinsic | 0.090 | ok |

**Consistency summary:**

- **minmax_branchless**: won 6/8, lost 2/8
- **minmax_intrinsic**: won 1/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 27.6ns | 794.4ns | 3.5% |  |
| minmax_branchless | 29.6ns | 819.3ns | 3.6% |  |
| minmax_intrinsic | 27.8ns | 814.4ns | 3.4% |  |

## Distribution (algo ns)

```
minmax_branch (n=160, range 560.6-1376.2 ns)
    560.6 |#################################
    601.4 |#
    642.2 |####################
    682.9 |########################################
    723.7 |#############################
    764.5 |#
    805.3 |
    846.1 |
    886.9 |
    927.6 |
    968.4 |
   1009.2 |#
   1050.0 |
   1090.8 |
   1131.5 |#
   1172.3 |
   1213.1 |
   1253.9 |
   1294.7 |
   1335.4 |
  (20 below, 15 above range)

minmax_branchless (n=160, range 564.1-1518.4 ns)
    564.1 |######
    611.9 |
    659.6 |########################################
    707.3 |
    755.0 |
    802.7 |
    850.4 |
    898.1 |
    945.8 |
    993.6 |
   1041.3 |
   1089.0 |
   1136.7 |
   1184.4 |
   1232.1 |
   1279.8 |
   1327.6 |
   1375.3 |
   1423.0 |
   1470.7 |
  (16 below, 17 above range)

minmax_intrinsic (n=160, range 585.8-1390.5 ns)
    585.8 |##############
    626.0 |
    666.2 |################################
    706.5 |########################################
    746.7 |###
    786.9 |
    827.2 |
    867.4 |
    907.6 |
    947.9 |
    988.1 |
   1028.3 |
   1068.6 |
   1108.8 |
   1149.0 |
   1189.3 |
   1229.5 |
   1269.7 |
   1310.0 |
   1350.2 |
  (16 below, 13 above range)

```

## Diagnostics

- **minmax_branch**: CV=53.2% (high variance, measurements may be unstable)
- **minmax_branchless**: CV=55.2% (high variance, measurements may be unstable)
- **minmax_intrinsic**: CV=53.5% (high variance, measurements may be unstable)
