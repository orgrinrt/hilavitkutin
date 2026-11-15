# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_per_iter is an outlier: 4.9x slower than the field

shr_per_iter (94 ns) is 4.9x the fastest (19 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_const is fastest but the noisiest (CV 93.4%)

shr_const wins on median (19 ns) yet has the highest variance (CV 93.4%), while shr_per_iter is the steadiest (CV 83.5%, 94 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### shr_per_iter shows warm-up / thermal drift (autocorr +0.59)

shr_per_iter's per-pass series has lag-1 autocorrelation +0.59, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (19 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.9x the fastest

Fastest shr_const (19 ns) to slowest shr_per_iter (94 ns): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_loop_invariant is inconsistent: worst-20% is 3.6x its best-20%

shr_loop_invariant's best 20% of batches run at 17 ns but its worst 20% at 61 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 19.2 ns median
- 2 variants significantly slower than baseline
- Spread: 4.90x (fastest 19.2 ns, slowest 94.0 ns)

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
| shr_const | 2845ns | 1897ns | 1740ns | 2087ns | 6221ns | base |
| shr_loop_invariant | 2818ns | 1898ns | 1614ns | 2185ns | 5922ns | -0.93% |
| shr_per_iter | 2594ns | 1968ns | 1610ns | 1864ns | 5766ns | -8.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 29ns | 18ns | 62ns | base | 35.708 |
| shr_loop_invariant | 29ns | 17ns | 61ns | +2.24% | 34.927 |
| shr_per_iter | 123ns | 77ns | 272ns | +330.24% | 8.300 |

## Performance model

- Peak throughput: **60.805 Gops/s** (shr_loop_invariant; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 53.333 | 87.7% |
| shr_loop_invariant | 51.717 | 85.1% |
| shr_per_iter | 10.888 | 17.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2845ns | 2845ns | base |
| shr_loop_invariant | 2818ns | 2818ns | -0.93% |
| shr_per_iter | 2594ns | 2594ns | -8.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 19ns | base | --- | [19, 19] | --- | --- | --- | --- |
| shr_loop_invariant | 20ns | +0.6ns (+3.1%) | [+0, +1]ns | [20, 20] | YES | 0.0000 | 0.0000 | 0 |
| shr_per_iter | 94ns | +64.3ns (+334.9%) | [+61, +65]ns | [78, 94] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 19ns | -15.5% | +301.8% |
| 1 | 2 | 16ns | +26.1% | +392.4% |
| 1 | 3 | 19ns | +3.1% | +302.6% |
| 1 | 4 | 19ns | -15.6% | +301.3% |
| 1 | 5 | 19ns | +3.1% | +303.6% |
| 1 | 6 | 30ns | +5.0% | +212.5% |
| 1 | 7 | 19ns | +3.1% | +390.6% |
| 1 | 8 | 19ns | +3.1% | +391.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | -0.068 | ok |
| shr_loop_invariant | -0.034 | ok |
| shr_per_iter | 0.590 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **shr_loop_invariant**: won 2/8, lost 6/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 35.3ns | 28.7ns | 123.1% | HIGH |
| shr_loop_invariant | 35.6ns | 29.3ns | 121.6% | HIGH |
| shr_per_iter | 31.5ns | 123.4ns | 25.5% | HIGH |

## Distribution (algo ns)

```
shr_const (n=160, range 17.6-62.3 ns)
     17.6 |########################################
     19.8 |
     22.0 |
     24.3 |
     26.5 |
     28.7 |#######
     31.0 |
     33.2 |
     35.4 |
     37.7 |
     39.9 |
     42.2 |
     44.4 |
     46.6 |
     48.9 |
     51.1 |
     53.3 |
     55.6 |
     57.8 |##
     60.0 |
  (14 below, 17 above range)

shr_loop_invariant (n=160, range 16.8-61.1 ns)
     16.8 |
     19.1 |########################################
     21.3 |#
     23.5 |
     25.7 |
     27.9 |
     30.1 |#########
     32.3 |
     34.5 |
     36.8 |
     39.0 |#
     41.2 |
     43.4 |
     45.6 |#
     47.8 |
     50.0 |#
     52.3 |#
     54.5 |#
     56.7 |
     58.9 |##
  (26 below, 14 above range)

shr_per_iter (n=160, range 76.9-272.1 ns)
     76.9 |########################################
     86.7 |######################################
     96.5 |#
    106.2 |
    116.0 |
    125.7 |
    135.5 |
    145.2 |
    155.0 |
    164.8 |
    174.5 |
    184.3 |
    194.0 |
    203.8 |#
    213.5 |#
    223.3 |####
    233.0 |#
    242.8 |##
    252.6 |##
    262.3 |
  (17 below, 12 above range)

```

## Diagnostics

- **shr_const**: CV=62.6% (high variance, measurements may be unstable)
- **shr_const**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **shr_const**: bridge=122.9% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=59.2% (high variance, measurements may be unstable)
- **shr_loop_invariant**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **shr_loop_invariant**: bridge=121.7% of algo (FFI overhead may distort results)
- **shr_per_iter**: CV=63.7% (high variance, measurements may be unstable)
- **shr_per_iter**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **shr_per_iter**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **shr_per_iter**: bridge=25.2% of algo (FFI overhead may distort results)
