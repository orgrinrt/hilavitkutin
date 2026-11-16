# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_const dominates: 20% faster than the next best (shr_loop_invariant)

shr_const (5 ns) leads shr_loop_invariant (6 ns) by 20%, a clear separation rather than a photo finish. CV 26.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### shr_per_iter is an outlier: 12.6x slower than the field

shr_per_iter (63 ns) is 12.6x the fastest (5 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_const is fastest but the noisiest (CV 26.2%)

shr_const wins on median (5 ns) yet has the highest variance (CV 26.2%), while shr_per_iter is the steadiest (CV 6.6%, 63 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (5 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 12.6x the fastest

Fastest shr_const (5 ns) to slowest shr_per_iter (63 ns): 12.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_const is inconsistent: worst-20% is 2.1x its best-20%

shr_const's best 20% of batches run at 3 ns but its worst 20% at 7 ns (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 5.0 ns median
- 2 variants significantly slower than baseline
- Spread: 12.58x (fastest 5.0 ns, slowest 62.9 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| shr_const | 2171ns | 2146ns | 2136ns | 2150ns | 2268ns | base |
| shr_loop_invariant | 2152ns | 2143ns | 2135ns | 2144ns | 2193ns | -0.87% |
| shr_per_iter | 2188ns | 2177ns | 2172ns | 2179ns | 2234ns | +0.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 5ns | 3ns | 7ns | base | 203.832 |
| shr_loop_invariant | 6ns | 4ns | 8ns | +18.09% | 172.609 |
| shr_per_iter | 63ns | 56ns | 68ns | +1145.71% | 16.363 |

## Performance model

- Peak throughput: **312.672 Gops/s** (shr_const; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 204.800 | 65.5% |
| shr_loop_invariant | 170.667 | 54.6% |
| shr_per_iter | 16.280 | 5.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2171ns | 2171ns | base |
| shr_loop_invariant | 2152ns | 2152ns | -0.87% |
| shr_per_iter | 2188ns | 2188ns | +0.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 5ns | base | --- | [5, 5] | --- | --- | --- | --- |
| shr_loop_invariant | 6ns | +0.9ns (+18.0%) | [+0, +1]ns | [6, 6] | YES | 0.0000 | 0.0000 | 14 |
| shr_per_iter | 63ns | +57.9ns (+1158.0%) | [+57, +58]ns | [62, 63] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 5ns | +3.8% | +1134.6% |
| 1 | 2 | 5ns | +20.0% | +1150.0% |
| 1 | 3 | 5ns | +11.1% | +1057.4% |
| 1 | 4 | 5ns | +29.2% | +1214.6% |
| 1 | 5 | 5ns | +16.0% | +1150.0% |
| 1 | 6 | 5ns | +19.2% | +1101.9% |
| 1 | 7 | 5ns | +34.4% | +1197.9% |
| 1 | 8 | 5ns | +16.0% | +1129.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | -0.442 | moderate- |
| shr_loop_invariant | -0.095 | ok |
| shr_per_iter | 0.033 | ok |

**Consistency summary:**

- **shr_loop_invariant**: won 0/8, lost 8/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 3.1ns | 5.0ns | 61.5% | HIGH |
| shr_loop_invariant | 3.4ns | 5.9ns | 57.1% | HIGH |
| shr_per_iter | 3.1ns | 62.6ns | 5.0% |  |

## Distribution (algo ns)

```
shr_const (n=160, range 3.3-6.9 ns)
      3.3 |#########################
      3.5 |
      3.6 |####################
      3.8 |
      4.0 |
      4.2 |########################################
      4.4 |
      4.5 |#############################
      4.7 |
      4.9 |##################################
      5.1 |
      5.3 |#############################
      5.5 |
      5.6 |################################
      5.8 |
      6.0 |
      6.2 |#######################
      6.4 |
      6.5 |##################
      6.7 |
  (9 below, 12 above range)

shr_loop_invariant (n=160, range 3.9-7.9 ns)
      3.9 |
      4.1 |#############
      4.3 |
      4.5 |#####################
      4.7 |
      4.9 |####################
      5.1 |
      5.3 |##################
      5.5 |
      5.7 |###################################
      5.9 |
      6.1 |########################################
      6.3 |
      6.5 |##############################
      6.7 |
      6.9 |##################
      7.1 |
      7.3 |###############
      7.5 |
      7.7 |##########
  (15 below, 12 above range)

shr_per_iter (n=160, range 56.4-68.2 ns)
     56.4 |
     57.0 |#######
     57.6 |#######
     58.2 |###########################
     58.8 |##
     59.4 |##########
     60.0 |######################
     60.6 |#################
     61.2 |###################################
     61.7 |############
     62.3 |########################################
     62.9 |#################
     63.5 |####################
     64.1 |#################
     64.7 |#################
     65.3 |######################
     65.9 |############
     66.5 |#######
     67.0 |############
     67.6 |##########
  (15 below, 17 above range)

```

## Diagnostics

- **shr_const**: CV=26.1% (high variance, measurements may be unstable)
- **shr_const**: bridge=58.0% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=24.4% (high variance, measurements may be unstable)
- **shr_loop_invariant**: bridge=55.0% of algo (FFI overhead may distort results)
