# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std has the worst median (584.11 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_engine at 570.99 us).


### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.3% (fastest 570.99 us to slowest 584.11 us).


### evs_engine shows alternating (throttle bounce) (autocorr -0.50)

evs_engine's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


## Key findings

- Fastest: evs_engine at 570991.7 ns median (-2.2% vs baseline)
- Spread: 1.02x (fastest 570991.7 ns, slowest 584108.3 ns)

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
| evs_engine | 576097ns | 573225ns | 569650ns | 570817ns | 581669ns | -4.09% |
| evs_std | 600650ns | 586279ns | 567125ns | 575433ns | 627944ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 573945ns | 566867ns | 579490ns | -4.08% | 7.308 |
| evs_std | 598366ns | 565117ns | 625315ns | base | 7.010 |

## Performance model

- Peak throughput: 7.422 Gops/s (evs_std; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 7.346 | 99.0% |
| evs_std | 7.181 | 96.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 576097ns | 576097ns | -4.09% |
| evs_std | 600650ns | 600650ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 584108ns | base | --- | [568042, 631550] | --- | --- | --- | --- |
| evs_engine | 570992ns | no significant difference | [-64683, +4025]ns | [568567, 575500] | no | 0.7266 | 0.7266 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 606288ns | -5.2% |
| 1 | 2 | 569167ns | -0.1% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | HIGH- (thermal bounce) |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 1/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 22804926.1ns | 573944.8ns | 3973.4% | HIGH |
| evs_std | 22889750.0ns | 598365.6ns | 3825.4% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 566866.7-579489.6 ns)
  566866.7 |####################
  567497.8 |
  568129.0 |####################
  568760.1 |########################################
  569391.3 |
  570022.4 |
  570653.6 |
  571284.7 |
  571915.9 |
  572547.0 |####################
  573178.1 |####################
  573809.3 |
  574440.4 |
  575071.6 |####################
  575702.7 |
  576333.9 |
  576965.0 |
  577596.2 |
  578227.3 |
  578858.5 |
  (0 below, 1 above range)

evs_std (n=8, range 565116.7-625314.6 ns)
  565116.7 |########################################
  568126.6 |####################
  571136.5 |
  574146.4 |
  577156.3 |
  580166.2 |####################
  583176.1 |####################
  586186.0 |
  589195.8 |
  592205.7 |
  595215.6 |
  598225.5 |
  601235.4 |
  604245.3 |
  607255.2 |
  610265.1 |
  613275.0 |
  616284.9 |
  619294.8 |
  622304.7 |
  (0 below, 3 above range)

```

## Diagnostics

- evs_engine: bridge=3990.3% of algo (FFI overhead may distort results)
- evs_std: bridge=3902.0% of algo (FFI overhead may distort results)
