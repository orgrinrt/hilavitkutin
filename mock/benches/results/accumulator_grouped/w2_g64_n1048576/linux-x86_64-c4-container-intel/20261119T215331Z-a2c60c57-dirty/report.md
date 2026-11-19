# W = 2, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: per_core** at 2486328.6 ns median (-2.5% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.83x (fastest 2486328.6 ns, slowest 4543967.0 ns)

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
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 2616228ns | 2550649ns | 2417920ns | 2528431ns | 2847077ns | base |
| per_core | 2447224ns | 2488342ns | 2117061ns | 2410752ns | 2667014ns | -6.46% |
| sequential | 4552673ns | 4545838ns | 4290053ns | 4508536ns | 4750186ns | +74.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 2614329ns | 2415894ns | 2844972ns | base | 0.401 |
| per_core | 2445167ns | 2115543ns | 2664707ns | -6.47% | 0.429 |
| sequential | 4550545ns | 4288237ns | 4747564ns | +74.06% | 0.230 |

## Performance model

- Peak throughput: **0.496 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.411 | 83.0% |
| per_core | 0.422 | 85.1% |
| sequential | 0.231 | 46.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 2616228ns | 2616228ns | base |
| per_core | 2447224ns | 2447224ns | -6.46% |
| sequential | 4552673ns | 4552673ns | +74.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 2549187ns | base | --- | [2498904, 2581922] | --- | --- | --- | --- |
| per_core | 2486329ns | no significant difference | [-367701, +91676]ns | [2284398, 2603802] | no | 0.4807 | 0.4807 | 0 |
| sequential | 4543967ns | +1959674.2ns (+76.9%) | [+1865275, +2026744]ns | [4418678, 4681624] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 2551083ns | -8.7% | +84.1% |
| 1 | 2 | 2551399ns | -4.4% | +71.3% |
| 1 | 3 | 2540199ns | +2.3% | +77.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.181 | ok |
| per_core | -0.008 | ok |
| sequential | -0.462 | moderate- |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 2714269.5ns | 2614329.3ns | 103.8% | HIGH |
| per_core | 2452464.6ns | 2445167.0ns | 100.3% | HIGH |
| sequential | 4536291.0ns | 4550544.6ns | 99.7% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 2415894.3-2844971.9 ns)
  2415894.3 |####################
  2437348.2 |
  2458802.1 |##########
  2480256.0 |##########
  2501709.8 |####################
  2523163.7 |##########
  2544617.6 |########################################
  2566071.5 |##########
  2587525.3 |##########
  2608979.2 |
  2630433.1 |
  2651887.0 |
  2673340.9 |
  2694794.7 |##########
  2716248.6 |##########
  2737702.5 |
  2759156.4 |
  2780610.2 |
  2802064.1 |
  2823518.0 |
  (1 below, 2 above range)

per_core (n=18, range 2115542.6-2664707.2 ns)
  2115542.6 |
  2143000.9 |########################################
  2170459.1 |
  2197917.3 |####################
  2225375.6 |
  2252833.8 |
  2280292.0 |
  2307750.2 |####################
  2335208.5 |
  2362666.7 |
  2390124.9 |####################
  2417583.2 |
  2445041.4 |####################
  2472499.6 |########################################
  2499957.9 |
  2527416.1 |####################
  2554874.3 |####################
  2582332.6 |########################################
  2609790.8 |####################
  2637249.0 |
  (2 below, 3 above range)

sequential (n=18, range 4288237.3-4747563.9 ns)
  4288237.3 |
  4311203.7 |####################
  4334170.0 |####################
  4357136.3 |####################
  4380102.6 |####################
  4403069.0 |
  4426035.3 |####################
  4449001.6 |
  4471968.0 |
  4494934.3 |########################################
  4517900.6 |####################
  4540867.0 |
  4563833.3 |####################
  4586799.6 |####################
  4609765.9 |
  4632732.3 |####################
  4655698.6 |
  4678664.9 |########################################
  4701631.3 |####################
  4724597.6 |
  (1 below, 3 above range)

```

## Diagnostics

- **grouped**: bridge=104.8% of algo (FFI overhead may distort results)
- **per_core**: bridge=100.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.5% of algo (FFI overhead may distort results)
