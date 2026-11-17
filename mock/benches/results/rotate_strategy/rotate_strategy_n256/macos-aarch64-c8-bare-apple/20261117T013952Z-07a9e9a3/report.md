# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 8.2x slower than the field

rot_variable_amount (47 ns) is 8.2x the fastest (6 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_intrinsic_const is fastest but the noisiest (CV 34.7%)

rot_intrinsic_const wins on median (6 ns) yet has the highest variance (CV 34.7%), while rot_variable_amount is the steadiest (CV 9.7%, 47 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (rot_intrinsic_const, rot_manual_shifts) are a dead heat (<1%)

rot_intrinsic_const (6 ns) and rot_manual_shifts (6 ns) differ by 0.00%, inside the noise, even though the wider field spreads 715.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (rot_intrinsic_const)

The baseline rot_intrinsic_const is the fastest (6 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 8.2x the fastest

Fastest rot_intrinsic_const (6 ns) to slowest rot_variable_amount (47 ns): 8.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### rot_intrinsic_const is inconsistent: worst-20% is 2.6x its best-20%

rot_intrinsic_const's best 20% of batches run at 4 ns but its worst 20% at 9 ns (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (rot_intrinsic_const) is the fastest** at 5.8 ns median
- 1 variant significantly slower than baseline
- Spread: 8.16x (fastest 5.8 ns, slowest 47.3 ns)

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
| rot_intrinsic_const | 2310ns | 2102ns | 2093ns | 2124ns | 3083ns | base |
| rot_manual_shifts | 2266ns | 2169ns | 2094ns | 2174ns | 2717ns | -1.88% |
| rot_variable_amount | 2176ns | 2158ns | 2126ns | 2160ns | 2272ns | -5.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 6ns | 4ns | 9ns | base | 41.911 |
| rot_manual_shifts | 6ns | 4ns | 9ns | -3.57% | 43.463 |
| rot_variable_amount | 48ns | 42ns | 55ns | +687.81% | 5.320 |

## Performance model

- Peak throughput: **72.304 Gops/s** (rot_intrinsic_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 44.138 | 61.0% |
| rot_manual_shifts | 44.138 | 61.0% |
| rot_variable_amount | 5.412 | 7.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2310ns | 2310ns | base |
| rot_manual_shifts | 2266ns | 2266ns | -1.88% |
| rot_variable_amount | 2176ns | 2176ns | -5.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 6ns | base | --- | [5, 6] | --- | --- | --- | --- |
| rot_manual_shifts | 6ns | no significant difference | [-0, +0]ns | [5, 6] | no | 0.3627 | 0.3627 | 14 |
| rot_variable_amount | 47ns | +42.1ns (+725.9%) | [+41, +43]ns | [47, 49] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 6ns | -6.5% | +635.5% |
| 1 | 2 | 6ns | -19.4% | +632.3% |
| 1 | 3 | 6ns | +0.0% | +737.5% |
| 1 | 4 | 5ns | -3.7% | +783.3% |
| 1 | 5 | 9ns | -28.3% | +425.6% |
| 1 | 6 | 5ns | +7.4% | +822.2% |
| 1 | 7 | 6ns | -3.6% | +758.9% |
| 1 | 8 | 5ns | +3.7% | +772.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.293 | moderate- |
| rot_manual_shifts | -0.242 | moderate- |
| rot_variable_amount | 0.419 | moderate+ |

**Consistency summary:**

- **rot_manual_shifts**: won 5/8, lost 2/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 11.3ns | 6.1ns | 185.1% | HIGH |
| rot_manual_shifts | 12.2ns | 5.9ns | 207.4% | HIGH |
| rot_variable_amount | 2.4ns | 48.1ns | 5.0% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 3.5-9.0 ns)
      3.5 |############################
      3.8 |
      4.1 |##############
      4.4 |###################################
      4.6 |
      4.9 |#####################
      5.2 |########################################
      5.5 |
      5.7 |############################
      6.0 |############################
      6.3 |
      6.6 |############################
      6.8 |########################################
      7.1 |
      7.4 |##################
      7.7 |################
      7.9 |
      8.2 |#########
      8.5 |
      8.8 |################
  (11 below, 11 above range)

rot_manual_shifts (n=160, range 3.7-8.7 ns)
      3.7 |#################
      3.9 |
      4.2 |####################
      4.4 |###################################
      4.7 |
      4.9 |#################################
      5.2 |###################################
      5.4 |
      5.7 |########################################
      5.9 |
      6.2 |############################
      6.4 |
      6.7 |###############################
      6.9 |#############
      7.2 |
      7.4 |###############
      7.7 |#############
      7.9 |
      8.2 |#############
      8.4 |
  (13 below, 13 above range)

rot_variable_amount (n=160, range 42.4-55.3 ns)
     42.4 |#########################
     43.1 |##############
     43.7 |#####################################
     44.4 |####################
     45.0 |######################
     45.6 |########################################
     46.3 |##############
     46.9 |############################
     47.6 |#################
     48.2 |#########################
     48.9 |###########
     49.5 |###############################
     50.1 |######################
     50.8 |##############
     51.4 |#####
     52.1 |##############
     52.7 |##############
     53.4 |########
     54.0 |###########
     54.6 |#####
  (11 below, 14 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=32.9% (high variance, measurements may be unstable)
- **rot_intrinsic_const**: bridge=193.1% of algo (FFI overhead may distort results)
- **rot_manual_shifts**: CV=30.5% (high variance, measurements may be unstable)
- **rot_manual_shifts**: bridge=208.6% of algo (FFI overhead may distort results)
