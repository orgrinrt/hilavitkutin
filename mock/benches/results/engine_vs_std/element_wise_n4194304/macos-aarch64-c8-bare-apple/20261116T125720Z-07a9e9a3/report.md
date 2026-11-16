# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.8% (fastest 573.05 us to slowest 577.82 us).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (573.05 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 573054.2 ns median
- Spread: 1.01x (fastest 573054.2 ns, slowest 577820.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | element_wise, four-stage RAW chain |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 586672ns | 579646ns | 569750ns | 576967ns | 598181ns | +2.00% |
| evs_std | 575180ns | 574675ns | 569058ns | 572456ns | 578754ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 584877ns | 567767ns | 596604ns | +2.02% | 7.171 |
| evs_std | 573307ns | 567150ns | 576979ns | base | 7.316 |

## Performance model

- Peak throughput: 7.395 Gops/s (evs_std; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 7.259 | 98.2% |
| evs_std | 7.319 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 586672ns | 586672ns | +2.00% |
| evs_std | 575180ns | 575180ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 573054ns | base | --- | [568925, 578983] | --- | --- | --- | --- |
| evs_engine | 577821ns | no significant difference | [-2750, +33950]ns | [572725, 601100] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 570438ns | +3.8% |
| 1 | 2 | 576642ns | -0.5% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 1/2, lost 1/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 22901294.8ns | 584877.1ns | 3915.6% | HIGH |
| evs_std | 22750762.5ns | 573307.3ns | 3968.3% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 567766.7-596604.2 ns)
  567766.7 |########################################
  569208.6 |
  570650.4 |
  572092.3 |########################################
  573534.2 |
  574976.1 |########################################
  576417.9 |########################################
  577859.8 |########################################
  579301.7 |
  580743.6 |
  582185.4 |########################################
  583627.3 |
  585069.2 |
  586511.1 |
  587952.9 |
  589394.8 |
  590836.7 |
  592278.6 |
  593720.4 |
  595162.3 |
  (0 below, 2 above range)

evs_std (n=8, range 567150.0-576979.2 ns)
  567150.0 |####################
  567641.5 |
  568132.9 |
  568624.4 |####################
  569115.8 |
  569607.3 |
  570098.7 |####################
  570590.2 |
  571081.7 |
  571573.1 |####################
  572064.6 |
  572556.0 |
  573047.5 |
  573538.9 |
  574030.4 |########################################
  574521.9 |
  575013.3 |
  575504.8 |
  575996.2 |
  576487.7 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=3941.1% of algo (FFI overhead may distort results)
- evs_std: bridge=3969.3% of algo (FFI overhead may distort results)
