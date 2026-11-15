# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_const dominates: 56% faster than the next best (shr_loop_invariant)

shr_const (131 ns) leads shr_loop_invariant (203 ns) by 56%, a clear separation rather than a photo finish. CV 102.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### shr_per_iter is an outlier: 10.2x slower than the field

shr_per_iter (1.33 us) is 10.2x the fastest (131 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_const is fastest but the noisiest (CV 102.9%)

shr_const wins on median (131 ns) yet has the highest variance (CV 102.9%), while shr_per_iter is the steadiest (CV 42.9%, 1.33 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### shr_const shows alternating (throttle bounce) (autocorr -0.52)

shr_const's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (131 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 10.2x the fastest

Fastest shr_const (131 ns) to slowest shr_per_iter (1.33 us): 10.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_const is inconsistent: worst-20% is 3.6x its best-20%

shr_const's best 20% of batches run at 128 ns but its worst 20% at 459 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 130.6 ns median
- 2 variants significantly slower than baseline
- Spread: 10.23x (fastest 130.6 ns, slowest 1334.9 ns)

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
| Counter frequency | 2112849135 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| shr_const | 2627ns | 1667ns | 1661ns | 1904ns | 5762ns | base |
| shr_loop_invariant | 3020ns | 2089ns | 1711ns | 2338ns | 6377ns | +14.97% |
| shr_per_iter | 3627ns | 3213ns | 2866ns | 3216ns | 5621ns | +38.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 208ns | 128ns | 459ns | base | 78.654 |
| shr_loop_invariant | 294ns | 175ns | 601ns | +41.15% | 55.722 |
| shr_per_iter | 1503ns | 1189ns | 2318ns | +621.44% | 10.902 |

## Performance model

- Peak throughput: **127.648 Gops/s** (shr_const; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 125.500 | 98.3% |
| shr_loop_invariant | 80.551 | 63.1% |
| shr_per_iter | 12.274 | 9.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2627ns | 2627ns | base |
| shr_loop_invariant | 3020ns | 3020ns | +14.97% |
| shr_per_iter | 3627ns | 3627ns | +38.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 131ns | base | --- | [130, 132] | --- | --- | --- | --- |
| shr_loop_invariant | 203ns | +72.5ns (+55.6%) | [+70, +75]ns | [202, 218] | YES | 0.0000 | 0.0000 | 0 |
| shr_per_iter | 1335ns | +1204.5ns (+922.6%) | [+1202, +1205]ns | [1335, 1335] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 130ns | +40.8% | +925.0% |
| 1 | 2 | 130ns | +41.4% | +925.7% |
| 1 | 3 | 132ns | +63.0% | +1550.2% |
| 1 | 4 | 129ns | +72.4% | +746.5% |
| 1 | 5 | 135ns | +49.9% | +888.3% |
| 1 | 6 | 130ns | +41.3% | +925.7% |
| 1 | 7 | 130ns | +56.3% | +926.5% |
| 1 | 8 | 133ns | +216.8% | +903.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | -0.520 | HIGH- (thermal bounce) |
| shr_loop_invariant | 0.001 | ok |
| shr_per_iter | -0.353 | moderate- |

**Consistency summary:**

- **shr_loop_invariant**: won 0/8, lost 8/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 31.2ns | 208.3ns | 15.0% | HIGH |
| shr_loop_invariant | 35.5ns | 294.0ns | 12.1% | HIGH |
| shr_per_iter | 26.9ns | 1502.8ns | 1.8% |  |

## Distribution (algo ns)

```
shr_const (n=160, range 128.4-459.0 ns)
    128.4 |########################################
    144.9 |
    161.4 |
    177.9 |
    194.5 |
    211.0 |
    227.5 |
    244.1 |
    260.6 |
    277.1 |
    293.7 |
    310.2 |
    326.7 |
    343.3 |
    359.8 |
    376.3 |#
    392.9 |#
    409.4 |#
    425.9 |##
    442.4 |#
  (15 below, 14 above range)

shr_loop_invariant (n=160, range 174.7-601.1 ns)
    174.7 |#######################
    196.0 |########################################
    217.4 |#################
    238.7 |##
    260.0 |
    281.3 |
    302.6 |#
    324.0 |#
    345.3 |
    366.6 |
    387.9 |
    409.2 |#
    430.6 |#
    451.9 |#
    473.2 |##
    494.5 |#
    515.8 |##
    537.2 |##
    558.5 |###
    579.8 |#
  (16 below, 15 above range)

shr_per_iter (n=160, range 1189.3-2317.9 ns)
   1189.3 |
   1245.7 |
   1302.2 |########################################
   1358.6 |
   1415.0 |
   1471.5 |
   1527.9 |
   1584.3 |
   1640.8 |
   1697.2 |
   1753.6 |
   1810.1 |
   1866.5 |
   1922.9 |
   1979.3 |
   2035.8 |
   2092.2 |
   2148.6 |
   2205.1 |
   2261.5 |
  (19 below, 14 above range)

```

## Diagnostics

- **shr_const**: CV=64.5% (high variance, measurements may be unstable)
- **shr_const**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **shr_const**: bridge=15.2% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=56.7% (high variance, measurements may be unstable)
- **shr_loop_invariant**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **shr_loop_invariant**: bridge=12.1% of algo (FFI overhead may distort results)
- **shr_per_iter**: CV=38.1% (high variance, measurements may be unstable)
