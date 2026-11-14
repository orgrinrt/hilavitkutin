# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_plain dominates: 14% faster than the next best (cs_exch)

cs_plain (10.16 ms) leads cs_exch (11.54 ms) by 14%, a clear separation rather than a photo finish. CV 5.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (cs_plain)

The baseline cs_plain is the fastest (10.16 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {cs_plain, cs_exch} vs {cs_snap_twin, cs_snap} (35% apart)

The field splits into a fast tier {cs_plain, cs_exch} and a slow tier {cs_snap_twin, cs_snap} with a 35% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (cs_plain) is the fastest** at 10160316.4 ns median
- 3 variants significantly slower than baseline
- Spread: 1.60x (fastest 10160316.4 ns, slowest 16219785.4 ns)

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
| cs_exch | 11499610ns | 11547592ns | 10704788ns | 11272414ns | 12237814ns | +12.07% |
| cs_plain | 10261541ns | 10169122ns | 9570841ns | 10071955ns | 10891269ns | base |
| cs_snap | 16030503ns | 16228382ns | 14336826ns | 15965079ns | 16975476ns | +56.22% |
| cs_snap_twin | 15673634ns | 15613386ns | 13485460ns | 15327118ns | 17287495ns | +52.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 11490749ns | 10696789ns | 12227675ns | +12.08% | 0.001 |
| cs_plain | 10252483ns | 9562156ns | 10881403ns | base | 0.001 |
| cs_snap | 16018415ns | 14329126ns | 16955939ns | +56.24% | 0.001 |
| cs_snap_twin | 15664945ns | 13477718ns | 17278747ns | +52.79% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (cs_plain; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.001 | 82.9% |
| cs_plain | 0.001 | 94.1% |
| cs_snap | 0.001 | 59.0% |
| cs_snap_twin | 0.001 | 61.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 11499610ns | 11499610ns | +12.07% |
| cs_plain | 10261541ns | 10261541ns | base |
| cs_snap | 16030503ns | 16030503ns | +56.22% |
| cs_snap_twin | 15673634ns | 15673634ns | +52.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 10160316ns | base | --- | [9893878, 10385698] | --- | --- | --- | --- |
| cs_exch | 11538794ns | +1268633.8ns (+12.5%) | [+760570, +1796229]ns | [10923919, 11698775] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap | 16219785ns | +5763980.1ns (+56.7%) | [+5160116, +6383294]ns | [15543544, 16529067] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 15605200ns | +5228395.6ns (+51.5%) | [+4694427, +6023217]ns | [14610512, 16465263] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 10147576ns | +6.3% | +64.9% | +72.4% |
| 1 | 2 | 10184665ns | +8.6% | +62.7% | +50.7% |
| 1 | 3 | 11201935ns | +7.8% | +44.2% | +47.5% |
| 2 | 1 | 10149891ns | +7.7% | +62.4% | +52.1% |
| 2 | 2 | 9650660ns | +20.0% | +45.7% | +53.2% |
| 2 | 3 | 9746635ns | +18.0% | +55.8% | +47.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.348 | moderate- |
| cs_plain | 0.145 | ok |
| cs_snap | 0.201 | moderate+ |
| cs_snap_twin | 0.040 | ok |

**Consistency summary:**

- **cs_exch**: won 0/6, lost 6/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 38582683.7ns | 11490749.4ns | 335.8% | HIGH |
| cs_plain | 41767920.0ns | 10252482.8ns | 407.4% | HIGH |
| cs_snap | 42150193.6ns | 16018414.7ns | 263.1% | HIGH |
| cs_snap_twin | 41351660.6ns | 15664944.8ns | 264.0% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 10696788.7-12227674.9 ns)
  10696788.7 |########################################
  10773333.0 |####################
  10849877.4 |####################
  10926421.7 |####################
  11002966.0 |####################
  11079510.3 |
  11156054.6 |
  11232598.9 |
  11309143.2 |####################
  11385687.5 |
  11462231.8 |####################
  11538776.1 |########################################
  11615320.5 |########################################
  11691864.8 |####################
  11768409.1 |
  11844953.4 |
  11921497.7 |
  11998042.0 |
  12074586.3 |####################
  12151130.6 |####################
  (1 below, 2 above range)

cs_plain (n=18, range 9562155.6-10881403.4 ns)
  9562155.6 |
  9628118.0 |##########################
  9694080.4 |#############
  9760042.8 |#############
  9826005.2 |
  9891967.6 |
  9957929.9 |#############
  10023892.3 |
  10089854.7 |########################################
  10155817.1 |##########################
  10221779.5 |#############
  10287741.9 |#############
  10353704.3 |
  10419666.7 |#############
  10485629.1 |
  10551591.5 |
  10617553.9 |
  10683516.2 |
  10749478.6 |
  10815441.0 |#############
  (1 below, 3 above range)

cs_snap (n=18, range 14329126.0-16955939.3 ns)
  14329126.0 |
  14460466.6 |
  14591807.3 |
  14723148.0 |
  14854488.6 |
  14985829.3 |########################################
  15117170.0 |####################
  15248510.6 |
  15379851.3 |
  15511192.0 |
  15642532.7 |
  15773873.3 |####################
  15905214.0 |####################
  16036554.7 |########################################
  16167895.3 |####################
  16299236.0 |####################
  16430576.7 |########################################
  16561917.3 |####################
  16693258.0 |########################################
  16824598.7 |
  (2 below, 2 above range)

cs_snap_twin (n=18, range 13477718.0-17278746.9 ns)
  13477718.0 |
  13667769.5 |
  13857820.9 |####################
  14047872.4 |
  14237923.8 |####################
  14427975.2 |####################
  14618026.7 |####################
  14808078.1 |
  14998129.6 |
  15188181.0 |####################
  15378232.5 |########################################
  15568283.9 |
  15758335.3 |########################################
  15948386.8 |
  16138438.2 |
  16328489.7 |########################################
  16518541.1 |####################
  16708592.6 |
  16898644.0 |
  17088695.4 |
  (2 below, 4 above range)

```

## Diagnostics

- **cs_exch**: bridge=332.4% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=416.3% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=262.8% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=270.8% of algo (FFI overhead may distort results)
