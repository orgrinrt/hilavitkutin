# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### seq_live dominates: 143% faster than the next best (seq_snapshot)

seq_live (1.52 ms) leads seq_snapshot (3.70 ms) by 143%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (seq_live)

The baseline seq_live is the fastest (1.52 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (seq_live) is the fastest** at 1522170.4 ns median
- 1 variant significantly slower than baseline
- Spread: 2.43x (fastest 1522170.4 ns, slowest 3704651.0 ns)

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
| seq_live | 1533615ns | 1523756ns | 1508644ns | 1525523ns | 1582858ns | base |
| seq_snapshot | 3757975ns | 3705300ns | 3636213ns | 3717814ns | 4000221ns | +145.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 1531977ns | 1506956ns | 1581342ns | base | 10.951 |
| seq_snapshot | 3757263ns | 3635671ns | 3999193ns | +145.26% | 4.465 |

## Performance model

- Peak throughput: **11.133 Gops/s** (seq_live; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 11.022 | 99.0% |
| seq_snapshot | 4.529 | 40.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 1533615ns | 1533615ns | base |
| seq_snapshot | 3757975ns | 3757975ns | +145.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 1522170ns | base | --- | [1518395, 1528120] | --- | --- | --- | --- |
| seq_snapshot | 3704651ns | +2177359.8ns (+143.0%) | [+2151510, +2200773]ns | [3691010, 3733726] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 1509579ns | +146.5% |
| 1 | 2 | 1524281ns | +140.8% |
| 1 | 3 | 1528245ns | +139.1% |
| 1 | 4 | 1519898ns | +144.4% |
| 1 | 5 | 1533767ns | +141.1% |
| 2 | 1 | 1523348ns | +142.0% |
| 2 | 2 | 1514641ns | +144.5% |
| 2 | 3 | 1543981ns | +146.5% |
| 2 | 4 | 1516997ns | +146.4% |
| 2 | 5 | 1523228ns | +146.3% |
| 3 | 1 | 1522932ns | +144.9% |
| 3 | 2 | 1521318ns | +140.0% |
| 3 | 3 | 1522768ns | +143.8% |
| 3 | 4 | 1524970ns | +142.0% |
| 3 | 5 | 1514280ns | +147.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | -0.392 | moderate- |
| seq_snapshot | 0.165 | ok |

**Consistency summary:**

- **seq_snapshot**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 27484453.4ns | 1531977.3ns | 1794.1% | HIGH |
| seq_snapshot | 27501635.7ns | 3757263.0ns | 732.0% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 1506956.4-1581341.6 ns)
  1506956.4 |#################################
  1510675.6 |########################################
  1514394.9 |#############
  1518114.2 |####################
  1521833.4 |####################
  1525552.7 |#################
  1529271.9 |###############
  1532991.2 |#############
  1536710.5 |####
  1540429.7 |##
  1544149.0 |######
  1547868.2 |########
  1551587.5 |######
  1555306.7 |
  1559026.0 |####
  1562745.3 |####
  1566464.5 |######
  1570183.8 |##
  1573903.0 |
  1577622.3 |
  (10 below, 11 above range)

seq_snapshot (n=120, range 3635671.1-3999192.7 ns)
  3635671.1 |########################################
  3653847.2 |#################
  3672023.3 |####
  3690199.3 |####################
  3708375.4 |#
  3726551.5 |########################
  3744727.6 |####
  3762903.7 |###
  3781079.7 |########
  3799255.8 |#
  3817431.9 |######
  3835608.0 |####
  3853784.1 |#
  3871960.1 |###
  3890136.2 |###
  3908312.3 |#
  3926488.4 |####
  3944664.5 |#
  3962840.5 |###
  3981016.6 |
  (10 below, 12 above range)

```

## Diagnostics

- **seq_live**: bridge=1799.9% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=739.6% of algo (FFI overhead may distort results)
