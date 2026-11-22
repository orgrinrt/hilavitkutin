# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 254% faster than the next best (match_64_arms)

match_2_arms (20 ns) leads match_64_arms (71 ns) by 254%, a clear separation rather than a photo finish. CV 14.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_8_arms is an outlier: 4.8x slower than the field

match_8_arms (95 ns) is 4.8x the fastest (20 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### match_2_arms is fastest but the noisiest (CV 14.0%)

match_2_arms wins on median (20 ns) yet has the highest variance (CV 14.0%), while match_8_arms is the steadiest (CV 4.8%, 95 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (20 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.8x the fastest

Fastest match_2_arms (20 ns) to slowest match_8_arms (95 ns): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 20.0 ns median
- 2 variants significantly slower than baseline
- Spread: 4.76x (fastest 20.0 ns, slowest 95.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x3c8715d9a4b682f1 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| match_2_arms | 2137ns | 2130ns | 2123ns | 2130ns | 2171ns | base |
| match_64_arms | 2184ns | 2170ns | 2162ns | 2171ns | 2244ns | +2.21% |
| match_8_arms | 2204ns | 2198ns | 2190ns | 2200ns | 2231ns | +3.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 20ns | 17ns | 24ns | base | 12.693 |
| match_64_arms | 71ns | 65ns | 77ns | +251.74% | 3.609 |
| match_8_arms | 96ns | 90ns | 102ns | +373.70% | 2.680 |

## Performance model

- Peak throughput: **15.112 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 12.800 | 84.7% |
| match_64_arms | 3.616 | 23.9% |
| match_8_arms | 2.689 | 17.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2137ns | 2137ns | base |
| match_64_arms | 2184ns | 2184ns | +2.21% |
| match_8_arms | 2204ns | 2204ns | +3.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 20ns | base | --- | [20, 20] | --- | --- | --- | --- |
| match_64_arms | 71ns | +50.8ns (+254.0%) | [+50, +52]ns | [70, 71] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 95ns | +75.4ns (+377.0%) | [+75, +76]ns | [95, 96] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 20ns | +254.1% | +388.8% |
| 1 | 2 | 20ns | +251.5% | +365.7% |
| 1 | 3 | 20ns | +253.4% | +358.6% |
| 1 | 4 | 19ns | +279.5% | +403.2% |
| 1 | 5 | 20ns | +254.1% | +384.7% |
| 1 | 6 | 21ns | +224.3% | +355.0% |
| 1 | 7 | 20ns | +247.0% | +379.8% |
| 1 | 8 | 20ns | +254.7% | +372.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.244 | moderate- |
| match_64_arms | 0.265 | moderate+ |
| match_8_arms | -0.213 | moderate- |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.8ns | 20.2ns | 14.0% | HIGH |
| match_64_arms | 2.9ns | 70.9ns | 4.1% |  |
| match_8_arms | 2.4ns | 95.5ns | 2.6% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 16.9-23.9 ns)
     16.9 |########################
     17.3 |#########
     17.6 |########################
     18.0 |#####################
     18.3 |
     18.7 |####################################
     19.0 |#################################
     19.4 |####################################
     19.7 |########################################
     20.1 |
     20.4 |#####################
     20.7 |###########################
     21.1 |#################################
     21.4 |#####################
     21.8 |###########################
     22.1 |
     22.5 |#####################
     22.8 |###############
     23.2 |#####################
     23.5 |###
  (15 below, 8 above range)

match_64_arms (n=160, range 65.2-77.0 ns)
     65.2 |##############################
     65.8 |#########
     66.4 |######
     67.0 |#####################
     67.6 |##################
     68.2 |##############################
     68.8 |#################################
     69.4 |#####################
     69.9 |####################################
     70.5 |##############################
     71.1 |#################################
     71.7 |##################
     72.3 |############
     72.9 |############
     73.5 |###############
     74.1 |#####################
     74.7 |#####################
     75.2 |########################################
     75.8 |#########
     76.4 |
  (11 below, 11 above range)

match_8_arms (n=160, range 89.9-102.1 ns)
     89.9 |###########
     90.5 |##############
     91.1 |########################################
     91.7 |#######
     92.3 |#####################
     92.9 |###########
     93.5 |####
     94.2 |############################
     94.8 |##################
     95.4 |#######################
     96.0 |################
     96.6 |############################
     97.2 |####
     97.8 |#######################
     98.4 |################
     99.1 |##############
     99.7 |##
    100.3 |##############
    100.9 |#######
    101.5 |##############
  (13 below, 10 above range)

```

## Diagnostics

- **match_2_arms**: bridge=14.5% of algo (FFI overhead may distort results)
