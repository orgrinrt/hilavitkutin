# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_plain dominates: 11% faster than the next best (cs_exch)

cs_plain (22.78 us) leads cs_exch (25.26 us) by 11%, a clear separation rather than a photo finish. CV 6.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### cs_snap is an outlier: 2.2x slower than the field

cs_snap (49.87 us) is 2.2x the fastest (22.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (cs_plain)

The baseline cs_plain is the fastest (22.78 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {cs_plain, cs_exch, cs_snap_twin} vs {cs_snap} (66% apart)

The field splits into a fast tier {cs_plain, cs_exch, cs_snap_twin} and a slow tier {cs_snap} with a 66% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### cs_snap_twin is inconsistent: worst-20% is 2.3x its best-20%

cs_snap_twin's best 20% of batches run at 24.42 us but its worst 20% at 56.70 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (cs_plain) is the fastest** at 22779.3 ns median
- 2 variants significantly slower than baseline
- Spread: 2.19x (fastest 22779.3 ns, slowest 49874.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | a buffered column's swap: plain floor vs Snapshot copy vs Exchange |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000004 |
| Counter frequency | 2100370818 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cs_exch | 31239ns | 28556ns | 20893ns | 27233ns | 42422ns | +24.28% |
| cs_plain | 25136ns | 25026ns | 22765ns | 24818ns | 26799ns | base |
| cs_snap | 51806ns | 53253ns | 28993ns | 49742ns | 66309ns | +106.10% |
| cs_snap_twin | 40117ns | 32622ns | 26378ns | 31626ns | 59722ns | +59.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 28567ns | 19002ns | 39344ns | +24.76% | 0.002 |
| cs_plain | 22898ns | 20758ns | 24436ns | base | 0.003 |
| cs_snap | 48212ns | 26773ns | 61600ns | +110.56% | 0.001 |
| cs_snap_twin | 37494ns | 24419ns | 56697ns | +63.75% | 0.002 |

## Performance model

- Peak throughput: **0.003 Gops/s** (cs_exch; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.003 | 75.2% |
| cs_plain | 0.003 | 83.4% |
| cs_snap | 0.001 | 38.1% |
| cs_snap_twin | 0.002 | 63.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 31239ns | 31239ns | +24.28% |
| cs_plain | 25136ns | 25136ns | base |
| cs_snap | 51806ns | 51806ns | +106.10% |
| cs_snap_twin | 40117ns | 40117ns | +59.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 22779ns | base | --- | [22090, 23586] | --- | --- | --- | --- |
| cs_exch | 25263ns | no significant difference | [-286, +6886]ns | [22868, 29430] | no | 0.2379 | 0.2379 | 0 |
| cs_snap | 49874ns | +27709.3ns (+121.6%) | [+19339, +31455]ns | [40801, 52634] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 30051ns | +6454.9ns (+28.3%) | [+4765, +16193]ns | [28029, 39449] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 23511ns | +10.6% | +193.3% | +29.0% |
| 1 | 2 | 21271ns | +8.2% | +135.8% | +153.9% |
| 1 | 3 | 21836ns | +135.0% | +80.5% | +180.7% |
| 2 | 1 | 23968ns | -20.9% | +15.7% | +17.1% |
| 2 | 2 | 23596ns | +3.4% | +141.5% | +27.0% |
| 2 | 3 | 22237ns | +26.2% | +128.2% | +18.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.434 | moderate- |
| cs_plain | -0.049 | ok |
| cs_snap | 0.059 | ok |
| cs_snap_twin | 0.159 | ok |

**Consistency summary:**

- **cs_exch**: won 1/6, lost 5/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 312602.0ns | 28566.7ns | 1094.3% | HIGH |
| cs_plain | 309033.4ns | 22897.6ns | 1349.6% | HIGH |
| cs_snap | 324571.0ns | 48212.1ns | 673.2% | HIGH |
| cs_snap_twin | 296714.1ns | 37494.2ns | 791.4% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 19002.1-39344.4 ns)
  19002.1 |#############
  20019.2 |
  21036.4 |
  22053.5 |########################################
  23070.6 |#############
  24087.7 |##########################
  25104.8 |#############
  26121.9 |#############
  27139.1 |#############
  28156.2 |
  29173.3 |##########################
  30190.4 |
  31207.5 |
  32224.6 |
  33241.7 |
  34258.9 |#############
  35276.0 |
  36293.1 |
  37310.2 |#############
  38327.3 |
  (2 below, 2 above range)

cs_plain (n=18, range 20758.1-24436.4 ns)
  20758.1 |
  20942.0 |
  21125.9 |####################
  21309.8 |
  21493.7 |
  21677.6 |########################################
  21861.6 |####################
  22045.5 |####################
  22229.4 |####################
  22413.3 |####################
  22597.2 |####################
  22781.1 |####################
  22965.0 |
  23149.0 |
  23332.9 |########################################
  23516.8 |########################################
  23700.7 |
  23884.6 |########################################
  24068.5 |
  24252.4 |
  (1 below, 2 above range)

cs_snap (n=18, range 26773.4-61599.9 ns)
  26773.4 |####################
  28514.8 |
  30256.1 |
  31997.4 |
  33738.7 |
  35480.0 |
  37221.4 |##########
  38962.7 |##########
  40704.0 |##########
  42445.3 |
  44186.7 |
  45928.0 |
  47669.3 |####################
  49410.6 |########################################
  51151.9 |##########
  52893.3 |##########
  54634.6 |
  56375.9 |##########
  58117.2 |
  59858.6 |
  (1 below, 3 above range)

cs_snap_twin (n=18, range 24418.7-56697.5 ns)
  24418.7 |####################
  26032.6 |##########
  27646.6 |########################################
  29260.5 |########################################
  30874.5 |
  32488.4 |##########
  34102.3 |
  35716.3 |
  37330.2 |
  38944.1 |
  40558.1 |
  42172.0 |
  43786.0 |##########
  45399.9 |
  47013.8 |
  48627.8 |
  50241.7 |
  51855.6 |
  53469.6 |##########
  55083.5 |
  (1 below, 3 above range)

```

## Diagnostics

- **cs_exch**: CV=34.2% (high variance, measurements may be unstable)
- **cs_exch**: bridge=1262.3% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=1360.1% of algo (FFI overhead may distort results)
- **cs_snap**: CV=27.1% (high variance, measurements may be unstable)
- **cs_snap**: bridge=672.2% of algo (FFI overhead may distort results)
- **cs_snap_twin**: CV=42.7% (high variance, measurements may be unstable)
- **cs_snap_twin**: bridge=1046.7% of algo (FFI overhead may distort results)
