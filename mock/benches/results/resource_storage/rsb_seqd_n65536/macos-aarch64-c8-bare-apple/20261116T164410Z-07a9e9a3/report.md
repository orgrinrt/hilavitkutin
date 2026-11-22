# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (seq_live)

The baseline seq_live is the fastest (334.80 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (seq_live) is the fastest** at 334803.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.02x (fastest 334803.0 ns, slowest 340558.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Seq collection member: live-stream vs snapshot-copy across payload size |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| seq_live | 335621ns | 335706ns | 334218ns | 335719ns | 336733ns | base |
| seq_snapshot | 341052ns | 340974ns | 340461ns | 340960ns | 341920ns | +1.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 334772ns | 333520ns | 335883ns | base | 0.196 |
| seq_snapshot | 340677ns | 340153ns | 341519ns | +1.76% | 0.192 |

## Performance model

- Peak throughput: **0.196 Gops/s** (seq_live; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 0.196 | 99.6% |
| seq_snapshot | 0.192 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 335621ns | 335621ns | base |
| seq_snapshot | 341052ns | 341052ns | +1.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 334803ns | base | --- | [334756, 334840] | --- | --- | --- | --- |
| seq_snapshot | 340559ns | +5722.9ns (+1.7%) | [+5664, +5810]ns | [340508, 340634] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 331399ns | +2.8% |
| 1 | 2 | 334702ns | +1.7% |
| 1 | 3 | 334821ns | +1.7% |
| 1 | 4 | 334793ns | +1.7% |
| 1 | 5 | 334778ns | +1.7% |
| 2 | 1 | 334878ns | +1.6% |
| 2 | 2 | 334858ns | +1.7% |
| 2 | 3 | 334609ns | +1.8% |
| 2 | 4 | 334976ns | +1.7% |
| 2 | 5 | 334753ns | +1.8% |
| 3 | 1 | 334786ns | +1.7% |
| 3 | 2 | 334758ns | +1.8% |
| 3 | 3 | 334968ns | +1.7% |
| 3 | 4 | 334825ns | +1.7% |
| 3 | 5 | 335028ns | +1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | 0.024 | ok |
| seq_snapshot | 0.131 | ok |

**Consistency summary:**

- **seq_snapshot**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 108483.8ns | 334771.9ns | 32.4% | HIGH |
| seq_snapshot | 108958.3ns | 340676.6ns | 32.0% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 333519.8-335882.9 ns)
  333519.8 |#
  333638.0 |
  333756.1 |
  333874.3 |
  333992.4 |
  334110.6 |
  334228.7 |
  334346.9 |#
  334465.1 |#############
  334583.2 |#####################
  334701.4 |########################################
  334819.5 |######################
  334937.7 |#############
  335055.8 |#########
  335174.0 |#########
  335292.1 |####
  335410.3 |#
  335528.4 |##
  335646.6 |##
  335764.8 |
  (7 below, 6 above range)

seq_snapshot (n=120, range 340152.9-341519.3 ns)
  340152.9 |####
  340221.3 |##########
  340289.6 |################
  340357.9 |#########################
  340426.2 |########################################
  340494.5 |#########################
  340562.8 |#####################
  340631.2 |#########################
  340699.5 |#############################
  340767.8 |############
  340836.1 |######
  340904.4 |##
  340972.7 |####
  341041.1 |##
  341109.4 |
  341177.7 |##
  341246.0 |##
  341314.3 |##
  341382.7 |##
  341451.0 |##
  (2 below, 6 above range)

```

## Diagnostics

- **seq_live**: bridge=32.4% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=32.0% of algo (FFI overhead may distort results)
