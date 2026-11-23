# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### seq_live dominates: 211% faster than the next best (seq_snapshot)

seq_live (4.56 ms) leads seq_snapshot (14.21 ms) by 211%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (seq_live)

The baseline seq_live is the fastest (4.56 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.1x the fastest

Fastest seq_live (4.56 ms) to slowest seq_snapshot (14.21 ms): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (seq_live) is the fastest** at 4563911.4 ns median
- 1 variant significantly slower than baseline
- Spread: 3.11x (fastest 4563911.4 ns, slowest 14207388.1 ns)

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
| Counter frequency | 2112730567 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| seq_live | 4572646ns | 4565100ns | 4535600ns | 4567444ns | 4625300ns | base |
| seq_snapshot | 14213890ns | 14208711ns | 14138428ns | 14203973ns | 14319104ns | +210.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 4571362ns | 4534296ns | 4624004ns | base | 3.670 |
| seq_snapshot | 14212547ns | 14137128ns | 14317750ns | +210.90% | 1.180 |

## Performance model

- Peak throughput: **3.700 Gops/s** (seq_live; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 3.676 | 99.4% |
| seq_snapshot | 1.181 | 31.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 4572646ns | 4572646ns | base |
| seq_snapshot | 14213890ns | 14213890ns | +210.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 4563911ns | base | --- | [4561011, 4572826] | --- | --- | --- | --- |
| seq_snapshot | 14207388ns | +9636309.6ns (+211.1%) | [+9617478, +9651972]ns | [14187922, 14219387] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 4569248ns | +210.4% |
| 1 | 2 | 4559867ns | +212.1% |
| 1 | 3 | 4558711ns | +210.0% |
| 1 | 4 | 4580502ns | +210.5% |
| 1 | 5 | 4565147ns | +209.8% |
| 2 | 1 | 4525025ns | +214.5% |
| 2 | 2 | 4583721ns | +209.9% |
| 2 | 3 | 4593738ns | +210.1% |
| 2 | 4 | 4545658ns | +211.2% |
| 2 | 5 | 4575767ns | +209.4% |
| 3 | 1 | 4559835ns | +211.8% |
| 3 | 2 | 4574186ns | +210.5% |
| 3 | 3 | 4538535ns | +213.1% |
| 3 | 4 | 4562140ns | +211.9% |
| 3 | 5 | 4576129ns | +210.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | -0.304 | moderate- |
| seq_snapshot | -0.368 | moderate- |

**Consistency summary:**

- **seq_snapshot**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 27172442.3ns | 4571361.9ns | 594.4% | HIGH |
| seq_snapshot | 27303937.8ns | 14212546.7ns | 192.1% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 4534296.4-4624003.9 ns)
  4534296.4 |#####
  4538781.8 |##############
  4543267.2 |###########
  4547752.6 |#########################
  4552237.9 |####################
  4556723.3 |###############################
  4561208.7 |########################################
  4565694.1 |########
  4570179.4 |######################
  4574664.8 |###############################
  4579150.2 |#################
  4583635.6 |######################
  4588120.9 |########
  4592606.3 |###########
  4597091.7 |#####
  4601577.1 |##
  4606062.4 |##
  4610547.8 |##
  4615033.2 |
  4619518.6 |
  (12 below, 8 above range)

seq_snapshot (n=120, range 14137127.7-14317749.7 ns)
  14137127.7 |#############################
  14146158.8 |########
  14155189.9 |##########
  14164221.0 |##################
  14173252.1 |#############################
  14182283.2 |##################
  14191314.3 |##########
  14200345.4 |########################
  14209376.5 |#############
  14218407.6 |########################################
  14227438.7 |#####################################
  14236469.8 |#####
  14245500.9 |##
  14254532.0 |################
  14263563.1 |
  14272594.2 |#####
  14281625.3 |##
  14290656.4 |##
  14299687.5 |##
  14308718.6 |##
  (8 below, 7 above range)

```

## Diagnostics

- **seq_live**: bridge=595.4% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=192.4% of algo (FFI overhead may distort results)
