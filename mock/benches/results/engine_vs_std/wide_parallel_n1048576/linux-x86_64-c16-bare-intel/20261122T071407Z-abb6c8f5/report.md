# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std has the worst median (10.70 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_std_par at 4.41 ms).


### evs_std_par beats baseline by 56% (significant)

evs_std_par is -6.04 ms (56%) faster than baseline evs_std, with a CI that excludes zero.


### evs_std is an outlier: 2.4x slower than the field

evs_std (10.70 ms) is 2.4x the fastest (4.41 ms), well off the pack.


### evs_std_par is fastest but the noisiest (CV 8.0%)

evs_std_par wins on median (4.41 ms) yet has the highest variance (CV 8.0%), while evs_engine is the steadiest (CV 0.4%, 10.43 ms).


### evs_engine_par shows alternating (throttle bounce) (autocorr -0.50)

evs_engine_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine_par} vs {evs_engine, evs_std} (120% apart)

The field splits into a fast tier {evs_std_par, evs_engine_par} and a slow tier {evs_engine, evs_std} with a 120% jump between them - a qualitative difference, not a gradient.


## Key findings

- Fastest: evs_std_par at 4411610.5 ns median (-58.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.43x (fastest 4411610.5 ns, slowest 10700248.8 ns)

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
| Counter frequency | 2112761676 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 10442681ns | 10432393ns | 10390210ns | 10416664ns | 10475311ns | -2.60% |
| evs_engine_par | 4734311ns | 4739224ns | 4267733ns | 4461498ns | 5055566ns | -55.84% |
| evs_std | 10721844ns | 10702038ns | 10415250ns | 10430300ns | 11017150ns | base |
| evs_std_par | 4591890ns | 4414892ns | 4363057ns | 4393938ns | 4797561ns | -57.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 10441576ns | 10388752ns | 10474138ns | -2.60% | 0.100 |
| evs_engine_par | 4731486ns | 4265038ns | 5052733ns | -55.86% | 0.222 |
| evs_std | 10720186ns | 10413729ns | 11015535ns | base | 0.098 |
| evs_std_par | 4588527ns | 4359628ns | 4794059ns | -57.20% | 0.229 |

## Performance model

- Peak throughput: 0.246 Gops/s (evs_engine_par; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.101 | 40.9% |
| evs_engine_par | 0.221 | 90.0% |
| evs_std | 0.098 | 39.9% |
| evs_std_par | 0.238 | 96.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 10442681ns | 10442681ns | -2.60% |
| evs_engine_par | 4734311ns | 4734311ns | -55.84% |
| evs_std | 10721844ns | 10721844ns | base |
| evs_std_par | 4591890ns | 4591890ns | -57.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 10700249ns | base | --- | [10425283, 10974173] | --- | --- | --- | --- |
| evs_engine | 10431291ns | no significant difference | [-566090, +24692]ns | [10408082, 10493146] | no | 0.2891 | 0.2891 | 0 |
| evs_engine_par | 4736490ns | -5912876.5ns (-55.3%) | [-6701271, -5448405]ns | [4304465, 5061254] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 4411611ns | -6038220.4ns (-56.4%) | [-6367650, -6001798]ns | [4384157, 4661415] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 10425714ns | +0.1% | -52.5% | -57.9% |
| 1 | 2 | 10974102ns | -5.0% | -59.6% | -57.8% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 1/2, lost 0/2
- evs_engine_par: won 2/2, lost 0/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 16982252.8ns | 10441576.1ns | 162.6% | HIGH |
| evs_engine_par | 17343349.9ns | 4731486.0ns | 366.6% | HIGH |
| evs_std | 17063907.7ns | 10720185.7ns | 159.2% | HIGH |
| evs_std_par | 17466669.0ns | 4588527.2ns | 380.7% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 10388751.7-10474137.9 ns)
  10388751.7 |########################################
  10393021.0 |
  10397290.3 |
  10401559.6 |
  10405828.9 |########################################
  10410098.2 |########################################
  10414367.6 |
  10418636.9 |
  10422906.2 |
  10427175.5 |########################################
  10431444.8 |########################################
  10435714.1 |########################################
  10439983.4 |
  10444252.7 |
  10448522.0 |
  10452791.3 |
  10457060.7 |
  10461330.0 |
  10465599.3 |
  10469868.6 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 4265038.4-5052732.6 ns)
  4265038.4 |########################################
  4304423.1 |########################################
  4343807.8 |
  4383192.5 |
  4422577.2 |
  4461962.0 |
  4501346.7 |########################################
  4540731.4 |########################################
  4580116.1 |
  4619500.8 |
  4658885.5 |
  4698270.2 |
  4737654.9 |
  4777039.6 |
  4816424.3 |
  4855809.0 |
  4895193.8 |########################################
  4934578.5 |
  4973963.2 |########################################
  5013347.9 |
  (0 below, 2 above range)

evs_std (n=8, range 10413728.9-11015535.2 ns)
  10413728.9 |########################################
  10443819.2 |
  10473909.5 |
  10503999.8 |
  10534090.2 |
  10564180.5 |
  10594270.8 |
  10624361.1 |
  10654451.4 |
  10684541.7 |
  10714632.0 |
  10744722.3 |
  10774812.7 |
  10804903.0 |
  10834993.3 |
  10865083.6 |
  10895173.9 |
  10925264.2 |
  10955354.5 |##############################
  10985444.8 |
  (0 below, 1 above range)

evs_std_par (n=8, range 4359627.7-4794058.8 ns)
  4359627.7 |#############
  4381349.3 |########################################
  4403070.8 |#############
  4424792.4 |
  4446513.9 |
  4468235.5 |
  4489957.0 |
  4511678.6 |
  4533400.1 |
  4555121.7 |
  4576843.2 |
  4598564.8 |#############
  4620286.4 |
  4642007.9 |#############
  4663729.5 |
  4685451.0 |
  4707172.6 |
  4728894.1 |
  4750615.7 |
  4772337.2 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=163.0% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=365.8% of algo (FFI overhead may distort results)
- evs_std: bridge=159.6% of algo (FFI overhead may distort results)
- evs_std_par: bridge=395.9% of algo (FFI overhead may distort results)
