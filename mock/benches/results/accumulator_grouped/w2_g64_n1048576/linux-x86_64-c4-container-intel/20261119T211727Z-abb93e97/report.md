# W = 2, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core dominates: 15% faster than the next best (grouped)

per_core (2.51 ms) leads grouped (2.89 ms) by 15%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### grouped shows alternating (throttle bounce) (autocorr -0.66)

grouped's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: per_core** at 2513529.5 ns median (-12.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.97x (fastest 2513529.5 ns, slowest 4950247.3 ns)

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
| Counter frequency | 2100372148 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 2900375ns | 2888159ns | 2670382ns | 2826545ns | 3126117ns | base |
| per_core | 2546029ns | 2516290ns | 2420308ns | 2506721ns | 2667851ns | -12.22% |
| sequential | 4943772ns | 4952748ns | 4648573ns | 4909668ns | 5142527ns | +70.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 2897741ns | 2667443ns | 3123431ns | base | 0.362 |
| per_core | 2543113ns | 2418153ns | 2664068ns | -12.24% | 0.412 |
| sequential | 4941118ns | 4646182ns | 5139802ns | +70.52% | 0.212 |

## Performance model

- Peak throughput: **0.434 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.363 | 83.8% |
| per_core | 0.417 | 96.2% |
| sequential | 0.212 | 48.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 2900375ns | 2900375ns | base |
| per_core | 2546029ns | 2546029ns | -12.22% |
| sequential | 4943772ns | 4943772ns | +70.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 2885909ns | base | --- | [2742510, 3012760] | --- | --- | --- | --- |
| per_core | 2513530ns | -372555.3ns (-12.9%) | [-462200, -259308]ns | [2489467, 2549959] | YES | 0.0013 | 0.0013 | 0 |
| sequential | 4950247ns | +2016235.8ns (+69.9%) | [+1827950, +2237548]ns | [4852583, 5035772] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 2947635ns | -14.9% | +69.2% |
| 1 | 2 | 2742510ns | -5.4% | +83.1% |
| 1 | 3 | 2926811ns | -13.7% | +61.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.661 | HIGH- (thermal bounce) |
| per_core | -0.644 | HIGH- (thermal bounce) |
| sequential | -0.229 | moderate- |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 3057402.2ns | 2897741.3ns | 105.5% | HIGH |
| per_core | 2554961.0ns | 2543113.4ns | 100.5% | HIGH |
| sequential | 4962092.6ns | 4941118.2ns | 100.4% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 2667442.6-3123431.3 ns)
  2667442.6 |#############
  2690242.0 |#############
  2713041.5 |
  2735840.9 |########################################
  2758640.3 |
  2781439.8 |
  2804239.2 |#############
  2827038.7 |
  2849838.1 |#############
  2872637.5 |
  2895437.0 |#############
  2918236.4 |#############
  2941035.8 |
  2963835.3 |##########################
  2986634.7 |
  3009434.2 |
  3032233.6 |#############
  3055033.0 |
  3077832.5 |
  3100631.9 |##########################
  (2 below, 2 above range)

per_core (n=18, range 2418153.3-2664068.2 ns)
  2418153.3 |#############
  2430449.0 |
  2442744.8 |#############
  2455040.5 |
  2467336.3 |#############
  2479632.0 |#############
  2491927.8 |########################################
  2504223.5 |##########################
  2516519.3 |#############
  2528815.0 |#############
  2541110.8 |#############
  2553406.5 |#############
  2565702.3 |
  2577998.0 |
  2590293.8 |
  2602589.5 |#############
  2614885.2 |
  2627181.0 |
  2639476.7 |
  2651772.5 |
  (1 below, 3 above range)

sequential (n=18, range 4646182.4-5139802.5 ns)
  4646182.4 |########################################
  4670863.4 |
  4695544.4 |
  4720225.4 |
  4744906.4 |####################
  4769587.4 |
  4794268.4 |####################
  4818949.4 |
  4843630.4 |
  4868311.4 |
  4892992.4 |########################################
  4917673.4 |########################################
  4942354.4 |########################################
  4967035.4 |
  4991716.4 |####################
  5016397.5 |########################################
  5041078.5 |
  5065759.5 |
  5090440.5 |
  5115121.5 |####################
  (1 below, 3 above range)

```

## Diagnostics

- **grouped**: bridge=105.4% of algo (FFI overhead may distort results)
- **per_core**: bridge=100.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.3% of algo (FFI overhead may distort results)
