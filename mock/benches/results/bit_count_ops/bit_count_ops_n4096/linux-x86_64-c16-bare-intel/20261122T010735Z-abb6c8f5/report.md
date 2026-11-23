# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 3.7x slower than the field

ctz_loop (660 ns) is 3.7x the fastest (179 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ctz_debruijn, ctz_intrinsic) are a dead heat (<1%)

ctz_debruijn (179 ns) and ctz_intrinsic (179 ns) differ by 0.11%, inside the noise, even though the wider field spreads 268.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (179 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.7x the fastest

Fastest ctz_debruijn (179 ns) to slowest ctz_loop (660 ns): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ctz_debruijn is inconsistent: worst-20% is 3.7x its best-20%

ctz_debruijn's best 20% of batches run at 146 ns but its worst 20% at 539 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 179.1 ns median
- 1 variant significantly slower than baseline
- Spread: 3.68x (fastest 179.1 ns, slowest 659.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6f1038d492c5ab07 |
| Counter frequency | 2112774681 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ctz_debruijn | 2815ns | 2058ns | 1681ns | 2064ns | 6204ns | base |
| ctz_intrinsic | 2921ns | 2058ns | 1682ns | 2248ns | 6180ns | +3.75% |
| ctz_loop | 2953ns | 2550ns | 2096ns | 2554ns | 5007ns | +4.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 245ns | 146ns | 539ns | base | 16.726 |
| ctz_intrinsic | 255ns | 146ns | 539ns | +4.05% | 16.075 |
| ctz_loop | 766ns | 531ns | 1305ns | +212.85% | 5.346 |

## Performance model

- Peak throughput: **28.060 Gops/s** (ctz_debruijn; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 22.870 | 81.5% |
| ctz_intrinsic | 22.844 | 81.4% |
| ctz_loop | 6.209 | 22.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2815ns | 2815ns | base |
| ctz_intrinsic | 2921ns | 2921ns | +3.75% |
| ctz_loop | 2953ns | 2953ns | +4.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 179ns | base | --- | [178, 180] | --- | --- | --- | --- |
| ctz_intrinsic | 179ns | no significant difference | [-0, +1]ns | [178, 180] | no | 0.6300 | 0.6300 | 5 |
| ctz_loop | 660ns | +475.0ns (+265.2%) | [+465, +482]ns | [657, 665] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 147ns | +0.5% | +351.4% |
| 1 | 2 | 180ns | +0.2% | +284.8% |
| 1 | 3 | 148ns | +21.7% | +386.9% |
| 1 | 4 | 181ns | +139.5% | +258.1% |
| 1 | 5 | 179ns | -17.7% | +254.9% |
| 1 | 6 | 181ns | -0.6% | +264.6% |
| 1 | 7 | 148ns | +0.3% | +258.3% |
| 1 | 8 | 180ns | +0.1% | +189.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.471 | moderate- |
| ctz_intrinsic | -0.190 | ok |
| ctz_loop | 0.474 | moderate+ |

**Consistency summary:**

- **ctz_intrinsic**: won 2/8, lost 6/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 32.1ns | 244.9ns | 13.1% | HIGH |
| ctz_intrinsic | 33.3ns | 254.8ns | 13.1% | HIGH |
| ctz_loop | 27.8ns | 766.1ns | 3.6% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 146.0-538.7 ns)
    146.0 |##############
    165.6 |########################################
    185.2 |
    204.9 |
    224.5 |
    244.2 |
    263.8 |
    283.4 |
    303.1 |
    322.7 |
    342.3 |#
    362.0 |
    381.6 |#
    401.3 |
    420.9 |#
    440.5 |
    460.2 |#
    479.8 |#
    499.4 |##
    519.1 |
  (17 below, 18 above range)

ctz_intrinsic (n=160, range 146.2-539.2 ns)
    146.2 |#################
    165.8 |########################################
    185.5 |#
    205.1 |
    224.8 |
    244.4 |
    264.1 |
    283.7 |
    303.4 |#
    323.0 |
    342.7 |
    362.3 |
    382.0 |
    401.6 |
    421.3 |####
    440.9 |#
    460.6 |#
    480.2 |##
    499.9 |#
    519.5 |#
  (17 below, 13 above range)

ctz_loop (n=160, range 531.2-1304.6 ns)
    531.2 |
    569.8 |
    608.5 |#######################
    647.2 |########################################
    685.8 |#################################
    724.5 |
    763.2 |
    801.9 |
    840.5 |
    879.2 |
    917.9 |
    956.6 |
    995.2 |
   1033.9 |
   1072.6 |
   1111.2 |
   1149.9 |
   1188.6 |
   1227.3 |
   1265.9 |
  (30 below, 15 above range)

```

## Diagnostics

- **ctz_debruijn**: CV=62.3% (high variance, measurements may be unstable)
- **ctz_debruijn**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **ctz_debruijn**: bridge=13.1% of algo (FFI overhead may distort results)
- **ctz_intrinsic**: CV=61.2% (high variance, measurements may be unstable)
- **ctz_intrinsic**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **ctz_intrinsic**: bridge=13.1% of algo (FFI overhead may distort results)
- **ctz_loop**: CV=50.2% (high variance, measurements may be unstable)
