# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_runtime is fastest but the noisiest (CV 90.7%)

dispatch_runtime wins on median (40 ns) yet has the highest variance (CV 90.7%), while dispatch_volatile is the steadiest (CV 89.1%, 47 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6 ns) is smaller than the fastest variant's own run-to-run std-dev (37 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### dispatch_direct is inconsistent: worst-20% is 3.8x its best-20%

dispatch_direct's best 20% of batches run at 35 ns but its worst 20% at 133 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: dispatch_runtime** at 40.5 ns median (-6.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.16x (fastest 40.5 ns, slowest 47.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x40ffee15d34db33f |
| Counter frequency | 2112860326 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 2768ns | 1921ns | 1571ns | 2132ns | 5871ns | base |
| dispatch_runtime | 2794ns | 1919ns | 1604ns | 2074ns | 6142ns | +0.93% |
| dispatch_volatile | 2682ns | 1924ns | 1574ns | 1985ns | 5880ns | -3.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 62ns | 35ns | 133ns | base | 4.107 |
| dispatch_runtime | 59ns | 34ns | 129ns | -5.50% | 4.346 |
| dispatch_volatile | 65ns | 38ns | 143ns | +4.95% | 3.913 |

## Performance model

- Peak throughput: **7.466 Gops/s** (dispatch_runtime; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 5.940 | 79.6% |
| dispatch_runtime | 6.321 | 84.7% |
| dispatch_volatile | 5.447 | 73.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2768ns | 2768ns | base |
| dispatch_runtime | 2794ns | 2794ns | +0.93% |
| dispatch_volatile | 2682ns | 2682ns | -3.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 43ns | base | --- | [43, 43] | --- | --- | --- | --- |
| dispatch_runtime | 40ns | -2.7ns (-6.1%) | [-3, -2]ns | [40, 41] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 47ns | +3.5ns (+8.2%) | [+3, +4]ns | [47, 47] | YES | 0.0325 | 0.0325 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 35ns | +15.0% | +33.3% |
| 1 | 2 | 35ns | +14.2% | +34.0% |
| 1 | 3 | 43ns | -17.3% | +9.9% |
| 1 | 4 | 35ns | -8.2% | +9.1% |
| 1 | 5 | 43ns | -2.1% | -10.4% |
| 1 | 6 | 43ns | -4.2% | -10.1% |
| 1 | 7 | 43ns | -11.2% | -10.2% |
| 1 | 8 | 43ns | -6.3% | +10.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | 0.061 | ok |
| dispatch_runtime | 0.040 | ok |
| dispatch_volatile | 0.363 | moderate+ |

**Consistency summary:**

- **dispatch_runtime**: won 6/8, lost 2/8
- **dispatch_volatile**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 33.8ns | 62.3ns | 54.2% | HIGH |
| dispatch_runtime | 34.9ns | 58.9ns | 59.3% | HIGH |
| dispatch_volatile | 35.0ns | 65.4ns | 53.5% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 35.2-133.2 ns)
     35.2 |##########
     40.1 |########################################
     45.0 |#
     49.9 |
     54.8 |
     59.7 |
     64.6 |
     69.5 |
     74.4 |
     79.3 |
     84.2 |
     89.1 |
     94.0 |
     98.9 |#
    103.8 |#
    108.7 |#
    113.6 |#
    118.5 |###
    123.4 |#
    128.3 |###
  (21 below, 13 above range)

dispatch_runtime (n=160, range 34.3-129.0 ns)
     34.3 |###############
     39.0 |########################################
     43.8 |
     48.5 |
     53.2 |
     58.0 |
     62.7 |
     67.4 |
     72.2 |
     76.9 |#
     81.6 |
     86.4 |
     91.1 |
     95.8 |
    100.6 |
    105.3 |
    110.0 |
    114.8 |##
    119.5 |###
    124.2 |##
  (15 below, 15 above range)

dispatch_volatile (n=160, range 38.3-142.5 ns)
     38.3 |########################
     43.5 |########################################
     48.7 |
     54.0 |
     59.2 |
     64.4 |
     69.6 |
     74.8 |
     80.0 |
     85.2 |
     90.4 |
     95.6 |
    100.8 |
    106.1 |#
    111.3 |###
    116.5 |##
    121.7 |##
    126.9 |##
    132.1 |
    137.3 |
  (17 below, 15 above range)

```

## Diagnostics

- **dispatch_direct**: CV=62.1% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dispatch_direct**: bridge=54.8% of algo (FFI overhead may distort results)
- **dispatch_runtime**: CV=62.4% (high variance, measurements may be unstable)
- **dispatch_runtime**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dispatch_runtime**: bridge=59.5% of algo (FFI overhead may distort results)
- **dispatch_volatile**: CV=64.0% (high variance, measurements may be unstable)
- **dispatch_volatile**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **dispatch_volatile**: bridge=52.6% of algo (FFI overhead may distort results)
