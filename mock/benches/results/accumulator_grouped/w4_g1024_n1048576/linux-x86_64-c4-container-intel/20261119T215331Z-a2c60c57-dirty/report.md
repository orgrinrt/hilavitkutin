# W = 4, G = 1024

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 2.1x slower than the field

sequential (4.69 ms) is 2.1x the fastest (2.28 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (per_core, grouped) are a dead heat (<1%)

per_core (2.28 ms) and grouped (2.28 ms) differ by 0.16%, inside the noise, even though the wider field spreads 105.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### per_core shows alternating (throttle bounce) (autocorr -0.61)

per_core's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: per_core** at 2277395.3 ns median (-0.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.06x (fastest 2277395.3 ns, slowest 4685870.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 1024 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 2268549ns | 2282975ns | 1808955ns | 2227119ns | 2560490ns | base |
| per_core | 2342166ns | 2279431ns | 2051040ns | 2250095ns | 2625835ns | +3.25% |
| sequential | 4726286ns | 4688468ns | 4502828ns | 4677740ns | 4910835ns | +108.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 2266409ns | 1806663ns | 2558553ns | base | 0.463 |
| per_core | 2340126ns | 2049068ns | 2623507ns | +3.25% | 0.448 |
| sequential | 4723423ns | 4500570ns | 4908089ns | +108.41% | 0.222 |

## Performance model

- Peak throughput: **0.580 Gops/s** (grouped; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.460 | 79.2% |
| per_core | 0.460 | 79.3% |
| sequential | 0.224 | 38.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 2268549ns | 2268549ns | base |
| per_core | 2342166ns | 2342166ns | +3.25% |
| sequential | 4726286ns | 4726286ns | +108.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 2281113ns | base | --- | [2082766, 2480501] | --- | --- | --- | --- |
| per_core | 2277395ns | no significant difference | [-98831, +246034]ns | [2183474, 2520007] | no | 0.4807 | 0.4807 | 0 |
| sequential | 4685871ns | +2385614.9ns (+104.6%) | [+2172963, +2605418]ns | [4640118, 4774402] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 2451080ns | -10.7% | +94.3% |
| 1 | 2 | 2360456ns | +4.9% | +98.7% |
| 1 | 3 | 2187957ns | +3.9% | +113.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.021 | ok |
| per_core | -0.614 | HIGH- (thermal bounce) |
| sequential | -0.041 | ok |

**Consistency summary:**

- **per_core**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 2282466.9ns | 2266408.9ns | 100.7% | HIGH |
| per_core | 2315945.8ns | 2340126.2ns | 99.0% | HIGH |
| sequential | 4754525.8ns | 4723422.8ns | 100.7% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 1806663.1-2558552.7 ns)
  1806663.1 |
  1844257.5 |#############
  1881852.0 |
  1919446.5 |#############
  1957041.0 |
  1994635.5 |#############
  2032230.0 |#############
  2069824.4 |
  2107418.9 |#############
  2145013.4 |
  2182607.9 |
  2220202.4 |##########################
  2257796.8 |##########################
  2295391.3 |
  2332985.8 |#############
  2370580.3 |
  2408174.8 |#############
  2445769.3 |#############
  2483363.7 |########################################
  2520958.2 |#############
  (1 below, 1 above range)

per_core (n=18, range 2049068.3-2623506.8 ns)
  2049068.3 |
  2077790.2 |#############
  2106512.2 |
  2135234.1 |########################################
  2163956.0 |
  2192677.9 |##########################
  2221399.9 |##########################
  2250121.8 |
  2278843.7 |
  2307565.6 |##########################
  2336287.6 |
  2365009.5 |
  2393731.4 |#############
  2422453.3 |
  2451175.3 |
  2479897.2 |#############
  2508619.1 |
  2537341.1 |##########################
  2566063.0 |#############
  2594784.9 |
  (1 below, 2 above range)

sequential (n=18, range 4500569.6-4908088.8 ns)
  4500569.6 |
  4520945.6 |
  4541321.5 |
  4561697.5 |
  4582073.5 |
  4602449.4 |########################################
  4622825.4 |####################
  4643201.3 |########################################
  4663577.3 |########################################
  4683953.3 |####################
  4704329.2 |####################
  4724705.2 |
  4745081.1 |########################################
  4765457.1 |####################
  4785833.0 |####################
  4806209.0 |####################
  4826585.0 |
  4846960.9 |
  4867336.9 |####################
  4887712.8 |
  (2 below, 1 above range)

```

## Diagnostics

- **grouped**: bridge=101.0% of algo (FFI overhead may distort results)
- **per_core**: bridge=98.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.3% of algo (FFI overhead may distort results)
