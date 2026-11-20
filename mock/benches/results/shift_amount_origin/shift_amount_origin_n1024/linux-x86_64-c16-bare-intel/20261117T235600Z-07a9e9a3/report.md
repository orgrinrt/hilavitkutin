# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_per_iter is an outlier: 4.9x slower than the field

shr_per_iter (95 ns) is 4.9x the fastest (19 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (19 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.9x the fastest

Fastest shr_const (19 ns) to slowest shr_per_iter (95 ns): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_loop_invariant is inconsistent: worst-20% is 3.7x its best-20%

shr_loop_invariant's best 20% of batches run at 16 ns but its worst 20% at 60 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 19.2 ns median
- 2 variants significantly slower than baseline
- Spread: 4.93x (fastest 19.2 ns, slowest 94.6 ns)

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
| Counter frequency | 2112788246 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| shr_const | 2764ns | 1905ns | 1554ns | 2186ns | 5705ns | base |
| shr_loop_invariant | 2575ns | 1905ns | 1557ns | 1872ns | 5704ns | -6.82% |
| shr_per_iter | 2866ns | 1979ns | 1797ns | 2243ns | 5803ns | +3.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 28ns | 16ns | 57ns | base | 36.974 |
| shr_loop_invariant | 27ns | 16ns | 60ns | -2.93% | 38.091 |
| shr_per_iter | 136ns | 86ns | 274ns | +391.40% | 7.524 |

## Performance model

- Peak throughput: **65.301 Gops/s** (shr_const; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 53.333 | 81.7% |
| shr_loop_invariant | 51.980 | 79.6% |
| shr_per_iter | 10.825 | 16.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2764ns | 2764ns | base |
| shr_loop_invariant | 2575ns | 2575ns | -6.82% |
| shr_per_iter | 2866ns | 2866ns | +3.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 19ns | base | --- | [19, 19] | --- | --- | --- | --- |
| shr_loop_invariant | 20ns | +0.5ns (+2.6%) | [+0, +1]ns | [20, 20] | YES | 0.0000 | 0.0000 | 0 |
| shr_per_iter | 95ns | +75.7ns (+394.0%) | [+75, +79]ns | [95, 95] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 16ns | +26.1% | +1166.9% |
| 1 | 2 | 16ns | +25.4% | +390.2% |
| 1 | 3 | 19ns | -16.1% | +390.2% |
| 1 | 4 | 19ns | +2.6% | +392.4% |
| 1 | 5 | 16ns | +2.5% | +499.4% |
| 1 | 6 | 16ns | +25.0% | +499.7% |
| 1 | 7 | 19ns | +2.6% | +391.7% |
| 1 | 8 | 19ns | -16.1% | +391.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | 0.133 | ok |
| shr_loop_invariant | -0.463 | moderate- |
| shr_per_iter | -0.170 | ok |

**Consistency summary:**

- **shr_loop_invariant**: won 2/8, lost 6/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 34.2ns | 27.7ns | 123.4% | HIGH |
| shr_loop_invariant | 32.4ns | 26.9ns | 120.5% | HIGH |
| shr_per_iter | 34.4ns | 136.1ns | 25.3% | HIGH |

## Distribution (algo ns)

```
shr_const (n=160, range 15.7-56.7 ns)
     15.7 |################################
     17.7 |########################################
     19.8 |
     21.8 |
     23.9 |
     25.9 |
     28.0 |#
     30.1 |
     32.1 |
     34.2 |
     36.2 |
     38.3 |
     40.3 |#
     42.4 |
     44.4 |####
     46.5 |
     48.5 |###
     50.6 |##
     52.6 |######
     54.7 |##
  (6 below, 14 above range)

shr_loop_invariant (n=160, range 16.1-60.2 ns)
     16.1 |##############
     18.3 |########################################
     20.5 |
     22.8 |
     25.0 |
     27.2 |
     29.4 |
     31.6 |
     33.8 |
     36.0 |
     38.2 |
     40.4 |#
     42.6 |
     44.8 |
     47.0 |
     49.2 |
     51.4 |
     53.6 |#
     55.8 |#
     58.0 |#
  (15 below, 13 above range)

shr_per_iter (n=160, range 85.6-273.7 ns)
     85.6 |########################################
     95.0 |#
    104.4 |
    113.8 |
    123.2 |
    132.6 |
    142.0 |
    151.4 |
    160.9 |
    170.3 |#
    179.7 |#
    189.1 |#
    198.5 |###
    207.9 |
    217.3 |
    226.7 |
    236.1 |
    245.5 |
    254.9 |
    264.3 |
  (16 below, 18 above range)

```

## Diagnostics

- **shr_const**: CV=59.7% (high variance, measurements may be unstable)
- **shr_const**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **shr_const**: bridge=122.9% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=67.1% (high variance, measurements may be unstable)
- **shr_loop_invariant**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **shr_loop_invariant**: bridge=121.8% of algo (FFI overhead may distort results)
- **shr_per_iter**: CV=56.3% (high variance, measurements may be unstable)
- **shr_per_iter**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **shr_per_iter**: bridge=25.3% of algo (FFI overhead may distort results)
