# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_engine_par beats baseline by 56% (significant)

evs_engine_par is -9.03 ms (56%) faster than baseline evs_std, with a CI that excludes zero.


### evs_engine is an outlier: 2.3x slower than the field

evs_engine (16.50 ms) is 2.3x the fastest (7.23 ms), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_engine_par, evs_std_par} vs {evs_std, evs_engine} (120% apart)

The field splits into a fast tier {evs_engine_par, evs_std_par} and a slow tier {evs_std, evs_engine} with a 120% jump between them - a qualitative difference, not a gradient.


## Key findings

- Fastest: evs_engine_par at 7226901.2 ns median (-55.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.28x (fastest 7226901.2 ns, slowest 16504731.8 ns)

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
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 16755400ns | 16512413ns | 15764253ns | 16285011ns | 17355979ns | +3.85% |
| evs_engine_par | 7152519ns | 7231849ns | 6399358ns | 6596295ns | 7757978ns | -55.67% |
| evs_std | 16134686ns | 16176159ns | 15170833ns | 15633696ns | 16751391ns | base |
| evs_std_par | 7590703ns | 7343582ns | 6736331ns | 7149133ns | 8135474ns | -52.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 16750123ns | 15760495ns | 17351154ns | +3.85% | 0.063 |
| evs_engine_par | 7147515ns | 6395270ns | 7752750ns | -55.69% | 0.147 |
| evs_std | 16129299ns | 15166558ns | 16746547ns | base | 0.065 |
| evs_std_par | 7585413ns | 6731032ns | 8130262ns | -52.97% | 0.138 |

## Performance model

- Peak throughput: 0.164 Gops/s (evs_engine_par; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.064 | 38.7% |
| evs_engine_par | 0.145 | 88.5% |
| evs_std | 0.065 | 39.5% |
| evs_std_par | 0.143 | 87.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 16755400ns | 16755400ns | +3.85% |
| evs_engine_par | 7152519ns | 7152519ns | -55.67% |
| evs_std | 16134686ns | 16134686ns | base |
| evs_std_par | 7590703ns | 7590703ns | -52.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 16171958ns | base | --- | [15270144, 16447837] | --- | --- | --- | --- |
| evs_engine | 16504732ns | no significant difference | [-577257, +1350332]ns | [15870580, 17579877] | no | 0.7266 | 0.7266 | 0 |
| evs_engine_par | 7226901ns | -9031729.5ns (-55.8%) | [-9879032, -8460370]ns | [6428931, 7779969] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 7338188ns | -8546074.3ns (-52.8%) | [-9294026, -7729789]ns | [7077020, 7540355] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 16434416ns | +0.0% | -53.1% | -55.2% |
| 1 | 2 | 15475953ns | +6.6% | -57.2% | -53.1% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 1/2
- evs_engine_par: won 2/2, lost 0/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 23987185.7ns | 16750123.5ns | 143.2% | HIGH |
| evs_engine_par | 23267821.7ns | 7147515.2ns | 325.5% | HIGH |
| evs_std | 23691099.2ns | 16129298.6ns | 146.9% | HIGH |
| evs_std_par | 23457753.6ns | 7585412.6ns | 309.2% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 15760495.1-17351154.2 ns)
  15760495.1 |####################
  15840028.1 |####################
  15919561.0 |
  15999094.0 |
  16078626.9 |
  16158159.9 |
  16237692.8 |
  16317225.8 |
  16396758.7 |####################
  16476291.7 |########################################
  16555824.7 |
  16635357.6 |
  16714890.6 |
  16794423.5 |
  16873956.5 |
  16953489.4 |####################
  17033022.4 |
  17112555.4 |
  17192088.3 |
  17271621.3 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 6395269.8-7752750.0 ns)
  6395269.8 |########################################
  6463143.8 |
  6531017.8 |####################
  6598891.8 |
  6666765.8 |
  6734639.8 |
  6802513.9 |####################
  6870387.9 |
  6938261.9 |
  7006135.9 |
  7074009.9 |
  7141883.9 |
  7209757.9 |
  7277631.9 |
  7345505.9 |
  7413379.9 |
  7481253.9 |
  7549127.9 |
  7617002.0 |####################
  7684876.0 |
  (0 below, 3 above range)

evs_std (n=8, range 15166557.9-16746547.2 ns)
  15166557.9 |####################
  15245557.4 |####################
  15324556.8 |
  15403556.3 |
  15482555.8 |
  15561555.2 |
  15640554.7 |####################
  15719554.2 |
  15798553.6 |
  15877553.1 |####################
  15956552.6 |
  16035552.0 |
  16114551.5 |
  16193551.0 |
  16272550.4 |
  16351549.9 |########################################
  16430549.4 |####################
  16509548.8 |
  16588548.3 |
  16667547.8 |
  (0 below, 1 above range)

evs_std_par (n=8, range 6731032.0-8130262.0 ns)
  6731032.0 |####################
  6800993.5 |
  6870955.0 |
  6940916.5 |
  7010878.0 |####################
  7080839.5 |####################
  7150801.0 |
  7220762.5 |####################
  7290724.0 |
  7360685.5 |
  7430647.0 |########################################
  7500608.5 |####################
  7570570.0 |
  7640531.5 |
  7710493.0 |
  7780454.5 |
  7850416.0 |
  7920377.5 |
  7990339.0 |
  8060300.5 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=146.3% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=324.2% of algo (FFI overhead may distort results)
- evs_std: bridge=147.4% of algo (FFI overhead may distort results)
- evs_std_par: bridge=321.2% of algo (FFI overhead may distort results)
