# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (742 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_scalar at 390 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_scalar dominates: 57% faster than the next best (ema_neon)

ema_scalar (390 ns) leads ema_neon (614 ns) by 57%, a clear separation rather than a photo finish. CV 79.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_scalar beats baseline by 41% (significant)

ema_scalar is -301 ns (41%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_scalar is fastest but the noisiest (CV 79.7%)

ema_scalar wins on median (390 ns) yet has the highest variance (CV 79.7%), while ema_autovec is the steadiest (CV 47.2%, 742 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ema_scalar is inconsistent: worst-20% is 2.8x its best-20%

ema_scalar's best 20% of batches run at 364 ns but its worst 20% at 1.02 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_scalar** at 390.4 ns median (-47.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.90x (fastest 390.4 ns, slowest 742.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | EMA streaming pass: NEON vs scalar vs autovec under realistic workload |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112807538 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_autovec | 2910ns | 2637ns | 2388ns | 2637ns | 4249ns | base |
| ema_neon | 2951ns | 2169ns | 2155ns | 2375ns | 5476ns | +1.42% |
| ema_scalar | 2727ns | 2077ns | 1905ns | 2118ns | 5373ns | -6.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 817ns | 673ns | 1182ns | base | 5.016 |
| ema_neon | 831ns | 609ns | 1533ns | +1.73% | 4.930 |
| ema_scalar | 520ns | 364ns | 1023ns | -36.29% | 7.873 |

## Performance model

- Peak throughput: **11.262 Gops/s** (ema_scalar; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 5.519 | 49.0% |
| ema_neon | 6.669 | 59.2% |
| ema_scalar | 10.492 | 93.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2910ns | 2910ns | base |
| ema_neon | 2951ns | 2951ns | +1.42% |
| ema_scalar | 2727ns | 2727ns | -6.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 742ns | base | --- | [742, 742] | --- | --- | --- | --- |
| ema_neon | 614ns | no significant difference | [-132, +2]ns | [609, 744] | no | 0.6928 | 0.6928 | 0 |
| ema_scalar | 390ns | -300.7ns (-40.5%) | [-376, -297]ns | [365, 445] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 742ns | -17.9% | -50.9% |
| 1 | 2 | 742ns | +0.4% | -50.7% |
| 1 | 3 | 742ns | -17.9% | -39.9% |
| 1 | 4 | 742ns | -18.0% | -39.9% |
| 1 | 5 | 742ns | -17.9% | -39.8% |
| 1 | 6 | 744ns | +0.0% | -51.1% |
| 1 | 7 | 607ns | +0.3% | -39.9% |
| 1 | 8 | 742ns | +0.9% | -50.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.174 | ok |
| ema_neon | -0.471 | moderate- |
| ema_scalar | 0.392 | moderate+ |

**Consistency summary:**

- **ema_neon**: won 4/8, lost 3/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 26.3ns | 816.6ns | 3.2% |  |
| ema_neon | 26.8ns | 830.8ns | 3.2% |  |
| ema_scalar | 28.5ns | 520.3ns | 5.5% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 672.7-1182.3 ns)
    672.7 |
    698.2 |
    723.7 |########################################
    749.2 |#
    774.7 |
    800.1 |
    825.6 |
    851.1 |
    876.6 |
    902.1 |
    927.5 |
    953.0 |
    978.5 |
   1004.0 |
   1029.5 |
   1054.9 |
   1080.4 |
   1105.9 |
   1131.4 |
   1156.9 |
  (16 below, 11 above range)

ema_neon (n=160, range 608.9-1533.1 ns)
    608.9 |########################################
    655.1 |
    701.3 |#######################
    747.5 |#######
    793.8 |#
    840.0 |#
    886.2 |
    932.4 |
    978.6 |
   1024.8 |
   1071.0 |
   1117.2 |
   1163.4 |
   1209.7 |
   1255.9 |
   1302.1 |
   1348.3 |
   1394.5 |
   1440.7 |
   1486.9 |
  (18 below, 17 above range)

ema_scalar (n=160, range 363.7-1022.7 ns)
    363.7 |########################################
    396.6 |#
    429.6 |################################
    462.5 |
    495.5 |
    528.4 |
    561.4 |
    594.3 |
    627.3 |
    660.2 |
    693.2 |
    726.1 |
    759.1 |
    792.0 |
    825.0 |
    857.9 |
    890.9 |
    923.8 |
    956.8 |
    989.7 |#
  (14 below, 17 above range)

```

## Diagnostics

- **ema_autovec**: CV=42.9% (high variance, measurements may be unstable)
- **ema_neon**: CV=55.3% (high variance, measurements may be unstable)
- **ema_scalar**: CV=59.8% (high variance, measurements may be unstable)
