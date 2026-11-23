# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par dominates: 17% faster than the next best (evs_std)

evs_std_par (356.93 us) leads evs_std (419.38 us) by 17%, a clear separation rather than a photo finish. CV 5.5%.


### evs_engine is an outlier: 2.4x slower than the field

evs_engine (851.27 us) is 2.4x the fastest (356.93 us), well off the pack.


### evs_engine shows alternating (throttle bounce) (autocorr -0.50)

evs_engine's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_std, evs_engine_par} vs {evs_engine} (64% apart)

The field splits into a fast tier {evs_std_par, evs_std, evs_engine_par} and a slow tier {evs_engine} with a 64% jump between them - a qualitative difference, not a gradient.


## Key findings

- Fastest: evs_std_par at 356929.2 ns median (-14.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.38x (fastest 356929.2 ns, slowest 851267.4 ns)

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
| Counter frequency | 2112831643 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 855307ns | 851790ns | 833956ns | 835925ns | 875182ns | +105.43% |
| evs_engine_par | 525756ns | 519872ns | 452329ns | 488673ns | 571926ns | +26.28% |
| evs_std | 416342ns | 420033ns | 391404ns | 402452ns | 432993ns | base |
| evs_std_par | 361287ns | 358304ns | 331164ns | 350998ns | 376534ns | -13.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 854830ns | 833325ns | 874723ns | +105.60% | 1.227 |
| evs_engine_par | 524422ns | 451315ns | 570557ns | +26.13% | 1.999 |
| evs_std | 415776ns | 390944ns | 432404ns | base | 2.522 |
| evs_std_par | 359830ns | 329681ns | 374966ns | -13.46% | 2.914 |

## Performance model

- Peak throughput: 3.181 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.232 | 38.7% |
| evs_engine_par | 2.022 | 63.6% |
| evs_std | 2.500 | 78.6% |
| evs_std_par | 2.938 | 92.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 855307ns | 855307ns | +105.43% |
| evs_engine_par | 525756ns | 525756ns | +26.28% |
| evs_std | 416342ns | 416342ns | base |
| evs_std_par | 361287ns | 361287ns | -13.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 419375ns | base | --- | [391511, 424722] | --- | --- | --- | --- |
| evs_engine | 851267ns | +414604.4ns (+98.9%) | [+411118, +486777]ns | [834481, 878288] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 518651ns | +90971.1ns (+21.7%) | [+50416, +202321]ns | [468978, 593264] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 356929ns | -51974.0ns (-12.4%) | [-68673, -45037]ns | [345574, 376547] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 420922ns | +98.4% | +12.4% | -15.0% |
| 1 | 2 | 393544ns | +122.7% | +41.7% | -9.6% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | HIGH- (thermal bounce) |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 0/2, lost 2/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 4086099.4ns | 854830.2ns | 478.0% | HIGH |
| evs_engine_par | 4194796.0ns | 524422.0ns | 799.9% | HIGH |
| evs_std | 4102182.5ns | 415776.2ns | 986.6% | HIGH |
| evs_std_par | 4135702.2ns | 359830.2ns | 1149.3% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 833324.6-874723.0 ns)
  833324.6 |########################################
  835394.5 |########################################
  837464.4 |
  839534.4 |
  841604.3 |
  843674.2 |
  845744.1 |
  847814.0 |
  849883.9 |
  851953.9 |
  854023.8 |
  856093.7 |
  858163.6 |
  860233.5 |
  862303.5 |
  864373.4 |####################
  866443.3 |
  868513.2 |
  870583.1 |
  872653.1 |
  (0 below, 3 above range)

evs_engine_par (n=8, range 451314.6-570556.7 ns)
  451314.6 |########################################
  457276.7 |
  463238.8 |########################################
  469200.9 |
  475163.0 |########################################
  481125.1 |
  487087.2 |
  493049.3 |
  499011.4 |
  504973.6 |
  510935.7 |########################################
  516897.8 |########################################
  522859.9 |
  528822.0 |
  534784.1 |
  540746.2 |
  546708.3 |
  552670.4 |
  558632.5 |
  564594.6 |########################################
  (0 below, 2 above range)

evs_std (n=8, range 390943.7-432403.8 ns)
  390943.7 |########################################
  393016.7 |
  395089.7 |####################
  397162.7 |
  399235.7 |
  401308.7 |
  403381.7 |
  405454.7 |
  407527.7 |
  409600.7 |
  411673.7 |
  413746.7 |
  415819.7 |
  417892.7 |####################
  419965.7 |########################################
  422038.7 |
  424111.7 |####################
  426184.7 |
  428257.7 |
  430330.7 |
  (0 below, 1 above range)

evs_std_par (n=8, range 329681.2-374965.6 ns)
  329681.2 |########################################
  331945.4 |
  334209.6 |
  336473.9 |
  338738.1 |
  341002.3 |
  343266.5 |
  345530.7 |########################################
  347795.0 |
  350059.2 |########################################
  352323.4 |########################################
  354587.6 |
  356851.8 |
  359116.1 |########################################
  361380.3 |########################################
  363644.5 |
  365908.7 |
  368172.9 |
  370437.2 |
  372701.4 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=480.3% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=807.7% of algo (FFI overhead may distort results)
- evs_std: bridge=977.0% of algo (FFI overhead may distort results)
- evs_std_par: bridge=1159.1% of algo (FFI overhead may distort results)
