# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 8.8x slower than the field

rot_variable_amount (48 ns) is 8.8x the fastest (5 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_intrinsic_const is fastest but the noisiest (CV 29.0%)

rot_intrinsic_const wins on median (5 ns) yet has the highest variance (CV 29.0%), while rot_variable_amount is the steadiest (CV 9.0%, 48 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### rot_intrinsic_const shows alternating (throttle bounce) (autocorr -0.69)

rot_intrinsic_const's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (rot_intrinsic_const)

The baseline rot_intrinsic_const is the fastest (5 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 8.8x the fastest

Fastest rot_intrinsic_const (5 ns) to slowest rot_variable_amount (48 ns): 8.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### rot_intrinsic_const is inconsistent: worst-20% is 2.2x its best-20%

rot_intrinsic_const's best 20% of batches run at 4 ns but its worst 20% at 8 ns (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (rot_intrinsic_const) is the fastest** at 5.4 ns median
- 1 variant significantly slower than baseline
- Spread: 8.80x (fastest 5.4 ns, slowest 47.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Rotate strategy: intrinsic const vs manual shifts vs variable amount |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x71f692ab4e08d325 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rot_intrinsic_const | 2143ns | 2135ns | 2130ns | 2135ns | 2179ns | base |
| rot_manual_shifts | 2141ns | 2136ns | 2130ns | 2136ns | 2166ns | -0.11% |
| rot_variable_amount | 2169ns | 2165ns | 2161ns | 2166ns | 2189ns | +1.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 6ns | 4ns | 8ns | base | 45.694 |
| rot_manual_shifts | 6ns | 4ns | 8ns | +4.36% | 43.784 |
| rot_variable_amount | 48ns | 42ns | 54ns | +754.17% | 5.349 |

## Performance model

- Peak throughput: **71.359 Gops/s** (rot_intrinsic_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 47.407 | 66.4% |
| rot_manual_shifts | 44.138 | 61.9% |
| rot_variable_amount | 5.389 | 7.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2143ns | 2143ns | base |
| rot_manual_shifts | 2141ns | 2141ns | -0.11% |
| rot_variable_amount | 2169ns | 2169ns | +1.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 5ns | base | --- | [5, 6] | --- | --- | --- | --- |
| rot_manual_shifts | 6ns | no significant difference | [+0, +1]ns | [5, 6] | no | 0.1621 | 0.1621 | 12 |
| rot_variable_amount | 48ns | +42.1ns (+779.6%) | [+41, +43]ns | [47, 48] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 5ns | +24.1% | +772.2% |
| 1 | 2 | 5ns | +7.4% | +787.0% |
| 1 | 3 | 6ns | -20.7% | +715.5% |
| 1 | 4 | 5ns | +12.0% | +854.0% |
| 1 | 5 | 6ns | +7.1% | +748.2% |
| 1 | 6 | 5ns | +0.0% | +768.5% |
| 1 | 7 | 5ns | +7.4% | +790.7% |
| 1 | 8 | 5ns | +14.8% | +763.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.692 | HIGH- (thermal bounce) |
| rot_manual_shifts | 0.022 | ok |
| rot_variable_amount | -0.639 | HIGH- (thermal bounce) |

**Consistency summary:**

- **rot_manual_shifts**: won 1/8, lost 6/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 11.8ns | 5.6ns | 211.3% | HIGH |
| rot_manual_shifts | 11.4ns | 5.8ns | 195.4% | HIGH |
| rot_variable_amount | 2.4ns | 47.9ns | 5.0% | HIGH |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 3.6-7.9 ns)
      3.6 |####################
      3.8 |
      4.0 |###################################
      4.2 |
      4.5 |###################################
      4.7 |
      4.9 |########################################
      5.1 |
      5.3 |#####################################
      5.5 |
      5.7 |#################################
      6.0 |
      6.2 |############################
      6.4 |
      6.6 |####################
      6.8 |
      7.0 |########################
      7.3 |
      7.5 |##########################
      7.7 |########
  (11 below, 9 above range)

rot_manual_shifts (n=160, range 3.9-8.0 ns)
      3.9 |
      4.1 |#####################
      4.3 |
      4.5 |##################################
      4.7 |
      4.9 |#########################
      5.1 |
      5.3 |#######################
      5.5 |
      5.7 |###########################
      5.9 |
      6.1 |########################################
      6.3 |
      6.5 |#####################
      6.7 |
      7.0 |##################
      7.2 |
      7.4 |##################
      7.6 |
      7.8 |##################
  (13 below, 10 above range)

rot_variable_amount (n=160, range 42.4-54.3 ns)
     42.4 |##############################
     43.0 |#########
     43.6 |#####################
     44.2 |##############################
     44.8 |#####################
     45.4 |########################################
     46.0 |##################
     46.6 |########################################
     47.2 |##################
     47.8 |####################################
     48.4 |#################################
     49.0 |#########
     49.6 |##############################
     50.2 |############
     50.8 |########################
     51.4 |
     52.0 |#####################
     52.6 |###
     53.2 |############
     53.8 |#########
  (8 below, 14 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=27.9% (high variance, measurements may be unstable)
- **rot_intrinsic_const**: bridge=216.7% of algo (FFI overhead may distort results)
- **rot_manual_shifts**: CV=25.3% (high variance, measurements may be unstable)
- **rot_manual_shifts**: bridge=193.1% of algo (FFI overhead may distort results)
