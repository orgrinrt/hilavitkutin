# W = 4, G = 1024

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### grouped dominates: 31% faster than the next best (per_core)

grouped (137.89 us) leads per_core (180.33 us) by 31%, a clear separation rather than a photo finish. CV 25.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential is an outlier: 2.2x slower than the field

sequential (305.15 us) is 2.2x the fastest (137.89 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### per_core shows alternating (throttle bounce) (autocorr -0.66)

per_core's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (grouped)

The baseline grouped is the fastest (137.89 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### per_core is inconsistent: worst-20% is 3.1x its best-20%

per_core's best 20% of batches run at 127.12 us but its worst 20% at 399.61 us (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (grouped) is the fastest** at 137886.5 ns median
- 1 variant significantly slower than baseline
- Spread: 2.21x (fastest 137886.5 ns, slowest 305151.7 ns)

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
| grouped | 147973ns | 138400ns | 104416ns | 132693ns | 192673ns | base |
| per_core | 240733ns | 180941ns | 127863ns | 171680ns | 400748ns | +62.69% |
| sequential | 310976ns | 306963ns | 292283ns | 305807ns | 328077ns | +110.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 147267ns | 103916ns | 191985ns | base | 0.445 |
| per_core | 239899ns | 127118ns | 399609ns | +62.90% | 0.273 |
| sequential | 306534ns | 291238ns | 318133ns | +108.15% | 0.214 |

## Performance model

- Peak throughput: **0.631 Gops/s** (grouped; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.475 | 75.4% |
| per_core | 0.363 | 57.6% |
| sequential | 0.215 | 34.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 147973ns | 147973ns | base |
| per_core | 240733ns | 240733ns | +62.69% |
| sequential | 310976ns | 310976ns | +110.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 137887ns | base | --- | [117069, 182870] | --- | --- | --- | --- |
| per_core | 180331ns | no significant difference | [-3793, +90458]ns | [154795, 201795] | no | 0.0963 | 0.0963 | 0 |
| sequential | 305152ns | +157960.0ns (+114.6%) | [+134741, +186362]ns | [301842, 308783] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 190139ns | +0.3% | +63.7% |
| 1 | 2 | 137887ns | +8.0% | +117.7% |
| 1 | 3 | 109200ns | +69.3% | +179.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.018 | ok |
| per_core | -0.656 | HIGH- (thermal bounce) |
| sequential | -0.500 | moderate- |

**Consistency summary:**

- **per_core**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 147402.9ns | 147266.9ns | 100.1% | HIGH |
| per_core | 218774.5ns | 239899.0ns | 91.2% | HIGH |
| sequential | 303622.2ns | 306533.9ns | 99.1% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 103916.2-191984.6 ns)
  103916.2 |
  108319.6 |########################################
  112723.0 |####################
  117126.5 |####################
  121529.9 |
  125933.3 |####################
  130336.7 |########################################
  134740.1 |
  139143.6 |####################
  143547.0 |####################
  147950.4 |
  152353.8 |
  156757.2 |####################
  161160.7 |
  165564.1 |
  169967.5 |
  174370.9 |
  178774.4 |####################
  183177.8 |####################
  187581.2 |####################
  (2 below, 3 above range)

per_core (n=18, range 127117.5-399608.5 ns)
  127117.5 |
  140742.1 |########################################
  154366.6 |##########################
  167991.2 |##########################
  181615.7 |##########################
  195240.3 |########################################
  208864.8 |
  222489.4 |
  236113.9 |#############
  249738.5 |
  263363.0 |
  276987.6 |
  290612.1 |
  304236.7 |
  317861.2 |
  331485.8 |
  345110.3 |
  358734.9 |
  372359.4 |
  385984.0 |
  (2 below, 3 above range)

sequential (n=18, range 291237.5-318132.8 ns)
  291237.5 |
  292582.3 |####################
  293927.1 |
  295271.8 |
  296616.6 |
  297961.3 |
  299306.1 |########################################
  300650.9 |####################
  301995.6 |########################################
  303340.4 |####################
  304685.2 |########################################
  306029.9 |########################################
  307374.7 |####################
  308719.4 |####################
  310064.2 |
  311409.0 |
  312753.7 |########################################
  314098.5 |
  315443.2 |
  316788.0 |
  (1 below, 2 above range)

```

## Diagnostics

- **grouped**: CV=23.9% (high variance, measurements may be unstable)
- **grouped**: bridge=99.0% of algo (FFI overhead may distort results)
- **per_core**: CV=72.1% (high variance, measurements may be unstable)
- **per_core**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **per_core**: bridge=96.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.6% of algo (FFI overhead may distort results)
