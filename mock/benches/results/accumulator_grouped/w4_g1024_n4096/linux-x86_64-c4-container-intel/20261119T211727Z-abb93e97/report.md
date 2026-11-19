# W = 4, G = 1024

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 54% faster than the next best (grouped)

sequential (27.85 us) leads grouped (42.93 us) by 54%, a clear separation rather than a photo finish. CV 6.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 38% (significant)

sequential is -16.29 us (38%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### per_core shows alternating (throttle bounce) (autocorr -0.65)

per_core's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### grouped is inconsistent: worst-20% is 3.9x its best-20%

grouped's best 20% of batches run at 39.70 us but its worst 20% at 156.57 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 27848.8 ns median (-35.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.57x (fastest 27848.8 ns, slowest 43639.2 ns)

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
| Counter frequency | 2100372148 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 80580ns | 43496ns | 40242ns | 42928ns | 157228ns | base |
| per_core | 45142ns | 44416ns | 41415ns | 43758ns | 49082ns | -43.98% |
| sequential | 28435ns | 28645ns | 25379ns | 28037ns | 30560ns | -64.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 80012ns | 39704ns | 156565ns | base | 0.051 |
| per_core | 44523ns | 40808ns | 48331ns | -44.36% | 0.092 |
| sequential | 27724ns | 24958ns | 29736ns | -65.35% | 0.148 |

## Performance model

- Peak throughput: **0.164 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.095 | 58.1% |
| per_core | 0.094 | 57.2% |
| sequential | 0.147 | 89.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 80580ns | 80580ns | base |
| per_core | 45142ns | 45142ns | -43.98% |
| sequential | 28435ns | 28435ns | -64.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 42933ns | base | --- | [41137, 47231] | --- | --- | --- | --- |
| per_core | 43639ns | no significant difference | [-3249, +1972]ns | [42640, 44312] | no | 0.4807 | 0.4807 | 0 |
| sequential | 27849ns | -16290.7ns (-37.9%) | [-19376, -13143]ns | [26574, 28564] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 45290ns | -3.1% | -35.5% |
| 1 | 2 | 43157ns | -1.9% | -35.4% |
| 1 | 3 | 42219ns | +3.4% | -38.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.032 | ok |
| per_core | -0.654 | HIGH- (thermal bounce) |
| sequential | -0.006 | ok |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 52081.1ns | 80012.1ns | 65.1% | HIGH |
| per_core | 44950.9ns | 44522.7ns | 101.0% | HIGH |
| sequential | 27657.3ns | 27724.5ns | 99.8% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 39703.9-156565.2 ns)
  39703.9 |########################################
  45546.9 |##########
  51390.0 |###
  57233.1 |
  63076.1 |
  68919.2 |
  74762.3 |
  80605.4 |
  86448.4 |
  92291.5 |
  98134.6 |
  103977.6 |
  109820.7 |
  115663.8 |
  121506.8 |
  127349.9 |
  133193.0 |
  139036.0 |
  144879.1 |
  150722.2 |
  (1 below, 2 above range)

per_core (n=18, range 40808.2-48331.2 ns)
  40808.2 |####################
  41184.3 |
  41560.5 |##########
  41936.6 |
  42312.8 |##########
  42688.9 |####################
  43065.1 |
  43441.2 |##############################
  43817.4 |##########
  44193.5 |########################################
  44569.7 |
  44945.8 |##########
  45322.0 |##########
  45698.1 |
  46074.3 |
  46450.5 |
  46826.6 |
  47202.8 |
  47578.9 |
  47955.1 |
  (1 below, 1 above range)

sequential (n=18, range 24957.9-29736.1 ns)
  24957.9 |
  25196.8 |########################################
  25435.7 |
  25674.6 |
  25913.5 |
  26152.4 |
  26391.3 |####################
  26630.2 |####################
  26869.2 |
  27108.1 |####################
  27347.0 |
  27585.9 |########################################
  27824.8 |####################
  28063.7 |####################
  28302.6 |########################################
  28541.5 |####################
  28780.4 |
  29019.4 |
  29258.3 |####################
  29497.2 |
  (2 below, 3 above range)

```

## Diagnostics

- **grouped**: CV=138.0% (high variance, measurements may be unstable)
- **grouped**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **grouped**: bridge=101.5% of algo (FFI overhead may distort results)
- **per_core**: bridge=100.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=97.7% of algo (FFI overhead may distort results)
