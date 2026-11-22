# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 164% faster than the next best (match_64_arms)

match_2_arms (26 ns) leads match_64_arms (69 ns) by 164%, a clear separation rather than a photo finish. CV 13.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_8_arms is an outlier: 3.6x slower than the field

match_8_arms (93 ns) is 3.6x the fastest (26 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### match_2_arms is fastest but the noisiest (CV 13.3%)

match_2_arms wins on median (26 ns) yet has the highest variance (CV 13.3%), while match_8_arms is the steadiest (CV 4.8%, 93 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### match_8_arms shows warm-up / thermal drift (autocorr +0.53)

match_8_arms's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

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
| match_2_arms | 2107ns | 2104ns | 2098ns | 2104ns | 2125ns | base |
| match_64_arms | 2151ns | 2140ns | 2133ns | 2141ns | 2200ns | +2.10% |
| match_8_arms | 2172ns | 2169ns | 2162ns | 2169ns | 2193ns | +3.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 26ns | 22ns | 31ns | base | 9.699 |
| match_64_arms | 69ns | 64ns | 75ns | +162.15% | 3.700 |
| match_8_arms | 93ns | 87ns | 99ns | +253.46% | 2.744 |

## Performance model

- Peak throughput: **11.726 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 9.771 | 83.3% |
| match_64_arms | 3.699 | 31.5% |
| match_8_arms | 2.744 | 23.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2107ns | 2107ns | base |
| match_64_arms | 2151ns | 2151ns | +2.10% |
| match_8_arms | 2172ns | 2172ns | +3.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 26ns | base | --- | [25, 27] | --- | --- | --- | --- |
| match_64_arms | 69ns | +42.9ns (+163.7%) | [+42, +43]ns | [68, 70] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 93ns | +67.1ns (+256.1%) | [+67, +68]ns | [92, 94] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 26ns | +156.4% | +251.6% |
| 1 | 2 | 27ns | +165.2% | +247.2% |
| 1 | 3 | 25ns | +173.2% | +265.7% |
| 1 | 4 | 26ns | +169.5% | +257.1% |
| 1 | 5 | 27ns | +165.7% | +255.0% |
| 1 | 6 | 26ns | +163.2% | +265.1% |
| 1 | 7 | 25ns | +168.1% | +267.3% |
| 1 | 8 | 25ns | +168.9% | +267.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | 0.055 | ok |
| match_64_arms | 0.018 | ok |
| match_8_arms | 0.530 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.9ns | 26.4ns | 11.0% | HIGH |
| match_64_arms | 2.7ns | 69.2ns | 3.9% |  |
| match_8_arms | 2.1ns | 93.3ns | 2.3% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 21.8-31.4 ns)
     21.8 |#####
     22.3 |####################
     22.8 |#################
     23.3 |########################################
     23.8 |######################
     24.2 |###############################
     24.7 |#################
     25.2 |####################
     25.7 |####################
     26.2 |####################
     26.6 |########################################
     27.1 |###########
     27.6 |#################
     28.1 |############################
     28.6 |##############
     29.0 |######################
     29.5 |##############
     30.0 |##############
     30.5 |
     31.0 |###########
  (10 below, 14 above range)

match_64_arms (n=160, range 63.5-75.1 ns)
     63.5 |########
     64.1 |########################
     64.7 |#####
     65.3 |########################
     65.8 |##################
     66.4 |##########
     67.0 |########################################
     67.6 |#####################
     68.1 |#####################
     68.7 |#############
     69.3 |#############
     69.9 |################################
     70.4 |########################
     71.0 |##########
     71.6 |#############
     72.2 |##########################
     72.7 |##################
     73.3 |##########
     73.9 |##########
     74.5 |#####################
  (13 below, 9 above range)

match_8_arms (n=160, range 87.0-99.4 ns)
     87.0 |####
     87.7 |####
     88.3 |#############
     88.9 |###########
     89.5 |########################
     90.1 |###########
     90.8 |########################
     91.4 |########
     92.0 |#################
     92.6 |########
     93.2 |########################################
     93.9 |####
     94.5 |######################
     95.1 |########
     95.7 |####################
     96.3 |#################
     96.9 |####################
     97.6 |#################
     98.2 |###########
     98.8 |####
  (16 below, 11 above range)

```

## Diagnostics

- **match_2_arms**: bridge=11.1% of algo (FFI overhead may distort results)
- **match_8_arms**: autocorrelation=0.53 (measurement drift or warm-up artifact)
