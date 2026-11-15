# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_snap_twin shows alternating (throttle bounce) (autocorr -0.58)

cs_snap_twin's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (cs_plain)

The baseline cs_plain is the fastest (16.47 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {cs_plain, cs_exch} vs {cs_snap, cs_snap_twin} (26% apart)

The field splits into a fast tier {cs_plain, cs_exch} and a slow tier {cs_snap, cs_snap_twin} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (cs_plain) is the fastest** at 16474.6 ns median
- 3 variants significantly slower than baseline
- Spread: 1.37x (fastest 16474.6 ns, slowest 22612.5 ns)

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
| Counter frequency | 2112762221 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cs_exch | 19050ns | 18877ns | 18426ns | 18822ns | 19705ns | +4.42% |
| cs_plain | 18243ns | 18107ns | 17769ns | 18118ns | 18668ns | base |
| cs_snap | 23598ns | 23417ns | 22946ns | 23286ns | 24393ns | +29.35% |
| cs_snap_twin | 24135ns | 24243ns | 23648ns | 24071ns | 24473ns | +32.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 17418ns | 16811ns | 18071ns | +5.13% | 0.004 |
| cs_plain | 16569ns | 16129ns | 16939ns | base | 0.004 |
| cs_snap | 21949ns | 21276ns | 22748ns | +32.47% | 0.003 |
| cs_snap_twin | 22482ns | 21980ns | 22807ns | +35.69% | 0.003 |

## Performance model

- Peak throughput: **0.004 Gops/s** (cs_plain; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.004 | 93.5% |
| cs_plain | 0.004 | 97.9% |
| cs_snap | 0.003 | 74.1% |
| cs_snap_twin | 0.003 | 71.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 19050ns | 19050ns | +4.42% |
| cs_plain | 18243ns | 18243ns | base |
| cs_snap | 23598ns | 23598ns | +29.35% |
| cs_snap_twin | 24135ns | 24135ns | +32.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 16475ns | base | --- | [16379, 16751] | --- | --- | --- | --- |
| cs_exch | 17251ns | +882.1ns (+5.4%) | [+494, +1221]ns | [16974, 17998] | YES | 0.0013 | 0.0013 | 0 |
| cs_snap | 21769ns | +5219.2ns (+31.7%) | [+4980, +5518]ns | [21366, 21943] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 22613ns | +5876.5ns (+35.7%) | [+5667, +6099]ns | [22124, 22746] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 16173ns | +6.8% | +35.7% | +40.4% |
| 1 | 2 | 16368ns | +3.6% | +30.2% | +34.8% |
| 1 | 3 | 16611ns | +1.6% | +31.5% | +37.2% |
| 2 | 1 | 16431ns | +4.9% | +32.2% | +37.6% |
| 2 | 2 | 16390ns | +9.9% | +30.2% | +34.5% |
| 2 | 3 | 16761ns | +8.3% | +40.7% | +35.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | 0.511 | HIGH+ (drift/warm-up) |
| cs_plain | -0.053 | ok |
| cs_snap | -0.210 | moderate- |
| cs_snap_twin | -0.583 | HIGH- (thermal bounce) |

**Consistency summary:**

- **cs_exch**: won 0/6, lost 6/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 239628.7ns | 17418.3ns | 1375.7% | HIGH |
| cs_plain | 239367.1ns | 16568.6ns | 1444.7% | HIGH |
| cs_snap | 239772.1ns | 21949.2ns | 1092.4% | HIGH |
| cs_snap_twin | 239802.3ns | 22481.8ns | 1066.7% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 16810.6-18070.9 ns)
  16810.6 |
  16873.6 |########################################
  16936.6 |########################################
  16999.6 |
  17062.6 |
  17125.6 |####################
  17188.7 |########################################
  17251.7 |####################
  17314.7 |
  17377.7 |
  17440.7 |########################################
  17503.7 |
  17566.7 |
  17629.8 |
  17692.8 |
  17755.8 |
  17818.8 |
  17881.8 |
  17944.8 |####################
  18007.8 |########################################
  (2 below, 3 above range)

cs_plain (n=18, range 16128.7-16939.4 ns)
  16128.7 |####################
  16169.2 |####################
  16209.8 |
  16250.3 |
  16290.8 |
  16331.4 |########################################
  16371.9 |########################################
  16412.4 |########################################
  16453.0 |
  16493.5 |####################
  16534.0 |
  16574.6 |####################
  16615.1 |
  16655.6 |
  16696.2 |####################
  16736.7 |########################################
  16777.2 |
  16817.8 |
  16858.3 |####################
  16898.8 |####################
  (1 below, 2 above range)

cs_snap (n=18, range 21275.8-22748.3 ns)
  21275.8 |########################################
  21349.5 |##########################
  21423.1 |
  21496.7 |
  21570.3 |
  21644.0 |#############
  21717.6 |##########################
  21791.2 |##########################
  21864.8 |#############
  21938.5 |##########################
  22012.1 |#############
  22085.7 |
  22159.3 |
  22232.9 |
  22306.6 |
  22380.2 |
  22453.8 |
  22527.4 |
  22601.1 |
  22674.7 |
  (1 below, 3 above range)

cs_snap_twin (n=18, range 21979.5-22807.2 ns)
  21979.5 |
  22020.9 |#############
  22062.3 |##########################
  22103.7 |
  22145.1 |#############
  22186.5 |
  22227.8 |
  22269.2 |
  22310.6 |
  22352.0 |
  22393.4 |
  22434.8 |#############
  22476.1 |
  22517.5 |
  22558.9 |#############
  22600.3 |########################################
  22641.7 |
  22683.0 |#############
  22724.4 |##########################
  22765.8 |##########################
  (2 below, 2 above range)

```

## Diagnostics

- **cs_exch**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **cs_exch**: bridge=1386.4% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=1451.5% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=1100.0% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=1058.4% of algo (FFI overhead may distort results)
