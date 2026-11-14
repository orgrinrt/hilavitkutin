# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 9.3x slower than the field

rot_variable_amount (50 ns) is 9.3x the fastest (5 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_manual_shifts is fastest but the noisiest (CV 30.6%)

rot_manual_shifts wins on median (5 ns) yet has the highest variance (CV 30.6%), while rot_variable_amount is the steadiest (CV 19.0%, 50 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 9.3x the fastest

Fastest rot_manual_shifts (5 ns) to slowest rot_variable_amount (50 ns): 9.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### rot_intrinsic_const is inconsistent: worst-20% is 2.3x its best-20%

rot_intrinsic_const's best 20% of batches run at 4 ns but its worst 20% at 8 ns (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: rot_manual_shifts** at 5.4 ns median (-6.9% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 9.30x (fastest 5.4 ns, slowest 50.2 ns)

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
| rot_intrinsic_const | 2203ns | 2222ns | 2082ns | 2179ns | 2394ns | base |
| rot_manual_shifts | 2243ns | 2241ns | 2086ns | 2239ns | 2412ns | +1.84% |
| rot_variable_amount | 2296ns | 2274ns | 2163ns | 2288ns | 2454ns | +4.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 6ns | 4ns | 8ns | base | 43.859 |
| rot_manual_shifts | 6ns | 4ns | 8ns | -1.11% | 44.353 |
| rot_variable_amount | 52ns | 44ns | 62ns | +783.71% | 4.963 |

## Performance model

- Peak throughput: **71.671 Gops/s** (rot_intrinsic_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 44.138 | 61.6% |
| rot_manual_shifts | 47.407 | 66.1% |
| rot_variable_amount | 5.100 | 7.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2203ns | 2203ns | base |
| rot_manual_shifts | 2243ns | 2243ns | +1.84% |
| rot_variable_amount | 2296ns | 2296ns | +4.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 6ns | base | --- | [5, 6] | --- | --- | --- | --- |
| rot_manual_shifts | 5ns | no significant difference | [-0, +0]ns | [5, 6] | no | 0.6255 | 0.6255 | 9 |
| rot_variable_amount | 50ns | +44.2ns (+762.1%) | [+43, +46]ns | [49, 51] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 6ns | +0.0% | +818.8% |
| 1 | 2 | 6ns | -7.1% | +810.7% |
| 1 | 3 | 6ns | -3.2% | +685.5% |
| 1 | 4 | 5ns | +16.7% | +981.3% |
| 1 | 5 | 6ns | +7.1% | +807.1% |
| 1 | 6 | 6ns | -7.1% | +789.3% |
| 1 | 7 | 6ns | -10.3% | +736.2% |
| 1 | 8 | 6ns | -16.3% | +687.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.192 | ok |
| rot_manual_shifts | -0.145 | ok |
| rot_variable_amount | -0.207 | moderate- |

**Consistency summary:**

- **rot_manual_shifts**: won 5/8, lost 2/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 11.3ns | 5.8ns | 194.4% | HIGH |
| rot_manual_shifts | 11.0ns | 5.8ns | 191.1% | HIGH |
| rot_variable_amount | 2.6ns | 51.6ns | 5.1% | HIGH |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 3.6-8.4 ns)
      3.6 |############
      3.8 |
      4.1 |################################
      4.3 |
      4.5 |######################
      4.8 |################################
      5.0 |
      5.3 |##############################
      5.5 |
      5.7 |############################
      6.0 |########################################
      6.2 |
      6.5 |########################
      6.7 |
      6.9 |######################
      7.2 |
      7.4 |##############
      7.7 |##########
      7.9 |
      8.1 |########
  (11 below, 12 above range)

rot_manual_shifts (n=160, range 3.6-8.3 ns)
      3.6 |##############
      3.9 |
      4.1 |#################################
      4.3 |
      4.6 |###############################
      4.8 |###############################
      5.0 |
      5.3 |########################################
      5.5 |
      5.7 |###############################
      6.0 |
      6.2 |#######################
      6.4 |
      6.6 |################
      6.9 |##################
      7.1 |
      7.3 |##############
      7.6 |
      7.8 |##################
      8.0 |
  (11 below, 18 above range)

rot_variable_amount (n=160, range 44.4-61.7 ns)
     44.4 |##
     45.3 |########################################
     46.2 |#########################
     47.0 |####################
     47.9 |########################################
     48.8 |########################################
     49.6 |###############################
     50.5 |######################
     51.4 |#########################
     52.2 |####################
     53.1 |##############
     53.9 |#########################
     54.8 |#################
     55.7 |#################
     56.5 |########
     57.4 |######################
     58.3 |########
     59.1 |########
     60.0 |##
     60.9 |##
  (15 below, 6 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=29.9% (high variance, measurements may be unstable)
- **rot_intrinsic_const**: bridge=193.1% of algo (FFI overhead may distort results)
- **rot_manual_shifts**: CV=28.6% (high variance, measurements may be unstable)
- **rot_manual_shifts**: bridge=207.4% of algo (FFI overhead may distort results)
