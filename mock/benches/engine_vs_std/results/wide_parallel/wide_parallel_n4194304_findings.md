# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std has the worst median (64.92 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_std_par at 21.11 ms).


### evs_std_par beats baseline by 66% (significant)

evs_std_par is -42.89 ms (66%) faster than baseline evs_std, with a CI that excludes zero.


### evs_std is an outlier: 3.1x slower than the field

evs_std (64.92 ms) is 3.1x the fastest (21.11 ms), well off the pack.


### evs_std_par is fastest but the noisiest (CV 8.3%)

evs_std_par wins on median (21.11 ms) yet has the highest variance (CV 8.3%), while evs_engine is the steadiest (CV 3.6%, 63.02 ms).


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine_par} vs {evs_engine, evs_std} (189% apart)

The field splits into a fast tier {evs_std_par, evs_engine_par} and a slow tier {evs_engine, evs_std} with a 189% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 3.1x the fastest

Fastest evs_std_par (21.11 ms) to slowest evs_std (64.92 ms): 3.1x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: evs_std_par at 21107731.0 ns median (-67.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.08x (fastest 21107731.0 ns, slowest 64918692.2 ns)

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
| evs_engine | 63127618ns | 63024225ns | 58073718ns | 62654043ns | 64746274ns | +0.81% |
| evs_engine_par | 21644765ns | 21829587ns | 19092155ns | 20355339ns | 23249987ns | -65.43% |
| evs_std | 62620298ns | 64923915ns | 54826110ns | 59891580ns | 66615383ns | base |
| evs_std_par | 21265658ns | 21115415ns | 17438369ns | 20823187ns | 22554334ns | -66.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 63122188ns | 58068613ns | 64740673ns | +0.81% | 0.066 |
| evs_engine_par | 21639367ns | 19086525ns | 23244443ns | -65.44% | 0.194 |
| evs_std | 62615168ns | 54821059ns | 66609989ns | base | 0.067 |
| evs_std_par | 21259673ns | 17433044ns | 22548908ns | -66.05% | 0.197 |

## Performance model

- Peak throughput: 0.241 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.067 | 27.7% |
| evs_engine_par | 0.192 | 79.9% |
| evs_std | 0.065 | 26.9% |
| evs_std_par | 0.199 | 82.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 63127618ns | 63127618ns | +0.81% |
| evs_engine_par | 21644765ns | 21644765ns | -65.43% |
| evs_std | 62620298ns | 62620298ns | base |
| evs_std_par | 21265658ns | 21265658ns | -66.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 64918692ns | base | --- | [54936779, 66978800] | --- | --- | --- | --- |
| evs_engine | 63019141ns | no significant difference | [-3175268, +3247554]ns | [62348823, 65264293] | no | 0.7266 | 0.7266 | 0 |
| evs_engine_par | 21824168ns | -42614753.9ns (-65.6%) | [-44621856, -35162426]ns | [19658633, 23367978] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 21107731ns | -42893742.7ns (-66.1%) | [-46456731, -33807576]ns | [20522068, 23091869] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 66200904ns | -2.2% | -67.0% | -68.2% |
| 1 | 2 | 57622975ns | +8.5% | -63.1% | -62.1% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 1/2, lost 1/2
- evs_engine_par: won 2/2, lost 0/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 94484909.9ns | 63122188.0ns | 149.7% | HIGH |
| evs_engine_par | 96446697.9ns | 21639367.2ns | 445.7% | HIGH |
| evs_std | 92277288.6ns | 62615167.7ns | 147.4% | HIGH |
| evs_std_par | 96873907.7ns | 21259673.0ns | 455.7% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 58068613.2-64740673.4 ns)
  58068613.2 |########################################
  58402216.2 |
  58735819.2 |
  59069422.2 |
  59403025.2 |
  59736628.3 |
  60070231.3 |
  60403834.3 |
  60737437.3 |
  61071040.3 |
  61404643.3 |
  61738246.3 |
  62071849.3 |########################################
  62405452.3 |########################################
  62739055.4 |########################################
  63072658.4 |########################################
  63406261.4 |
  63739864.4 |
  64073467.4 |########################################
  64407070.4 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 19086525.4-23244443.0 ns)
  19086525.4 |####################
  19294421.3 |
  19502317.2 |####################
  19710213.0 |
  19918108.9 |
  20126004.8 |
  20333900.7 |
  20541796.6 |####################
  20749692.4 |####################
  20957588.3 |
  21165484.2 |
  21373380.1 |
  21581276.0 |
  21789171.8 |
  21997067.7 |
  22204963.6 |
  22412859.5 |
  22620755.4 |
  22828651.2 |########################################
  23036547.1 |
  (0 below, 2 above range)

evs_std (n=8, range 54821059.4-66609989.1 ns)
  54821059.4 |########################################
  55410505.9 |
  55999952.4 |
  56589398.9 |
  57178845.3 |
  57768291.8 |
  58357738.3 |
  58947184.8 |
  59536631.3 |
  60126077.8 |####################
  60715524.3 |
  61304970.7 |
  61894417.2 |
  62483863.7 |
  63073310.2 |
  63662756.7 |
  64252203.2 |####################
  64841649.7 |####################
  65431096.2 |####################
  66020542.6 |
  (0 below, 2 above range)

evs_std_par (n=8, range 17433043.6-22548908.0 ns)
  17433043.6 |####################
  17688836.8 |
  17944630.0 |
  18200423.3 |
  18456216.5 |
  18712009.7 |
  18967802.9 |
  19223596.1 |
  19479389.4 |
  19735182.6 |
  19990975.8 |
  20246769.0 |
  20502562.2 |####################
  20758355.4 |########################################
  21014148.7 |####################
  21269941.9 |
  21525735.1 |
  21781528.3 |
  22037321.5 |
  22293114.8 |
  (0 below, 3 above range)

```

## Diagnostics

- evs_engine: bridge=150.6% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=441.6% of algo (FFI overhead may distort results)
- evs_std: bridge=146.4% of algo (FFI overhead may distort results)
- evs_std_par: bridge=460.8% of algo (FFI overhead may distort results)
