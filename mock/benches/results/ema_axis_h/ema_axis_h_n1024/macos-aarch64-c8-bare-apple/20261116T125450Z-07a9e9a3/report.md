# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (381 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_neon at 40 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_neon dominates: 110% faster than the next best (ema_scalar)

ema_neon (40 ns) leads ema_scalar (85 ns) by 110%, a clear separation rather than a photo finish. CV 21.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_neon beats baseline by 90% (significant)

ema_neon is -341 ns (90%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec is an outlier: 9.4x slower than the field

ema_autovec (381 ns) is 9.4x the fastest (40 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_neon is fastest but the noisiest (CV 21.5%)

ema_neon wins on median (40 ns) yet has the highest variance (CV 21.5%), while ema_autovec is the steadiest (CV 2.3%, 381 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 9.4x the fastest

Fastest ema_neon (40 ns) to slowest ema_autovec (381 ns): 9.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ema_neon is inconsistent: worst-20% is 2.0x its best-20%

ema_neon's best 20% of batches run at 25 ns but its worst 20% at 48 ns (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_neon** at 40.4 ns median (-89.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 9.44x (fastest 40.4 ns, slowest 381.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_autovec | 2492ns | 2472ns | 2467ns | 2473ns | 2576ns | base |
| ema_neon | 2156ns | 2139ns | 2133ns | 2144ns | 2215ns | -13.50% |
| ema_scalar | 2176ns | 2167ns | 2160ns | 2167ns | 2217ns | -12.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 384ns | 376ns | 399ns | base | 2.666 |
| ema_neon | 39ns | 25ns | 48ns | -89.86% | 26.285 |
| ema_scalar | 85ns | 79ns | 92ns | -77.80% | 12.010 |

## Performance model

- Peak throughput: **41.244 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 2.686 | 6.5% |
| ema_neon | 25.347 | 61.5% |
| ema_scalar | 12.075 | 29.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2492ns | 2492ns | base |
| ema_neon | 2156ns | 2156ns | -13.50% |
| ema_scalar | 2176ns | 2176ns | -12.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 381ns | base | --- | [380, 382] | --- | --- | --- | --- |
| ema_neon | 40ns | -341.2ns (-89.5%) | [-342, -340]ns | [40, 42] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 85ns | -296.7ns (-77.8%) | [-298, -296]ns | [84, 86] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 383ns | -89.1% | -77.5% |
| 1 | 2 | 379ns | -94.8% | -77.9% |
| 1 | 3 | 379ns | -89.1% | -77.6% |
| 1 | 4 | 381ns | -88.9% | -77.7% |
| 1 | 5 | 383ns | -88.9% | -78.1% |
| 1 | 6 | 380ns | -89.3% | -77.5% |
| 1 | 7 | 402ns | -89.6% | -78.4% |
| 1 | 8 | 381ns | -89.4% | -78.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.188 | ok |
| ema_neon | -0.156 | ok |
| ema_scalar | -0.385 | moderate- |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 2.3ns | 384.0ns | 0.6% |  |
| ema_neon | 2.3ns | 39.0ns | 5.9% | HIGH |
| ema_scalar | 2.1ns | 85.3ns | 2.5% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 375.8-399.0 ns)
    375.8 |#################################
    376.9 |########################################
    378.1 |###############################
    379.3 |#################################
    380.4 |#################
    381.6 |############################
    382.8 |######################
    383.9 |##########################
    385.1 |###########
    386.2 |####
    387.4 |####
    388.6 |######
    389.7 |###############
    390.9 |
    392.1 |####
    393.2 |##
    394.4 |######
    395.5 |
    396.7 |##
    397.9 |
  (11 below, 18 above range)

ema_neon (n=160, range 24.8-48.5 ns)
     24.8 |##
     26.0 |
     27.2 |
     28.4 |
     29.6 |
     30.7 |####
     31.9 |#######
     33.1 |###########
     34.3 |#########
     35.5 |#######################
     36.6 |#########################
     37.8 |##############
     39.0 |#####################################
     40.2 |#####################
     41.4 |#####################################
     42.6 |##############
     43.7 |########################################
     44.9 |############################
     46.1 |################
     47.3 |#######
  (19 below, 13 above range)

ema_scalar (n=160, range 78.8-92.1 ns)
     78.8 |######
     79.5 |##########
     80.1 |################
     80.8 |################
     81.5 |########################################
     82.1 |#################################
     82.8 |####################################
     83.5 |################
     84.1 |########################################
     84.8 |####################
     85.4 |##########
     86.1 |##############################
     86.8 |##############################
     87.4 |#################################
     88.1 |####################################
     88.8 |################
     89.4 |################
     90.1 |######
     90.8 |##########
     91.4 |###
  (15 below, 16 above range)

```

## Diagnostics

- **ema_neon**: CV=22.3% (high variance, measurements may be unstable)
