# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par beats baseline by 42% (significant)

evs_std_par is -79.20 us (42%) faster than baseline evs_std, with a CI that excludes zero.


### evs_std_par is fastest but the noisiest (CV 6.3%)

evs_std_par wins on median (110.44 us) yet has the highest variance (CV 6.3%), while evs_std is the steadiest (CV 1.3%, 189.40 us).


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine_par} vs {evs_std, evs_engine} (64% apart)

The field splits into a fast tier {evs_std_par, evs_engine_par} and a slow tier {evs_std, evs_engine} with a 64% jump between them - a qualitative difference, not a gradient.


## Key findings

- Fastest: evs_std_par at 110437.5 ns median (-41.7% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.79x (fastest 110437.5 ns, slowest 197554.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | wide_parallel, four independent heavy chains |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000004 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 200738ns | 198125ns | 193967ns | 195756ns | 206167ns | +6.06% |
| evs_engine_par | 117456ns | 117112ns | 105825ns | 113689ns | 123190ns | -37.94% |
| evs_std | 189267ns | 189712ns | 183617ns | 188189ns | 191488ns | base |
| evs_std_par | 111961ns | 111733ns | 102592ns | 107069ns | 117973ns | -40.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 200202ns | 193867ns | 205502ns | +6.18% | 0.327 |
| evs_engine_par | 116554ns | 105392ns | 122275ns | -38.18% | 0.562 |
| evs_std | 188552ns | 183333ns | 190438ns | base | 0.348 |
| evs_std_par | 111138ns | 101500ns | 117192ns | -41.06% | 0.590 |

## Performance model

- Peak throughput: 0.646 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.332 | 51.4% |
| evs_engine_par | 0.566 | 87.6% |
| evs_std | 0.346 | 53.6% |
| evs_std_par | 0.593 | 91.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 200738ns | 200738ns | +6.06% |
| evs_engine_par | 117456ns | 117456ns | -37.94% |
| evs_std | 189267ns | 189267ns | base |
| evs_std_par | 111961ns | 111961ns | -40.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 189396ns | base | --- | [186242, 190975] | --- | --- | --- | --- |
| evs_engine | 197554ns | +7275.0ns (+3.8%) | [+4675, +21433]ns | [194375, 208767] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 115829ns | -72108.3ns (-38.1%) | [-79358, -64467]ns | [111083, 123000] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 110438ns | -79204.1ns (-41.8%) | [-84742, -68108]ns | [102983, 117600] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 189396ns | +2.9% | -41.1% | -43.0% |
| 1 | 2 | 188017ns | +10.0% | -34.6% | -37.8% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 2/2, lost 0/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 1507732.3ns | 200202.1ns | 753.1% | HIGH |
| evs_engine_par | 1447347.9ns | 116554.2ns | 1241.8% | HIGH |
| evs_std | 1451802.1ns | 188552.1ns | 770.0% | HIGH |
| evs_std_par | 1454484.4ns | 111137.5ns | 1308.7% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 193866.7-205502.1 ns)
  193866.7 |########################################
  194448.5 |
  195030.2 |####################
  195612.0 |####################
  196193.8 |
  196775.5 |
  197357.3 |
  197939.1 |
  198520.9 |####################
  199102.6 |
  199684.4 |
  200266.2 |
  200847.9 |
  201429.7 |
  202011.5 |
  202593.2 |
  203175.0 |
  203756.8 |
  204338.6 |####################
  204920.3 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 105391.7-122275.0 ns)
  105391.7 |########################################
  106235.9 |
  107080.0 |
  107924.2 |
  108768.4 |
  109612.5 |
  110456.7 |########################################
  111300.9 |########################################
  112145.0 |
  112989.2 |
  113833.4 |
  114677.5 |########################################
  115521.7 |
  116365.8 |########################################
  117210.0 |
  118054.2 |
  118898.3 |
  119742.5 |
  120586.7 |
  121430.8 |
  (0 below, 3 above range)

evs_std (n=8, range 183333.3-190437.5 ns)
  183333.3 |########################################
  183688.5 |
  184043.7 |
  184398.9 |
  184754.1 |
  185109.3 |
  185464.6 |
  185819.8 |
  186175.0 |########################################
  186530.2 |
  186885.4 |
  187240.6 |
  187595.8 |########################################
  187951.0 |
  188306.2 |
  188661.5 |
  189016.7 |########################################
  189371.9 |########################################
  189727.1 |########################################
  190082.3 |
  (0 below, 2 above range)

evs_std_par (n=8, range 101500.0-117191.7 ns)
  101500.0 |########################################
  102284.6 |########################################
  103069.2 |
  103853.8 |
  104638.3 |
  105422.9 |
  106207.5 |
  106992.1 |########################################
  107776.7 |
  108561.3 |########################################
  109345.8 |
  110130.4 |
  110915.0 |
  111699.6 |########################################
  112484.2 |
  113268.8 |
  114053.3 |
  114837.9 |
  115622.5 |########################################
  116407.1 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=724.4% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=1249.7% of algo (FFI overhead may distort results)
- evs_std: bridge=761.6% of algo (FFI overhead may distort results)
- evs_std_par: bridge=1317.6% of algo (FFI overhead may distort results)
