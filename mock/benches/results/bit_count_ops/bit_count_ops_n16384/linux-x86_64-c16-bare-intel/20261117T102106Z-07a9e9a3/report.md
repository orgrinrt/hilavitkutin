# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_intrinsic dominates: 18% faster than the next best (ctz_debruijn)

ctz_intrinsic (524 ns) leads ctz_debruijn (616 ns) by 18%, a clear separation rather than a photo finish. CV 77.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ctz_loop is an outlier: 4.8x slower than the field

ctz_loop (2.51 us) is 4.8x the fastest (524 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_intrinsic is fastest but the noisiest (CV 77.4%)

ctz_intrinsic wins on median (524 ns) yet has the highest variance (CV 77.4%), while ctz_loop is the steadiest (CV 7.1%, 2.51 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ctz_loop shows alternating (throttle bounce) (autocorr -0.52)

ctz_loop's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.8x the fastest

Fastest ctz_intrinsic (524 ns) to slowest ctz_loop (2.51 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ctz_debruijn is inconsistent: worst-20% is 2.7x its best-20%

ctz_debruijn's best 20% of batches run at 504 ns but its worst 20% at 1.35 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### ctz_intrinsic's edge over baseline is significant but tiny (-4 ns, 0.65%)

ctz_intrinsic differs from baseline ctz_debruijn by -4 ns (0.65%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ctz_intrinsic** at 523.6 ns median (-15.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.79x (fastest 523.6 ns, slowest 2505.9 ns)

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
| Counter frequency | 2112755836 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ctz_debruijn | 2896ns | 2503ns | 2047ns | 2356ns | 5368ns | base |
| ctz_intrinsic | 2819ns | 2091ns | 2047ns | 2270ns | 5236ns | -2.67% |
| ctz_loop | 4296ns | 4404ns | 3820ns | 4383ns | 4511ns | +48.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 722ns | 504ns | 1348ns | base | 22.692 |
| ctz_intrinsic | 703ns | 504ns | 1321ns | -2.68% | 23.316 |
| ctz_loop | 2433ns | 2132ns | 2597ns | +236.95% | 6.734 |

## Performance model

- Peak throughput: **32.505 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 26.587 | 81.8% |
| ctz_intrinsic | 31.291 | 96.3% |
| ctz_loop | 6.538 | 20.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2896ns | 2896ns | base |
| ctz_intrinsic | 2819ns | 2819ns | -2.67% |
| ctz_loop | 4296ns | 4296ns | +48.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 616ns | base | --- | [613, 618] | --- | --- | --- | --- |
| ctz_intrinsic | 524ns | -4.0ns (-0.6%) | [-29, -0]ns | [512, 616] | YES | 0.0478 | 0.0478 | 0 |
| ctz_loop | 2506ns | +1903.1ns (+308.8%) | [+1886, +1926]ns | [2473, 2525] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 515ns | -0.7% | +381.2% |
| 1 | 2 | 507ns | +23.9% | +357.0% |
| 1 | 3 | 622ns | -18.3% | +319.0% |
| 1 | 4 | 625ns | -18.7% | +224.6% |
| 1 | 5 | 509ns | +22.8% | +397.4% |
| 1 | 6 | 619ns | +0.7% | +314.9% |
| 1 | 7 | 525ns | -3.4% | +358.0% |
| 1 | 8 | 621ns | -17.8% | +307.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.333 | moderate- |
| ctz_intrinsic | -0.154 | ok |
| ctz_loop | -0.520 | HIGH- (thermal bounce) |

**Consistency summary:**

- **ctz_intrinsic**: won 5/8, lost 3/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 28.0ns | 722.0ns | 3.9% |  |
| ctz_intrinsic | 27.4ns | 702.7ns | 3.9% |  |
| ctz_loop | 24.6ns | 2432.9ns | 1.0% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 504.4-1348.2 ns)
    504.4 |##################################
    546.6 |
    588.8 |########################################
    631.0 |#######
    673.2 |
    715.4 |
    757.6 |
    799.7 |
    841.9 |
    884.1 |
    926.3 |
    968.5 |
   1010.7 |
   1052.9 |
   1095.1 |
   1137.2 |
   1179.4 |
   1221.6 |
   1263.8 |
   1306.0 |
  (13 below, 19 above range)

ctz_intrinsic (n=160, range 504.0-1321.4 ns)
    504.0 |########################################
    544.9 |#
    585.8 |###################
    626.7 |###########
    667.5 |
    708.4 |#
    749.3 |
    790.1 |
    831.0 |
    871.9 |
    912.7 |
    953.6 |
    994.5 |
   1035.3 |
   1076.2 |
   1117.1 |
   1157.9 |
   1198.8 |
   1239.7 |
   1280.6 |#
  (14 below, 16 above range)

ctz_loop (n=160, range 2131.7-2597.5 ns)
   2131.7 |
   2155.0 |
   2178.3 |
   2201.6 |
   2224.8 |
   2248.1 |
   2271.4 |
   2294.7 |####################
   2318.0 |##########
   2341.3 |###
   2364.6 |
   2387.9 |###########################
   2411.2 |######
   2434.5 |
   2457.7 |##########################
   2481.0 |########
   2504.3 |###########################
   2527.6 |########################################
   2550.9 |########################
   2574.2 |##########
  (20 below, 21 above range)

```

## Diagnostics

- **ctz_debruijn**: CV=57.3% (high variance, measurements may be unstable)
- **ctz_intrinsic**: CV=57.7% (high variance, measurements may be unstable)
