# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 4.0x slower than the field

ctz_loop (2.46 us) is 4.0x the fastest (616 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_intrinsic is fastest but the noisiest (CV 71.7%)

ctz_intrinsic wins on median (616 ns) yet has the highest variance (CV 71.7%), while ctz_loop is the steadiest (CV 11.7%, 2.46 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (ctz_intrinsic, ctz_debruijn) are a dead heat (<1%)

ctz_intrinsic (616 ns) and ctz_debruijn (620 ns) differ by 0.49%, inside the noise, even though the wider field spreads 299.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ctz_loop shows alternating (throttle bounce) (autocorr -0.70)

ctz_loop's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.0x the fastest

Fastest ctz_intrinsic (616 ns) to slowest ctz_loop (2.46 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ctz_intrinsic is inconsistent: worst-20% is 2.9x its best-20%

ctz_intrinsic's best 20% of batches run at 504 ns but its worst 20% at 1.46 us (2.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ctz_intrinsic** at 616.5 ns median (-0.5% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 3.99x (fastest 616.5 ns, slowest 2461.7 ns)

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
| ctz_debruijn | 3019ns | 2501ns | 2044ns | 2444ns | 5717ns | base |
| ctz_intrinsic | 2984ns | 2496ns | 2041ns | 2352ns | 5823ns | -1.14% |
| ctz_loop | 4140ns | 4361ns | 3544ns | 4176ns | 4631ns | +37.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 756ns | 507ns | 1430ns | base | 21.664 |
| ctz_intrinsic | 747ns | 504ns | 1457ns | -1.22% | 21.931 |
| ctz_loop | 2350ns | 1993ns | 2651ns | +210.71% | 6.972 |

## Performance model

- Peak throughput: **32.515 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 26.447 | 81.3% |
| ctz_intrinsic | 26.576 | 81.7% |
| ctz_loop | 6.656 | 20.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 3019ns | 3019ns | base |
| ctz_intrinsic | 2984ns | 2984ns | -1.14% |
| ctz_loop | 4140ns | 4140ns | +37.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 620ns | base | --- | [618, 622] | --- | --- | --- | --- |
| ctz_intrinsic | 616ns | no significant difference | [-6, +2]ns | [614, 620] | no | 0.4769 | 0.4769 | 0 |
| ctz_loop | 2462ns | +1794.2ns (+289.6%) | [+1453, +1856]ns | [2431, 2472] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 515ns | +25.0% | +379.2% |
| 1 | 2 | 619ns | +5.7% | +231.2% |
| 1 | 3 | 508ns | +0.5% | +412.8% |
| 1 | 4 | 648ns | -4.2% | +215.9% |
| 1 | 5 | 623ns | -17.3% | +304.9% |
| 1 | 6 | 515ns | -1.1% | +281.0% |
| 1 | 7 | 630ns | -0.2% | +285.9% |
| 1 | 8 | 643ns | -21.2% | +294.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.391 | moderate- |
| ctz_intrinsic | -0.265 | moderate- |
| ctz_loop | -0.701 | HIGH- (thermal bounce) |

**Consistency summary:**

- **ctz_intrinsic**: won 5/8, lost 3/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 29.2ns | 756.3ns | 3.9% |  |
| ctz_intrinsic | 28.8ns | 747.1ns | 3.9% |  |
| ctz_loop | 23.8ns | 2349.8ns | 1.0% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 506.6-1429.9 ns)
    506.6 |##################
    552.7 |
    598.9 |########################################
    645.1 |################
    691.2 |
    737.4 |#
    783.6 |
    829.7 |
    875.9 |
    922.0 |
    968.2 |
   1014.4 |
   1060.5 |
   1106.7 |
   1152.9 |
   1199.0 |
   1245.2 |
   1291.4 |
   1337.5 |
   1383.7 |
  (17 below, 20 above range)

ctz_intrinsic (n=160, range 503.9-1456.6 ns)
    503.9 |########################################
    551.5 |
    599.2 |###################################
    646.8 |#################
    694.4 |
    742.1 |
    789.7 |
    837.3 |
    885.0 |
    932.6 |
    980.2 |
   1027.9 |
   1075.5 |
   1123.1 |
   1170.8 |
   1218.4 |#
   1266.0 |
   1313.7 |
   1361.3 |
   1409.0 |
  (13 below, 18 above range)

ctz_loop (n=160, range 1993.5-2651.1 ns)
   1993.5 |
   2026.3 |########################################
   2059.2 |####
   2092.1 |
   2125.0 |
   2157.9 |
   2190.8 |
   2223.6 |
   2256.5 |
   2289.4 |
   2322.3 |
   2355.2 |
   2388.1 |
   2420.9 |####################
   2453.8 |######################
   2486.7 |##########
   2519.6 |#################################
   2552.5 |####
   2585.4 |####################
   2618.2 |##
  (19 below, 2 above range)

```

## Diagnostics

- **ctz_debruijn**: CV=55.1% (high variance, measurements may be unstable)
- **ctz_intrinsic**: CV=59.2% (high variance, measurements may be unstable)
