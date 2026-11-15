# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Key findings

- **Baseline (sb_none) is the fastest** at 10895.9 ns median
- 2 variants significantly slower than baseline
- Spread: 238.71x (fastest 10895.9 ns, slowest 2600983.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sb_none | 17022ns | 16885ns | 16354ns | 16804ns | 17683ns | base |
| sb_plan | 20493ns | 20408ns | 19483ns | 20271ns | 21331ns | +20.39% |
| sb_value | 2656854ns | 2606554ns | 2526847ns | 2597099ns | 2811489ns | +15508.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 10933ns | 10539ns | 11242ns | base | 0.749 |
| sb_plan | 14362ns | 13818ns | 14696ns | +31.37% | 0.570 |
| sb_value | 2650918ns | 2520807ns | 2805291ns | +24147.73% | 0.003 |

## Performance model

- Peak throughput: **0.777 Gops/s** (sb_none; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.752 | 96.7% |
| sb_plan | 0.571 | 73.5% |
| sb_value | 0.003 | 0.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 17022ns | 17022ns | base |
| sb_plan | 20493ns | 20493ns | +20.39% |
| sb_value | 2656854ns | 2656854ns | +15508.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 10896ns | base | --- | [10725, 11131] | --- | --- | --- | --- |
| sb_plan | 14335ns | +3495.9ns (+32.1%) | [+3329, +3688]ns | [14258, 14465] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 2600983ns | +2589983.4ns (+23770.4%) | [+2550625, +2677292]ns | [2562717, 2688138] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|
| 1 | 10900ns | +30.8% | +23151.5% |
| 2 | 10875ns | +31.9% | +23465.2% |
| 3 | 10854ns | +29.4% | +23935.6% |
| 4 | 11146ns | +31.6% | +23165.3% |
| 5 | 11388ns | +29.3% | +21949.5% |
| 6 | 11050ns | +33.1% | +24372.2% |
| 7 | 10692ns | +33.4% | +23634.6% |
| 8 | 10642ns | +35.6% | +25009.7% |
| 9 | 11008ns | +26.1% | +23159.0% |
| 10 | 10892ns | +31.3% | +23547.8% |
| 11 | 10725ns | +33.6% | +24593.5% |
| 12 | 10554ns | +37.3% | +23749.5% |
| 13 | 11117ns | +29.8% | +23035.2% |
| 14 | 11229ns | +27.6% | +26313.7% |
| 15 | 10721ns | +34.7% | +24746.4% |
| 16 | 11292ns | +34.1% | +24515.6% |
| 17 | 11283ns | +19.9% | +25057.0% |
| 18 | 10421ns | +36.3% | +27454.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | 0.028 | ok |
| sb_plan | -0.172 | ok |
| sb_value | 0.236 | moderate+ |

**Consistency summary:**

- **sb_plan**: won 0/18, lost 18/18
- **sb_value**: won 0/18, lost 18/18

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 45111072.9ns | 10932.6ns | 412627.3% | HIGH |
| sb_plan | 45125010.4ns | 14362.0ns | 314196.3% | HIGH |
| sb_value | 45123146.8ns | 2650918.1ns | 1702.2% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 10538.9-11242.4 ns)
  10538.9 |####################
  10574.1 |
  10609.2 |####################
  10644.4 |
  10679.6 |####################
  10714.8 |########################################
  10749.9 |
  10785.1 |
  10820.3 |####################
  10855.5 |####################
  10890.6 |########################################
  10925.8 |
  10961.0 |
  10996.2 |####################
  11031.3 |####################
  11066.5 |
  11101.7 |####################
  11136.8 |####################
  11172.0 |
  11207.2 |####################
  (1 below, 3 above range)

sb_plan (n=18, range 13818.1-14695.8 ns)
  13818.1 |
  13862.0 |##########
  13905.8 |
  13949.7 |
  13993.6 |
  14037.5 |##########
  14081.4 |
  14125.3 |
  14169.2 |##########
  14213.1 |
  14257.0 |####################
  14300.8 |########################################
  14344.7 |
  14388.6 |
  14432.5 |##############################
  14476.4 |##########
  14520.3 |
  14564.2 |
  14608.1 |
  14652.0 |##########
  (1 below, 3 above range)

sb_value (n=18, range 2520806.9-2805291.0 ns)
  2520806.9 |####################
  2535031.1 |####################
  2549255.3 |########################################
  2563479.5 |########################################
  2577703.7 |
  2591927.9 |####################
  2606152.1 |####################
  2620376.3 |
  2634600.5 |####################
  2648824.7 |
  2663049.0 |########################################
  2677273.2 |
  2691497.4 |####################
  2705721.6 |
  2719945.8 |
  2734170.0 |
  2748394.2 |
  2762618.4 |
  2776842.6 |####################
  2791066.8 |
  (2 below, 3 above range)

```

## Diagnostics

- **sb_none**: bridge=413474.4% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=314396.4% of algo (FFI overhead may distort results)
- **sb_value**: bridge=1734.2% of algo (FFI overhead may distort results)
