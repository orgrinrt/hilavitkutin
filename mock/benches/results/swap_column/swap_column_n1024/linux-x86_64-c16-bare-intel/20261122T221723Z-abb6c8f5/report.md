# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_plain dominates: 83% faster than the next best (cs_exch)

cs_plain (375.90 us) leads cs_exch (689.72 us) by 83%, a clear separation rather than a photo finish. CV 10.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### cs_snap is an outlier: 2.6x slower than the field

cs_snap (981.97 us) is 2.6x the fastest (375.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### cs_plain is fastest but the noisiest (CV 10.8%)

cs_plain wins on median (375.90 us) yet has the highest variance (CV 10.8%), while cs_snap is the steadiest (CV 3.1%, 981.97 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### cs_snap_twin shows alternating (throttle bounce) (autocorr -0.61)

cs_snap_twin's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (cs_plain)

The baseline cs_plain is the fastest (375.90 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {cs_plain} vs {cs_exch, cs_snap_twin, cs_snap} (83% apart)

The field splits into a fast tier {cs_plain} and a slow tier {cs_exch, cs_snap_twin, cs_snap} with a 83% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (cs_plain) is the fastest** at 375904.3 ns median
- 3 variants significantly slower than baseline
- Spread: 2.61x (fastest 375904.3 ns, slowest 981973.1 ns)

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
| Counter frequency | 2112756002 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cs_exch | 697736ns | 692744ns | 641674ns | 681740ns | 749761ns | +76.97% |
| cs_plain | 394258ns | 378512ns | 355442ns | 372801ns | 445851ns | base |
| cs_snap | 993365ns | 985463ns | 957870ns | 981111ns | 1029495ns | +151.96% |
| cs_snap_twin | 910351ns | 902917ns | 815657ns | 886273ns | 993815ns | +130.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 694829ns | 638836ns | 746822ns | +77.43% | 0.001 |
| cs_plain | 391617ns | 352726ns | 443164ns | base | 0.003 |
| cs_snap | 989791ns | 954233ns | 1025954ns | +152.74% | 0.001 |
| cs_snap_twin | 906972ns | 812086ns | 990297ns | +131.60% | 0.001 |

## Performance model

- Peak throughput: **0.003 Gops/s** (cs_plain; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.001 | 51.1% |
| cs_plain | 0.003 | 93.8% |
| cs_snap | 0.001 | 35.9% |
| cs_snap_twin | 0.001 | 39.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 697736ns | 697736ns | +76.97% |
| cs_plain | 394258ns | 394258ns | base |
| cs_snap | 993365ns | 993365ns | +151.96% |
| cs_snap_twin | 910351ns | 910351ns | +130.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 375904ns | base | --- | [360743, 415923] | --- | --- | --- | --- |
| cs_exch | 689717ns | +298640.3ns (+79.4%) | [+282088, +329686]ns | [664078, 701345] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap | 981973ns | +604792.5ns (+160.9%) | [+575932, +616678]ns | [967668, 1009629] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 899771ns | +495796.2ns (+131.9%) | [+447945, +570081]ns | [853629, 957778] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 470468ns | +67.2% | +118.1% | +104.6% |
| 1 | 2 | 353976ns | +93.2% | +170.0% | +157.6% |
| 1 | 3 | 360602ns | +78.1% | +179.7% | +169.8% |
| 2 | 1 | 382757ns | +73.2% | +162.7% | +111.9% |
| 2 | 2 | 365607ns | +91.3% | +168.3% | +160.6% |
| 2 | 3 | 417216ns | +68.2% | +132.4% | +109.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | 0.101 | ok |
| cs_plain | -0.194 | ok |
| cs_snap | -0.431 | moderate- |
| cs_snap_twin | -0.606 | HIGH- (thermal bounce) |

**Consistency summary:**

- **cs_exch**: won 0/6, lost 6/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 3854900.1ns | 694829.5ns | 554.8% | HIGH |
| cs_plain | 3840426.8ns | 391616.8ns | 980.7% | HIGH |
| cs_snap | 3976670.4ns | 989790.5ns | 401.8% | HIGH |
| cs_snap_twin | 3973370.7ns | 906971.7ns | 438.1% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 638836.3-746821.9 ns)
  638836.3 |##############################
  644235.6 |
  649634.9 |
  655034.1 |
  660433.4 |####################
  665832.7 |
  671232.0 |
  676631.3 |
  682030.5 |####################
  687429.8 |##############################
  692829.1 |
  698228.4 |########################################
  703627.7 |
  709026.9 |
  714426.2 |
  719825.5 |
  725224.8 |
  730624.1 |
  736023.3 |
  741422.6 |
  (1 below, 3 above range)

cs_plain (n=18, range 352725.6-443164.1 ns)
  352725.6 |########################################
  357247.6 |##########################
  361769.5 |##########################
  366291.4 |
  370813.3 |#############
  375335.3 |
  379857.2 |##########################
  384379.1 |#############
  388901.0 |
  393423.0 |
  397944.9 |
  402466.8 |
  406988.7 |
  411510.6 |#############
  416032.6 |##########################
  420554.5 |
  425076.4 |
  429598.3 |
  434120.3 |
  438642.2 |
  (1 below, 3 above range)

cs_snap (n=18, range 954233.1-1025954.5 ns)
  954233.1 |########################################
  957819.2 |
  961405.3 |####################
  964991.3 |####################
  968577.4 |####################
  972163.5 |########################################
  975749.5 |
  979335.6 |####################
  982921.7 |########################################
  986507.7 |
  990093.8 |
  993679.9 |
  997265.9 |
  1000852.0 |
  1004438.1 |####################
  1008024.1 |########################################
  1011610.2 |
  1015196.3 |####################
  1018782.3 |####################
  1022368.4 |
  (1 below, 2 above range)

cs_snap_twin (n=18, range 812085.5-990297.1 ns)
  812085.5 |####################
  820996.1 |####################
  829906.7 |
  838817.2 |####################
  847727.8 |
  856638.4 |
  865549.0 |####################
  874459.6 |########################################
  883370.2 |####################
  892280.7 |
  901191.3 |
  910101.9 |########################################
  919012.5 |
  927923.1 |
  936833.6 |
  945744.2 |########################################
  954654.8 |####################
  963565.4 |####################
  972476.0 |####################
  981386.6 |####################
  (2 below, 1 above range)

```

## Diagnostics

- **cs_exch**: bridge=559.3% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=1020.6% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=398.8% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=433.3% of algo (FFI overhead may distort results)
