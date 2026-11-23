# W = 3, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential shows alternating (throttle bounce) (autocorr -0.53)

sequential's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.57 us) is smaller than the fastest variant's own run-to-run std-dev (3.30 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### per_core is inconsistent: worst-20% is 1.7x its best-20%

per_core's best 20% of batches run at 11.88 us but its worst 20% at 20.28 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: per_core** at 19163.2 ns median (-4.1% vs baseline)
- Spread: 1.13x (fastest 19163.2 ns, slowest 21733.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 3, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112788431 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 22689ns | 21033ns | 18045ns | 20386ns | 28465ns | base |
| per_core | 18393ns | 19821ns | 12393ns | 18404ns | 21375ns | -18.94% |
| sequential | 25302ns | 22768ns | 19245ns | 21978ns | 33317ns | +11.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 21726ns | 17478ns | 27235ns | base | 0.189 |
| per_core | 17551ns | 11877ns | 20280ns | -19.22% | 0.233 |
| sequential | 24183ns | 18639ns | 31531ns | +11.31% | 0.169 |

## Performance model

- Peak throughput: **0.345 Gops/s** (per_core; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.205 | 59.4% |
| per_core | 0.214 | 62.0% |
| sequential | 0.188 | 54.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 22689ns | 22689ns | base |
| per_core | 18393ns | 18393ns | -18.94% |
| sequential | 25302ns | 25302ns | +11.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 19984ns | base | --- | [18508, 25330] | --- | --- | --- | --- |
| per_core | 19163ns | no significant difference | [-11355, +1297]ns | [15605, 20123] | no | 1.0000 | 1.0000 | 0 |
| sequential | 21734ns | no significant difference | [-2244, +1986]ns | [20356, 22751] | no | 0.4758 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 27319ns | -55.6% | -18.2% |
| 1 | 2 | 19984ns | +0.9% | +3.9% |
| 1 | 3 | 18285ns | +7.2% | +29.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.077 | ok |
| per_core | -0.203 | moderate- |
| sequential | -0.527 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 1/3, lost 2/3
- **sequential**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 22458.8ns | 21726.0ns | 103.4% | HIGH |
| per_core | 18236.9ns | 17551.0ns | 103.9% | HIGH |
| sequential | 23309.7ns | 24183.4ns | 96.4% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 17477.6-27235.0 ns)
  17477.6 |
  17965.5 |##########
  18453.3 |########################################
  18941.2 |####################
  19429.1 |
  19917.0 |
  20404.8 |##########
  20892.7 |####################
  21380.6 |
  21868.4 |
  22356.3 |
  22844.2 |
  23332.0 |##########
  23819.9 |
  24307.8 |
  24795.7 |
  25283.5 |
  25771.4 |
  26259.3 |
  26747.1 |##########
  (2 below, 4 above range)

per_core (n=18, range 11877.3-20280.4 ns)
  11877.3 |########################
  12297.5 |
  12717.6 |
  13137.8 |
  13557.9 |
  13978.1 |
  14398.2 |
  14818.4 |########
  15238.6 |
  15658.7 |
  16078.9 |########
  16499.0 |
  16919.2 |
  17339.3 |
  17759.5 |
  18179.7 |########
  18599.8 |################
  19020.0 |########
  19440.1 |########
  19860.3 |########################################
  (1 below, 2 above range)

sequential (n=18, range 18639.5-31531.1 ns)
  18639.5 |########################################
  19284.0 |
  19928.6 |#############
  20573.2 |########################################
  21217.8 |#############
  21862.4 |##########################
  22507.0 |########################################
  23151.5 |#############
  23796.1 |
  24440.7 |
  25085.3 |
  25729.9 |
  26374.4 |#############
  27019.0 |#############
  27663.6 |
  28308.2 |
  28952.8 |
  29597.3 |
  30241.9 |
  30886.5 |
  (1 below, 1 above range)

```

## Diagnostics

- **grouped**: bridge=103.8% of algo (FFI overhead may distort results)
- **per_core**: bridge=103.2% of algo (FFI overhead may distort results)
- **sequential**: CV=43.2% (high variance, measurements may be unstable)
- **sequential**: bridge=104.2% of algo (FFI overhead may distort results)
