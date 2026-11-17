# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### seq_live dominates: 22% faster than the next best (seq_snapshot)

seq_live (330.33 us) leads seq_snapshot (402.42 us) by 22%, a clear separation rather than a photo finish. CV 5.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### seq_live is fastest but the noisiest (CV 5.8%)

seq_live wins on median (330.33 us) yet has the highest variance (CV 5.8%), while seq_snapshot is the steadiest (CV 1.9%, 402.42 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (seq_live)

The baseline seq_live is the fastest (330.33 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (seq_live) is the fastest** at 330334.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.22x (fastest 330334.8 ns, slowest 402423.5 ns)

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
| seq_live | 336353ns | 332268ns | 330451ns | 332410ns | 354084ns | base |
| seq_snapshot | 404693ns | 402502ns | 398211ns | 403170ns | 415745ns | +20.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 334667ns | 328438ns | 353120ns | base | 3.133 |
| seq_snapshot | 404602ns | 398142ns | 415615ns | +20.90% | 2.592 |

## Performance model

- Peak throughput: **3.193 Gops/s** (seq_live; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 3.174 | 99.4% |
| seq_snapshot | 2.606 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 336353ns | 336353ns | base |
| seq_snapshot | 404693ns | 404693ns | +20.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 330335ns | base | --- | [330067, 330624] | --- | --- | --- | --- |
| seq_snapshot | 402424ns | +71484.1ns (+21.6%) | [+70599, +72249]ns | [401605, 403706] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 329718ns | +21.4% |
| 1 | 2 | 330149ns | +22.0% |
| 1 | 3 | 344719ns | +19.5% |
| 1 | 4 | 330089ns | +22.6% |
| 1 | 5 | 329880ns | +21.7% |
| 2 | 1 | 330837ns | +21.4% |
| 2 | 2 | 336448ns | +19.1% |
| 2 | 3 | 329514ns | +21.8% |
| 2 | 4 | 330447ns | +21.2% |
| 2 | 5 | 329979ns | +21.1% |
| 3 | 1 | 330983ns | +24.3% |
| 3 | 2 | 329526ns | +21.6% |
| 3 | 3 | 333112ns | +22.2% |
| 3 | 4 | 329788ns | +24.2% |
| 3 | 5 | 330226ns | +21.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | -0.190 | ok |
| seq_snapshot | -0.070 | ok |

**Consistency summary:**

- **seq_snapshot**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 1721238.5ns | 334667.2ns | 514.3% | HIGH |
| seq_snapshot | 1712190.3ns | 404602.4ns | 423.2% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 328438.5-353119.9 ns)
  328438.5 |###########################
  329672.6 |########################################
  330906.6 |###########
  332140.7 |#######
  333374.8 |#######
  334608.8 |#####
  335842.9 |##
  337077.0 |#
  338311.1 |#
  339545.1 |#
  340779.2 |#
  342013.3 |
  343247.4 |##
  344481.4 |#
  345715.5 |#
  346949.6 |
  348183.6 |
  349417.7 |
  350651.8 |
  351885.9 |
  (12 below, 4 above range)

seq_snapshot (n=120, range 398141.6-415614.7 ns)
  398141.6 |#################################
  399015.2 |########################################
  399888.9 |##############################
  400762.5 |#################################
  401636.2 |#################################
  402509.8 |#######################
  403383.5 |##########################
  404257.1 |################
  405130.8 |######
  406004.5 |##########
  406878.1 |##########
  407751.8 |######
  408625.4 |################
  409499.1 |##########
  410372.7 |##########
  411246.4 |################
  412120.1 |###
  412993.7 |##########
  413867.4 |###
  414741.0 |##########
  (10 below, 5 above range)

```

## Diagnostics

- **seq_live**: bridge=516.6% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=424.2% of algo (FFI overhead may distort results)
