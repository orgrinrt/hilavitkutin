# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_plain dominates: 57% faster than the next best (cs_exch)

cs_plain (451.50 us) leads cs_exch (707.51 us) by 57%, a clear separation rather than a photo finish. CV 19.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### cs_snap_twin is an outlier: 2.3x slower than the field

cs_snap_twin (1.02 ms) is 2.3x the fastest (451.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### cs_plain is fastest but the noisiest (CV 19.3%)

cs_plain wins on median (451.50 us) yet has the highest variance (CV 19.3%), while cs_snap_twin is the steadiest (CV 7.7%, 1.02 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### cs_plain shows alternating (throttle bounce) (autocorr -0.79)

cs_plain's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (cs_plain)

The baseline cs_plain is the fastest (451.50 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### cs_plain is inconsistent: worst-20% is 1.6x its best-20%

cs_plain's best 20% of batches run at 338.85 us but its worst 20% at 553.01 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (cs_plain) is the fastest** at 451499.2 ns median
- 3 variants significantly slower than baseline
- Spread: 2.26x (fastest 451499.2 ns, slowest 1019139.2 ns)

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
| cs_exch | 716485ns | 710455ns | 624022ns | 703724ns | 781857ns | +56.23% |
| cs_plain | 458620ns | 454090ns | 341599ns | 432972ns | 555601ns | base |
| cs_snap | 957944ns | 955510ns | 843813ns | 931942ns | 1054012ns | +108.88% |
| cs_snap_twin | 1003274ns | 1022202ns | 874515ns | 996333ns | 1078064ns | +118.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 713714ns | 621288ns | 779130ns | +56.50% | 0.001 |
| cs_plain | 456034ns | 338849ns | 553008ns | base | 0.002 |
| cs_snap | 954740ns | 840416ns | 1050727ns | +109.36% | 0.001 |
| cs_snap_twin | 999996ns | 871076ns | 1074738ns | +119.28% | 0.001 |

## Performance model

- Peak throughput: **0.003 Gops/s** (cs_plain; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.001 | 47.9% |
| cs_plain | 0.002 | 75.0% |
| cs_snap | 0.001 | 35.6% |
| cs_snap_twin | 0.001 | 33.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 716485ns | 716485ns | +56.23% |
| cs_plain | 458620ns | 458620ns | base |
| cs_snap | 957944ns | 957944ns | +108.88% |
| cs_snap_twin | 1003274ns | 1003274ns | +118.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 451499ns | base | --- | [385021, 509682] | --- | --- | --- | --- |
| cs_exch | 707506ns | +257221.4ns (+57.0%) | [+222668, +292695]ns | [696064, 730267] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap | 952525ns | +499698.1ns (+110.7%) | [+385634, +562064]ns | [894872, 982538] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 1019139ns | +588083.8ns (+130.3%) | [+441159, +612804]ns | [985793, 1034588] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 509008ns | +37.1% | +75.7% | +119.9% |
| 1 | 2 | 383380ns | +82.3% | +190.1% | +166.0% |
| 1 | 3 | 506417ns | +40.0% | +94.0% | +72.7% |
| 2 | 1 | 340135ns | +82.5% | +146.8% | +202.4% |
| 2 | 2 | 593683ns | +38.2% | +57.5% | +72.5% |
| 2 | 3 | 406351ns | +74.1% | +139.6% | +144.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.485 | moderate- |
| cs_plain | -0.795 | HIGH- (thermal bounce) |
| cs_snap | -0.151 | ok |
| cs_snap_twin | -0.088 | ok |

**Consistency summary:**

- **cs_exch**: won 0/6, lost 6/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 3845889.9ns | 713714.4ns | 538.9% | HIGH |
| cs_plain | 3849348.4ns | 456034.1ns | 844.1% | HIGH |
| cs_snap | 3856394.9ns | 954739.7ns | 403.9% | HIGH |
| cs_snap_twin | 3890544.2ns | 999996.1ns | 389.1% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 621288.2-779129.5 ns)
  621288.2 |##########
  629180.3 |
  637072.4 |
  644964.4 |
  652856.5 |
  660748.6 |
  668640.6 |
  676532.7 |
  684424.8 |##########
  692316.8 |########################################
  700208.9 |####################
  708100.9 |##############################
  715993.0 |
  723885.1 |
  731777.1 |
  739669.2 |##########
  747561.3 |##########
  755453.3 |
  763345.4 |
  771237.5 |
  (2 below, 3 above range)

cs_plain (n=18, range 338848.8-553007.6 ns)
  338848.8 |##########################
  349556.7 |
  360264.7 |
  370972.6 |#############
  381680.5 |##########################
  392388.5 |#############
  403096.4 |##########################
  413804.4 |
  424512.3 |
  435220.3 |
  445928.2 |
  456636.1 |
  467344.1 |
  478052.0 |
  488760.0 |#############
  499467.9 |########################################
  510175.9 |##########################
  520883.8 |
  531591.7 |
  542299.7 |
  (1 below, 3 above range)

cs_snap (n=18, range 840416.0-1050727.3 ns)
  840416.0 |#############
  850931.6 |
  861447.2 |
  871962.7 |
  882478.3 |#############
  892993.8 |##########################
  903509.4 |#############
  914025.0 |#############
  924540.5 |#############
  935056.1 |
  945571.7 |
  956087.2 |
  966602.8 |########################################
  977118.4 |##########################
  987633.9 |#############
  998149.5 |
  1008665.0 |
  1019180.6 |
  1029696.2 |
  1040211.7 |
  (2 below, 3 above range)

cs_snap_twin (n=18, range 871076.3-1074738.2 ns)
  871076.3 |########################################
  881259.4 |
  891442.5 |
  901625.5 |
  911808.6 |
  921991.7 |
  932174.8 |
  942357.9 |
  952541.0 |
  962724.1 |
  972907.2 |#############
  983090.3 |##########################
  993273.4 |
  1003456.5 |#############
  1013639.6 |##########################
  1023822.7 |########################################
  1034005.8 |#############
  1044188.9 |#############
  1054372.0 |
  1064555.1 |
  (1 below, 3 above range)

```

## Diagnostics

- **cs_exch**: bridge=543.4% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=850.9% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=402.4% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=381.7% of algo (FFI overhead may distort results)
