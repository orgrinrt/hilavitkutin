# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_per_iter is an outlier: 12.5x slower than the field

shr_per_iter (62 ns) is 12.5x the fastest (5 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (5 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 12.5x the fastest

Fastest shr_const (5 ns) to slowest shr_per_iter (62 ns): 12.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_loop_invariant is inconsistent: worst-20% is 2.2x its best-20%

shr_loop_invariant's best 20% of batches run at 4 ns but its worst 20% at 8 ns (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 5.0 ns median
- 2 variants significantly slower than baseline
- Spread: 12.46x (fastest 5.0 ns, slowest 62.3 ns)

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
| shr_const | 2151ns | 2106ns | 2095ns | 2137ns | 2250ns | base |
| shr_loop_invariant | 2143ns | 2103ns | 2096ns | 2129ns | 2236ns | -0.37% |
| shr_per_iter | 2173ns | 2138ns | 2130ns | 2153ns | 2276ns | +1.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 5ns | 3ns | 7ns | base | 197.612 |
| shr_loop_invariant | 6ns | 4ns | 8ns | +9.53% | 180.421 |
| shr_per_iter | 62ns | 56ns | 68ns | +1100.13% | 16.466 |

## Performance model

- Peak throughput: **302.567 Gops/s** (shr_const; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 204.800 | 67.7% |
| shr_loop_invariant | 189.630 | 62.7% |
| shr_per_iter | 16.437 | 5.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2151ns | 2151ns | base |
| shr_loop_invariant | 2143ns | 2143ns | -0.37% |
| shr_per_iter | 2173ns | 2173ns | +1.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 5ns | base | --- | [5, 5] | --- | --- | --- | --- |
| shr_loop_invariant | 5ns | +0.5ns (+10.0%) | [+0, +1]ns | [5, 6] | YES | 0.0024 | 0.0024 | 10 |
| shr_per_iter | 62ns | +56.9ns (+1138.0%) | [+56, +58]ns | [62, 63] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 5ns | -3.8% | +1098.1% |
| 1 | 2 | 6ns | -3.6% | +1060.7% |
| 1 | 3 | 5ns | +16.0% | +1146.0% |
| 1 | 4 | 6ns | -6.9% | +970.7% |
| 1 | 5 | 5ns | +45.7% | +1250.0% |
| 1 | 6 | 5ns | +8.0% | +1142.0% |
| 1 | 7 | 5ns | +3.8% | +1081.7% |
| 1 | 8 | 5ns | +11.5% | +1101.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | -0.458 | moderate- |
| shr_loop_invariant | -0.225 | moderate- |
| shr_per_iter | -0.023 | ok |

**Consistency summary:**

- **shr_loop_invariant**: won 3/8, lost 5/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 3.0ns | 5.2ns | 58.2% | HIGH |
| shr_loop_invariant | 3.5ns | 5.7ns | 61.3% | HIGH |
| shr_per_iter | 3.1ns | 62.2ns | 4.9% |  |

## Distribution (algo ns)

```
shr_const (n=160, range 3.4-7.1 ns)
      3.4 |
      3.6 |###########################
      3.8 |
      3.9 |
      4.1 |#########################
      4.3 |
      4.5 |######################################
      4.7 |
      4.9 |##############################
      5.0 |
      5.2 |########################################
      5.4 |
      5.6 |
      5.8 |################################
      6.0 |
      6.2 |#####################
      6.3 |
      6.5 |####################
      6.7 |
      6.9 |
  (14 below, 16 above range)

shr_loop_invariant (n=160, range 3.6-7.9 ns)
      3.6 |#########
      3.8 |
      4.1 |###########
      4.3 |
      4.5 |########################
      4.7 |
      4.9 |###################################
      5.1 |
      5.4 |########################################
      5.6 |
      5.8 |######################
      6.0 |##############
      6.2 |
      6.4 |
      6.6 |########################
      6.9 |
      7.1 |##############
      7.3 |#################
      7.5 |
      7.7 |########
  (13 below, 9 above range)

shr_per_iter (n=160, range 56.0-67.6 ns)
     56.0 |########
     56.6 |#############
     57.2 |####
     57.7 |#############
     58.3 |####
     58.9 |####
     59.5 |###############
     60.1 |###############
     60.6 |##########################
     61.2 |#############
     61.8 |############################
     62.4 |########################################
     63.0 |######
     63.6 |#############
     64.1 |#############
     64.7 |###############
     65.3 |######################
     65.9 |#############
     66.5 |###########
     67.0 |###########
  (13 below, 14 above range)

```

## Diagnostics

- **shr_const**: CV=25.4% (high variance, measurements may be unstable)
- **shr_const**: bridge=58.0% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=27.3% (high variance, measurements may be unstable)
- **shr_loop_invariant**: bridge=61.1% of algo (FFI overhead may distort results)
