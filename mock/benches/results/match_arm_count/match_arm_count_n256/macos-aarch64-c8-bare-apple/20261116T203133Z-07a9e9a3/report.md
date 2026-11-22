# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 164% faster than the next best (match_64_arms)

match_2_arms (26 ns) leads match_64_arms (69 ns) by 164%, a clear separation rather than a photo finish. CV 12.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_8_arms is an outlier: 3.6x slower than the field

match_8_arms (93 ns) is 3.6x the fastest (26 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### match_2_arms is fastest but the noisiest (CV 12.4%)

match_2_arms wins on median (26 ns) yet has the highest variance (CV 12.4%), while match_8_arms is the steadiest (CV 5.0%, 93 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### match_2_arms shows alternating (throttle bounce) (autocorr -0.68)

match_2_arms's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (26 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.6x the fastest

Fastest match_2_arms (26 ns) to slowest match_8_arms (93 ns): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 26.2 ns median
- 2 variants significantly slower than baseline
- Spread: 3.56x (fastest 26.2 ns, slowest 93.3 ns)

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
| match_2_arms | 2111ns | 2105ns | 2099ns | 2106ns | 2140ns | base |
| match_64_arms | 2157ns | 2142ns | 2134ns | 2143ns | 2219ns | +2.15% |
| match_8_arms | 2173ns | 2171ns | 2163ns | 2170ns | 2193ns | +2.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 26ns | 22ns | 31ns | base | 9.725 |
| match_64_arms | 69ns | 64ns | 75ns | +163.23% | 3.695 |
| match_8_arms | 93ns | 87ns | 100ns | +253.89% | 2.748 |

## Performance model

- Peak throughput: **11.660 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 9.771 | 83.8% |
| match_64_arms | 3.699 | 31.7% |
| match_8_arms | 2.744 | 23.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2111ns | 2111ns | base |
| match_64_arms | 2157ns | 2157ns | +2.15% |
| match_8_arms | 2173ns | 2173ns | +2.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 26ns | base | --- | [26, 27] | --- | --- | --- | --- |
| match_64_arms | 69ns | +42.9ns (+163.7%) | [+42, +43]ns | [68, 70] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 93ns | +67.1ns (+256.1%) | [+66, +68]ns | [92, 94] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 26ns | +166.8% | +265.4% |
| 1 | 2 | 27ns | +157.1% | +241.9% |
| 1 | 3 | 26ns | +169.0% | +262.6% |
| 1 | 4 | 26ns | +178.5% | +254.7% |
| 1 | 5 | 26ns | +158.2% | +256.9% |
| 1 | 6 | 26ns | +160.1% | +262.6% |
| 1 | 7 | 27ns | +151.3% | +246.7% |
| 1 | 8 | 25ns | +172.2% | +266.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.678 | HIGH- (thermal bounce) |
| match_64_arms | 0.035 | ok |
| match_8_arms | -0.436 | moderate- |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.9ns | 26.3ns | 11.0% | HIGH |
| match_64_arms | 2.5ns | 69.3ns | 3.6% |  |
| match_8_arms | 2.1ns | 93.2ns | 2.2% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 22.0-31.1 ns)
     22.0 |#################
     22.4 |######################
     22.9 |################################
     23.3 |#################
     23.8 |##########
     24.2 |###############
     24.7 |#######
     25.1 |###############
     25.6 |###########################
     26.1 |######################
     26.5 |####################
     27.0 |#################
     27.4 |#######
     27.9 |########################################
     28.3 |###############
     28.8 |#####
     29.2 |############
     29.7 |############
     30.2 |##########
     30.6 |##########
  (10 below, 15 above range)

match_64_arms (n=160, range 63.5-75.4 ns)
     63.5 |##
     64.1 |####################
     64.7 |#################
     65.3 |##########
     65.9 |#######
     66.5 |#################
     67.1 |#########################
     67.7 |#########################
     68.3 |##############################
     68.9 |######################
     69.5 |########################################
     70.0 |############
     70.6 |###########################
     71.2 |#####
     71.8 |#######
     72.4 |#########################
     73.0 |##########
     73.6 |##
     74.2 |##########
     74.8 |###############
  (14 below, 13 above range)

match_8_arms (n=160, range 86.8-99.7 ns)
     86.8 |#########
     87.4 |########################
     88.1 |#####################
     88.7 |#################################
     89.4 |###############
     90.0 |######
     90.6 |#################################
     91.3 |############
     91.9 |##################
     92.6 |##################
     93.2 |###########################
     93.9 |########################
     94.5 |########################################
     95.2 |##################
     95.8 |########################################
     96.4 |#########
     97.1 |##################
     97.7 |##################
     98.4 |######
     99.0 |##################
  (13 below, 12 above range)

```

## Diagnostics

- **match_2_arms**: bridge=11.1% of algo (FFI overhead may distort results)
