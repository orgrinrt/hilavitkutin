# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_per_iter is an outlier: 12.3x slower than the field

shr_per_iter (66 ns) is 12.3x the fastest (5 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_per_iter shows warm-up / thermal drift (autocorr +0.52)

shr_per_iter's per-pass series has lag-1 autocorrelation +0.52, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (5 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 12.3x the fastest

Fastest shr_const (5 ns) to slowest shr_per_iter (66 ns): 12.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_const is inconsistent: worst-20% is 2.2x its best-20%

shr_const's best 20% of batches run at 3 ns but its worst 20% at 7 ns (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 5.4 ns median
- 1 variant significantly slower than baseline
- Spread: 12.26x (fastest 5.4 ns, slowest 66.2 ns)

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
| shr_const | 2253ns | 2242ns | 2158ns | 2241ns | 2386ns | base |
| shr_loop_invariant | 2228ns | 2220ns | 2081ns | 2211ns | 2425ns | -1.12% |
| shr_per_iter | 2301ns | 2281ns | 2126ns | 2285ns | 2525ns | +2.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 5ns | 3ns | 7ns | base | 193.848 |
| shr_loop_invariant | 6ns | 4ns | 8ns | +15.58% | 167.714 |
| shr_per_iter | 66ns | 58ns | 73ns | +1143.45% | 15.590 |

## Performance model

- Peak throughput: **308.260 Gops/s** (shr_const; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 189.630 | 61.5% |
| shr_loop_invariant | 176.552 | 57.3% |
| shr_per_iter | 15.468 | 5.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2253ns | 2253ns | base |
| shr_loop_invariant | 2228ns | 2228ns | -1.12% |
| shr_per_iter | 2301ns | 2301ns | +2.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 5ns | base | --- | [5, 5] | --- | --- | --- | --- |
| shr_loop_invariant | 6ns | no significant difference | [+0, +1]ns | [6, 6] | no (adj: YES) | 0.0046 | 0.0046 | 15 |
| shr_per_iter | 66ns | +60.5ns (+1120.4%) | [+59, +62]ns | [65, 67] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 5ns | +42.6% | +1076.9% |
| 1 | 2 | 6ns | -3.6% | +1008.9% |
| 1 | 3 | 5ns | +15.4% | +1173.1% |
| 1 | 4 | 5ns | +40.2% | +1393.5% |
| 1 | 5 | 6ns | -6.9% | +1070.7% |
| 1 | 6 | 5ns | +20.8% | +1327.1% |
| 1 | 7 | 5ns | -3.8% | +1213.5% |
| 1 | 8 | 5ns | +7.4% | +1118.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | -0.475 | moderate- |
| shr_loop_invariant | -0.181 | ok |
| shr_per_iter | 0.521 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **shr_loop_invariant**: won 3/8, lost 5/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 3.4ns | 5.3ns | 64.0% | HIGH |
| shr_loop_invariant | 3.7ns | 6.1ns | 60.8% | HIGH |
| shr_per_iter | 3.2ns | 65.7ns | 4.9% |  |

## Distribution (algo ns)

```
shr_const (n=160, range 3.3-7.2 ns)
      3.3 |
      3.5 |###################
      3.7 |
      3.9 |
      4.1 |####################
      4.3 |
      4.5 |############################
      4.7 |
      4.9 |####################################
      5.1 |
      5.3 |########################################
      5.5 |
      5.7 |####################################
      5.9 |
      6.0 |############################
      6.2 |
      6.4 |
      6.6 |##########################
      6.8 |
      7.0 |###########
  (19 below, 11 above range)

shr_loop_invariant (n=160, range 4.0-8.4 ns)
      4.0 |################
      4.2 |
      4.5 |################
      4.7 |
      4.9 |##########################
      5.1 |
      5.3 |#################################
      5.6 |
      5.8 |########################################
      6.0 |########
      6.2 |
      6.4 |
      6.6 |####################
      6.9 |
      7.1 |################
      7.3 |##########################
      7.5 |
      7.7 |################
      8.0 |
      8.2 |######
  (11 below, 12 above range)

shr_per_iter (n=160, range 58.1-73.0 ns)
     58.1 |#################
     58.9 |##
     59.6 |#####
     60.4 |#################
     61.1 |############################
     61.8 |#########################
     62.6 |#################
     63.3 |##############
     64.1 |############################
     64.8 |##############
     65.6 |#########################
     66.3 |#########################
     67.1 |####################
     67.8 |########################################
     68.6 |##################################
     69.3 |#################
     70.0 |#####
     70.8 |#####
     71.5 |##############
     72.3 |#################
  (14 below, 14 above range)

```

## Diagnostics

- **shr_const**: CV=26.0% (high variance, measurements may be unstable)
- **shr_const**: bridge=61.1% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=26.1% (high variance, measurements may be unstable)
- **shr_loop_invariant**: bridge=63.8% of algo (FFI overhead may distort results)
- **shr_per_iter**: autocorrelation=0.52 (measurement drift or warm-up artifact)
