# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (44 ns) is smaller than the fastest variant's own run-to-run std-dev (413 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader minmax_branchless vs stability leader minmax_intrinsic (+7% speed for 1.1x steadier)

minmax_branchless is fastest (664 ns, CV 62.1%); minmax_intrinsic gives up 6.6% median for 1.1x lower variance (CV 57.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### minmax_branch is inconsistent: worst-20% is 2.8x its best-20%

minmax_branch's best 20% of batches run at 569 ns but its worst 20% at 1.60 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: minmax_branchless** at 664.2 ns median (-1.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.07x (fastest 664.2 ns, slowest 708.1 ns)

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
| Counter frequency | 2112775109 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 3063ns | 2566ns | 2113ns | 2432ns | 5907ns | base |
| minmax_branchless | 2944ns | 2557ns | 2095ns | 2482ns | 5181ns | -3.88% |
| minmax_intrinsic | 2899ns | 2590ns | 2104ns | 2495ns | 4905ns | -5.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 831ns | 569ns | 1599ns | base | 19.727 |
| minmax_branchless | 776ns | 552ns | 1367ns | -6.52% | 21.103 |
| minmax_intrinsic | 787ns | 562ns | 1326ns | -5.19% | 20.805 |

## Performance model

- Peak throughput: **29.665 Gops/s** (minmax_branchless; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 24.280 | 81.8% |
| minmax_branchless | 24.665 | 83.1% |
| minmax_intrinsic | 23.138 | 78.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 3063ns | 3063ns | base |
| minmax_branchless | 2944ns | 2944ns | -3.88% |
| minmax_intrinsic | 2899ns | 2899ns | -5.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 675ns | base | --- | [668, 698] | --- | --- | --- | --- |
| minmax_branchless | 664ns | -18.6ns (-2.7%) | [-31, -5]ns | [663, 672] | YES | 0.0000 | 0.0000 | 0 |
| minmax_intrinsic | 708ns | no significant difference | [-2, +18]ns | [685, 712] | no | 0.1788 | 0.1788 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 560ns | -0.8% | +22.2% |
| 1 | 2 | 593ns | +11.5% | +22.2% |
| 1 | 3 | 710ns | -22.8% | +0.1% |
| 1 | 4 | 724ns | -4.5% | +0.1% |
| 1 | 5 | 718ns | -7.5% | -18.1% |
| 1 | 6 | 589ns | -5.4% | +1.4% |
| 1 | 7 | 674ns | -0.4% | -18.2% |
| 1 | 8 | 589ns | +19.1% | +22.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | 0.139 | ok |
| minmax_branchless | -0.341 | moderate- |
| minmax_intrinsic | 0.233 | moderate+ |

**Consistency summary:**

- **minmax_branchless**: won 6/8, lost 2/8
- **minmax_intrinsic**: won 2/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 28.9ns | 830.6ns | 3.5% |  |
| minmax_branchless | 28.1ns | 776.4ns | 3.6% |  |
| minmax_intrinsic | 27.3ns | 787.5ns | 3.5% |  |

## Distribution (algo ns)

```
minmax_branch (n=160, range 569.0-1599.0 ns)
    569.0 |################################
    620.5 |#####
    672.0 |########################################
    723.5 |######
    775.0 |
    826.5 |
    878.0 |
    929.5 |
    981.0 |
   1032.5 |
   1084.0 |
   1135.5 |
   1187.0 |
   1238.5 |
   1290.0 |
   1341.5 |
   1393.0 |
   1444.5 |
   1496.0 |
   1547.5 |
  (19 below, 22 above range)

minmax_branchless (n=160, range 552.3-1366.8 ns)
    552.3 |##########################
    593.0 |
    633.7 |########################################
    674.5 |##############################
    715.2 |
    755.9 |
    796.6 |
    837.4 |
    878.1 |
    918.8 |
    959.6 |
   1000.3 |
   1041.0 |
   1081.7 |
   1122.5 |
   1163.2 |
   1203.9 |
   1244.6 |
   1285.4 |
   1326.1 |
  (14 below, 17 above range)

minmax_intrinsic (n=160, range 562.2-1325.7 ns)
    562.2 |#####################
    600.4 |######
    638.5 |
    676.7 |##################################
    714.9 |########################################
    753.1 |
    791.2 |
    829.4 |
    867.6 |
    905.8 |
    944.0 |
    982.1 |
   1020.3 |
   1058.5 |
   1096.7 |
   1134.8 |#
   1173.0 |
   1211.2 |
   1249.4 |
   1287.5 |
  (19 below, 14 above range)

```

## Diagnostics

- **minmax_branch**: CV=55.5% (high variance, measurements may be unstable)
- **minmax_branchless**: CV=53.1% (high variance, measurements may be unstable)
- **minmax_intrinsic**: CV=51.3% (high variance, measurements may be unstable)
