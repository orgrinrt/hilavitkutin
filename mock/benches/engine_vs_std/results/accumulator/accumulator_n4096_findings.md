# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std dominates: 738% faster than the next best (engine)

std (1.96 us) leads engine (16.42 us) by 738%, a clear separation rather than a photo finish. CV 29.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std beats baseline by 88% (significant)

std is -14.46 us (88%) faster than baseline engine, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### engine_par is an outlier: 28.9x slower than the field

engine_par (56.67 us) is 28.9x the fastest (1.96 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std is fastest but the noisiest (CV 29.5%)

std wins on median (1.96 us) yet has the highest variance (CV 29.5%), while engine_par is the steadiest (CV 7.5%, 56.67 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### std_par shows alternating (throttle bounce) (autocorr -0.50)

std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {std} vs {engine, std_par, engine_par} (738% apart)

The field splits into a fast tier {std} and a slow tier {engine, std_par, engine_par} with a 738% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 28.9x the fastest

Fastest std (1.96 us) to slowest engine_par (56.67 us): 28.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### std is inconsistent: worst-20% is 1.8x its best-20%

std's best 20% of batches run at 1.39 us but its worst 20% at 2.55 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std** at 1959.0 ns median (-88.1% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 28.93x (fastest 1959.0 ns, slowest 56667.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | accumulator, a transform feeding the append surface |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000003 |
| Counter frequency | 2300371984 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 16592ns | 16558ns | 12190ns | 12413ns | 20827ns | base |
| engine_par | 57293ns | 56883ns | 52345ns | 54677ns | 60493ns | +245.31% |
| std | 2142ns | 2154ns | 1502ns | 1540ns | 2754ns | -87.09% |
| std_par | 52585ns | 49236ns | 41854ns | 47815ns | 58846ns | +216.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 16404ns | 12050ns | 20603ns | base | 0.250 |
| engine_par | 57063ns | 52065ns | 60286ns | +247.86% | 0.072 |
| std | 1968ns | 1387ns | 2547ns | -88.00% | 2.081 |
| std_par | 52137ns | 41585ns | 58305ns | +217.83% | 0.079 |

## Performance model

- Peak throughput: **2.952 Gops/s** (std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.250 | 8.5% |
| engine_par | 0.072 | 2.4% |
| std | 2.091 | 70.8% |
| std_par | 0.084 | 2.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 16592ns | 16592ns | base |
| engine_par | 57293ns | 57293ns | +245.31% |
| std | 2142ns | 2142ns | -87.09% |
| std_par | 52585ns | 52585ns | +216.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 16416ns | base | --- | [12070, 20410] | --- | --- | --- | --- |
| engine_par | 56667ns | +39834.3ns (+242.7%) | [+33020, +46635]ns | [53376, 59110] | YES | 0.0078 | 0.0078 | 0 |
| std | 1959ns | -14456.5ns (-88.1%) | [-17863, -10683]ns | [1389, 2562] | YES | 0.0078 | 0.0078 | 0 |
| std_par | 48808ns | +36320.4ns (+221.3%) | [+24713, +45729]ns | [45108, 62633] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | engine_par | std | std_par |
|---|---|---|---|---|---|
| 1 | 1 | 20402ns | +168.4% | -87.5% | +130.5% |
| 1 | 2 | 12148ns | +378.7% | -88.6% | +338.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| std | -0.500 | moderate- |
| std_par | -0.500 | moderate- |

**Consistency summary:**

- **engine_par**: won 0/2, lost 2/2
- **std**: won 2/2, lost 0/2
- **std_par**: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 19327.6ns | 16404.2ns | 117.8% | HIGH |
| engine_par | 19361.3ns | 57062.8ns | 33.9% | HIGH |
| std | 18321.3ns | 1967.9ns | 931.0% | HIGH |
| std_par | 19645.2ns | 52137.4ns | 37.7% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 12049.5-20603.2 ns)
  12049.5 |########################################
  12477.2 |
  12904.9 |
  13332.6 |
  13760.2 |
  14187.9 |
  14615.6 |
  15043.3 |
  15471.0 |
  15898.7 |
  16326.4 |
  16754.0 |
  17181.7 |
  17609.4 |
  18037.1 |
  18464.8 |
  18892.5 |
  19320.1 |
  19747.8 |
  20175.5 |##############################
  (0 below, 1 above range)

engine_par (n=8, range 52064.8-60285.5 ns)
  52064.8 |########################################
  52475.8 |
  52886.9 |
  53297.9 |########################################
  53708.9 |########################################
  54120.0 |
  54531.0 |
  54942.1 |
  55353.1 |
  55764.1 |########################################
  56175.2 |
  56586.2 |
  56997.2 |########################################
  57408.3 |
  57819.3 |
  58230.3 |########################################
  58641.4 |
  59052.4 |########################################
  59463.5 |
  59874.5 |
  (0 below, 1 above range)

std (n=8, range 1387.3-2546.6 ns)
   1387.3 |########################################
   1445.3 |
   1503.2 |
   1561.2 |
   1619.2 |
   1677.1 |
   1735.1 |
   1793.1 |
   1851.0 |
   1909.0 |
   1967.0 |
   2024.9 |
   2082.9 |
   2140.9 |
   2198.8 |
   2256.8 |
   2314.8 |
   2372.7 |
   2430.7 |
   2488.7 |####################
  (0 below, 2 above range)

std_par (n=8, range 41585.1-58305.2 ns)
  41585.1 |#############
  42421.1 |
  43257.1 |
  44093.1 |
  44929.1 |#############
  45765.1 |
  46601.1 |
  47437.1 |
  48273.1 |########################################
  49109.1 |
  49945.1 |
  50781.2 |
  51617.2 |
  52453.2 |
  53289.2 |
  54125.2 |
  54961.2 |
  55797.2 |
  56633.2 |
  57469.2 |#############
  (0 below, 2 above range)

```

## Diagnostics

- **engine**: CV=25.7% (high variance, measurements may be unstable)
- **engine**: bridge=105.8% of algo (FFI overhead may distort results)
- **engine_par**: bridge=32.4% of algo (FFI overhead may distort results)
- **std**: CV=29.4% (high variance, measurements may be unstable)
- **std**: bridge=874.9% of algo (FFI overhead may distort results)
- **std_par**: bridge=38.2% of algo (FFI overhead may distort results)
