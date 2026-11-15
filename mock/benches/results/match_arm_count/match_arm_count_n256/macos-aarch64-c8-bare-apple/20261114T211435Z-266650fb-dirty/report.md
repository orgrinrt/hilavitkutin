# Match arm count: 2 vs 8 vs 64 — jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 172% faster than the next best (match_64_arms)

match_2_arms (28 ns) leads match_64_arms (76 ns) by 172%, a clear separation rather than a photo finish. CV 14.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_8_arms is an outlier: 3.6x slower than the field

match_8_arms (100 ns) is 3.6x the fastest (28 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### match_2_arms is fastest but the noisiest (CV 14.5%)

match_2_arms wins on median (28 ns) yet has the highest variance (CV 14.5%), while match_8_arms is the steadiest (CV 7.3%, 100 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### match_2_arms shows alternating (throttle bounce) (autocorr -0.54)

match_2_arms's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (28 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.6x the fastest

Fastest match_2_arms (28 ns) to slowest match_8_arms (100 ns): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 27.9 ns median
- 2 variants significantly slower than baseline
- Spread: 3.58x (fastest 27.9 ns, slowest 100.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Match arm count: 2 vs 8 vs 64 — jump-table dispatch scaling |
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
| match_2_arms | 2243ns | 2197ns | 2094ns | 2230ns | 2427ns | base |
| match_64_arms | 2338ns | 2454ns | 2131ns | 2362ns | 2473ns | +4.26% |
| match_8_arms | 2326ns | 2316ns | 2157ns | 2324ns | 2502ns | +3.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 28ns | 23ns | 34ns | base | 9.065 |
| match_64_arms | 75ns | 66ns | 84ns | +167.26% | 3.392 |
| match_8_arms | 100ns | 91ns | 111ns | +255.13% | 2.553 |

## Performance model

- Peak throughput: **11.120 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 9.176 | 82.5% |
| match_64_arms | 3.377 | 30.4% |
| match_8_arms | 2.560 | 23.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2243ns | 2243ns | base |
| match_64_arms | 2338ns | 2338ns | +4.26% |
| match_8_arms | 2326ns | 2326ns | +3.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 28ns | base | --- | [27, 29] | --- | --- | --- | --- |
| match_64_arms | 76ns | +48.1ns (+172.6%) | [+47, +50]ns | [74, 77] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 100ns | +71.6ns (+256.6%) | [+69, +73]ns | [99, 101] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 26ns | +184.0% | +283.2% |
| 1 | 2 | 28ns | +178.6% | +254.6% |
| 1 | 3 | 28ns | +185.5% | +286.2% |
| 1 | 4 | 30ns | +169.1% | +222.3% |
| 1 | 5 | 25ns | +210.8% | +308.7% |
| 1 | 6 | 30ns | +120.1% | +229.6% |
| 1 | 7 | 29ns | +161.1% | +232.5% |
| 1 | 8 | 28ns | +149.6% | +252.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.543 | HIGH- (thermal bounce) |
| match_64_arms | 0.042 | ok |
| match_8_arms | -0.278 | moderate- |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.9ns | 28.2ns | 10.1% | HIGH |
| match_64_arms | 2.8ns | 75.5ns | 3.7% |  |
| match_8_arms | 2.4ns | 100.3ns | 2.4% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 23.0-34.3 ns)
     23.0 |####
     23.6 |######
     24.1 |######################
     24.7 |##############
     25.3 |########################################
     25.8 |########
     26.4 |######
     27.0 |####################
     27.5 |##########
     28.1 |##########
     28.7 |########################
     29.2 |############
     29.8 |########
     30.3 |####################
     30.9 |######
     31.5 |##########
     32.0 |########
     32.6 |######
     33.2 |############
     33.7 |######################
  (17 below, 9 above range)

match_64_arms (n=160, range 66.1-84.3 ns)
     66.1 |#####################
     67.0 |#####################
     67.9 |#########
     68.8 |############
     69.7 |#####################
     70.6 |############
     71.6 |#####################
     72.5 |########################
     73.4 |########################
     74.3 |#####################
     75.2 |#####################
     76.1 |###########################
     77.0 |###########################
     77.9 |#####################
     78.9 |#################################
     79.8 |#########
     80.7 |########################################
     81.6 |###############
     82.5 |###############
     83.4 |############
  (13 below, 12 above range)

match_8_arms (n=160, range 90.5-111.3 ns)
     90.5 |################
     91.5 |##################
     92.6 |#############
     93.6 |##################################
     94.7 |
     95.7 |##################
     96.7 |################
     97.8 |##################
     98.8 |##################################
     99.8 |########################################
    100.9 |#############
    101.9 |#############################
    103.0 |#####################
    104.0 |################
    105.0 |########
    106.1 |################
    107.1 |##########
    108.1 |#####
    109.2 |#####
    110.2 |##################
  (14 below, 13 above range)

```

## Diagnostics

- **match_2_arms**: bridge=10.4% of algo (FFI overhead may distort results)
