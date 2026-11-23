# W = 2, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential shows alternating (throttle bounce) (autocorr -0.65)

sequential's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (grouped)

The baseline grouped is the fastest (18.47 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (grouped) is the fastest** at 18473.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.13x (fastest 18473.6 ns, slowest 20867.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 2, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112789207 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 20572ns | 19507ns | 17816ns | 19420ns | 23679ns | base |
| per_core | 20996ns | 20213ns | 18117ns | 19681ns | 24408ns | +2.06% |
| sequential | 23139ns | 21445ns | 19763ns | 21389ns | 27452ns | +12.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 19702ns | 17262ns | 22653ns | base | 0.208 |
| per_core | 20172ns | 17506ns | 23299ns | +2.39% | 0.203 |
| sequential | 22255ns | 19205ns | 26309ns | +12.96% | 0.184 |

## Performance model

- Peak throughput: **0.237 Gops/s** (grouped; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.222 | 93.4% |
| per_core | 0.214 | 90.3% |
| sequential | 0.196 | 82.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 20572ns | 20572ns | base |
| per_core | 20996ns | 20996ns | +2.06% |
| sequential | 23139ns | 23139ns | +12.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 18474ns | base | --- | [17965, 21183] | --- | --- | --- | --- |
| per_core | 19110ns | no significant difference | [-2915, +2203]ns | [18439, 20719] | no | 0.8145 | 0.8145 | 0 |
| sequential | 20868ns | +2363.3ns (+12.8%) | [+589, +5454]ns | [19550, 24450] | YES (adj: no) | 0.0618 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 17965ns | +18.6% | +14.0% |
| 1 | 2 | 18419ns | +2.1% | +39.4% |
| 1 | 3 | 21585ns | -17.2% | -9.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.105 | ok |
| per_core | -0.044 | ok |
| sequential | -0.652 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 1/3, lost 2/3
- **sequential**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 20519.4ns | 19701.5ns | 104.2% | HIGH |
| per_core | 19709.1ns | 20172.5ns | 97.7% | HIGH |
| sequential | 23004.0ns | 22255.1ns | 103.4% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 17262.3-22653.2 ns)
  17262.3 |
  17531.9 |##########
  17801.4 |########################################
  18071.0 |##########
  18340.5 |####################
  18610.1 |
  18879.6 |##########
  19149.1 |
  19418.7 |
  19688.2 |
  19957.8 |
  20227.3 |
  20496.9 |
  20766.4 |##########
  21036.0 |####################
  21305.5 |
  21575.1 |
  21844.6 |##########
  22114.1 |
  22383.7 |
  (2 below, 3 above range)

per_core (n=18, range 17505.9-23299.1 ns)
  17505.9 |##########################
  17795.6 |
  18085.3 |##########################
  18374.9 |#############
  18664.6 |########################################
  18954.2 |
  19243.9 |#############
  19533.5 |
  19823.2 |##########################
  20112.9 |#############
  20402.5 |
  20692.2 |
  20981.8 |
  21271.5 |#############
  21561.2 |
  21850.8 |
  22140.5 |
  22430.1 |#############
  22719.8 |
  23009.5 |
  (1 below, 3 above range)

sequential (n=18, range 19204.9-26308.6 ns)
  19204.9 |########################################
  19560.1 |
  19915.3 |
  20270.4 |##############################
  20625.6 |
  20980.8 |##########
  21336.0 |
  21691.2 |##########
  22046.4 |##########
  22401.5 |
  22756.7 |
  23111.9 |
  23467.1 |
  23822.3 |
  24177.5 |##########
  24532.6 |##########
  24887.8 |
  25243.0 |
  25598.2 |
  25953.4 |
  (2 below, 4 above range)

```

## Diagnostics

- **grouped**: bridge=105.4% of algo (FFI overhead may distort results)
- **per_core**: bridge=99.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=102.4% of algo (FFI overhead may distort results)
