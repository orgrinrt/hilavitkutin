# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_const dominates: 46% faster than the next best (shr_loop_invariant)

shr_const (139 ns) leads shr_loop_invariant (203 ns) by 46%, a clear separation rather than a photo finish. CV 108.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### shr_per_iter is an outlier: 9.6x slower than the field

shr_per_iter (1.33 us) is 9.6x the fastest (139 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_const is fastest but the noisiest (CV 108.1%)

shr_const wins on median (139 ns) yet has the highest variance (CV 108.1%), while shr_per_iter is the steadiest (CV 34.7%, 1.33 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (139 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.6x the fastest

Fastest shr_const (139 ns) to slowest shr_per_iter (1.33 us): 9.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_const is inconsistent: worst-20% is 3.9x its best-20%

shr_const's best 20% of batches run at 128 ns but its worst 20% at 498 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 139.2 ns median
- 2 variants significantly slower than baseline
- Spread: 9.57x (fastest 139.2 ns, slowest 1332.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Shift amount: const vs loop-invariant runtime vs per-iter independent |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6a92bf08e314750d |
| Counter frequency | 2112797885 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| shr_const | 2690ns | 1687ns | 1662ns | 2033ns | 5685ns | base |
| shr_loop_invariant | 2726ns | 2083ns | 1712ns | 2083ns | 5671ns | +1.37% |
| shr_per_iter | 3402ns | 3210ns | 2859ns | 3212ns | 4519ns | +26.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 230ns | 128ns | 498ns | base | 71.314 |
| shr_loop_invariant | 271ns | 174ns | 565ns | +17.78% | 60.549 |
| shr_per_iter | 1412ns | 1185ns | 1876ns | +514.40% | 11.607 |

## Performance model

- Peak throughput: **128.360 Gops/s** (shr_const; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 117.701 | 91.7% |
| shr_loop_invariant | 80.869 | 63.0% |
| shr_per_iter | 12.296 | 9.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2690ns | 2690ns | base |
| shr_loop_invariant | 2726ns | 2726ns | +1.37% |
| shr_per_iter | 3402ns | 3402ns | +26.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 139ns | base | --- | [130, 181] | --- | --- | --- | --- |
| shr_loop_invariant | 203ns | +72.6ns (+52.2%) | [+53, +74]ns | [202, 203] | YES | 0.0000 | 0.0000 | 0 |
| shr_per_iter | 1332ns | +1201.5ns (+863.1%) | [+1163, +1203]ns | [1332, 1333] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 128ns | +57.8% | +938.0% |
| 1 | 2 | 183ns | +10.5% | +626.6% |
| 1 | 3 | 184ns | +10.3% | +626.0% |
| 1 | 4 | 129ns | +57.5% | +936.1% |
| 1 | 5 | 128ns | +57.9% | +938.3% |
| 1 | 6 | 131ns | +40.1% | +917.9% |
| 1 | 7 | 130ns | +56.1% | +926.7% |
| 1 | 8 | 183ns | -9.1% | +497.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | 0.051 | ok |
| shr_loop_invariant | -0.138 | ok |
| shr_per_iter | -0.018 | ok |

**Consistency summary:**

- **shr_loop_invariant**: won 1/8, lost 7/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 32.1ns | 229.7ns | 14.0% | HIGH |
| shr_loop_invariant | 32.1ns | 270.6ns | 11.9% | HIGH |
| shr_per_iter | 25.3ns | 1411.6ns | 1.8% |  |

## Distribution (algo ns)

```
shr_const (n=160, range 127.6-497.7 ns)
    127.6 |########################################
    146.1 |#
    164.7 |#######
    183.2 |########
    201.7 |
    220.2 |
    238.7 |
    257.2 |
    275.7 |
    294.2 |
    312.7 |
    331.2 |
    349.7 |#
    368.2 |##
    386.7 |####
    405.2 |#
    423.7 |#
    442.2 |#
    460.7 |##
    479.2 |
  (18 below, 14 above range)

shr_loop_invariant (n=160, range 174.2-564.7 ns)
    174.2 |########
    193.7 |########################################
    213.2 |######
    232.8 |
    252.3 |
    271.8 |
    291.3 |
    310.9 |
    330.4 |
    349.9 |
    369.4 |
    389.0 |
    408.5 |
    428.0 |
    447.5 |
    467.1 |
    486.6 |#
    506.1 |
    525.6 |##
    545.2 |#
  (16 below, 17 above range)

shr_per_iter (n=160, range 1184.8-1875.7 ns)
   1184.8 |
   1219.3 |
   1253.9 |
   1288.4 |
   1323.0 |########################################
   1357.5 |
   1392.1 |
   1426.6 |
   1461.2 |
   1495.7 |
   1530.2 |
   1564.8 |
   1599.3 |
   1633.9 |
   1668.4 |
   1703.0 |
   1737.5 |
   1772.1 |
   1806.6 |
   1841.2 |
  (19 below, 9 above range)

```

## Diagnostics

- **shr_const**: CV=65.5% (high variance, measurements may be unstable)
- **shr_const**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **shr_const**: bridge=14.4% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=59.4% (high variance, measurements may be unstable)
- **shr_loop_invariant**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **shr_loop_invariant**: bridge=12.2% of algo (FFI overhead may distort results)
- **shr_per_iter**: CV=32.8% (high variance, measurements may be unstable)
