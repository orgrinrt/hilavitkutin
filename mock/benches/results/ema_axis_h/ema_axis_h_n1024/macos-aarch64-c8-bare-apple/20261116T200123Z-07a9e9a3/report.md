# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (383 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_neon at 42 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_neon dominates: 105% faster than the next best (ema_scalar)

ema_neon (42 ns) leads ema_scalar (85 ns) by 105%, a clear separation rather than a photo finish. CV 11.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_neon beats baseline by 89% (significant)

ema_neon is -342 ns (89%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec is an outlier: 9.2x slower than the field

ema_autovec (383 ns) is 9.2x the fastest (42 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_neon is fastest but the noisiest (CV 11.2%)

ema_neon wins on median (42 ns) yet has the highest variance (CV 11.2%), while ema_autovec is the steadiest (CV 3.1%, 383 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 9.2x the fastest

Fastest ema_neon (42 ns) to slowest ema_autovec (383 ns): 9.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ema_neon** at 41.7 ns median (-89.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 9.19x (fastest 41.7 ns, slowest 383.3 ns)

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
| ema_autovec | 2506ns | 2482ns | 2468ns | 2495ns | 2576ns | base |
| ema_neon | 2155ns | 2141ns | 2134ns | 2150ns | 2191ns | -14.00% |
| ema_scalar | 2189ns | 2169ns | 2161ns | 2179ns | 2246ns | -12.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 387ns | 377ns | 402ns | base | 2.649 |
| ema_neon | 42ns | 35ns | 48ns | -89.18% | 24.475 |
| ema_scalar | 86ns | 80ns | 93ns | -77.81% | 11.938 |

## Performance model

- Peak throughput: **28.914 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 2.672 | 9.2% |
| ema_neon | 24.556 | 84.9% |
| ema_scalar | 11.991 | 41.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2506ns | 2506ns | base |
| ema_neon | 2155ns | 2155ns | -14.00% |
| ema_scalar | 2189ns | 2189ns | -12.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 383ns | base | --- | [382, 385] | --- | --- | --- | --- |
| ema_neon | 42ns | -341.7ns (-89.1%) | [-343, -340]ns | [41, 42] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 85ns | -297.9ns (-77.7%) | [-299, -297]ns | [85, 86] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 380ns | -88.9% | -77.3% |
| 1 | 2 | 380ns | -89.3% | -78.0% |
| 1 | 3 | 394ns | -89.4% | -78.5% |
| 1 | 4 | 384ns | -89.1% | -77.4% |
| 1 | 5 | 381ns | -89.0% | -77.7% |
| 1 | 6 | 395ns | -89.4% | -77.5% |
| 1 | 7 | 381ns | -89.2% | -77.6% |
| 1 | 8 | 384ns | -88.8% | -77.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.387 | moderate- |
| ema_neon | -0.312 | moderate- |
| ema_scalar | -0.192 | ok |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 2.3ns | 386.5ns | 0.6% |  |
| ema_neon | 2.4ns | 41.8ns | 5.7% | HIGH |
| ema_scalar | 2.1ns | 85.8ns | 2.5% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 376.6-402.0 ns)
    376.6 |########################
    377.9 |##########################
    379.1 |##########################
    380.4 |###############################
    381.7 |########################################
    382.9 |####################
    384.2 |#############
    385.5 |########
    386.7 |######
    388.0 |###############
    389.3 |########
    390.6 |#################
    391.8 |########
    393.1 |#############
    394.4 |###############
    395.6 |#############
    396.9 |#############
    398.2 |########
    399.4 |##
    400.7 |
  (12 below, 6 above range)

ema_neon (n=160, range 35.4-48.5 ns)
     35.4 |#####
     36.1 |###########################
     36.7 |##
     37.4 |############
     38.0 |############
     38.7 |#################
     39.3 |##
     40.0 |###########################
     40.6 |########################################
     41.3 |####################
     41.9 |######################
     42.6 |#######
     43.3 |################################
     43.9 |######################
     44.6 |###############
     45.2 |################################
     45.9 |##########
     46.5 |#####
     47.2 |#######
     47.8 |#######
  (17 below, 11 above range)

ema_scalar (n=160, range 79.7-92.7 ns)
     79.7 |####
     80.3 |#########################
     81.0 |##################
     81.6 |#######################
     82.3 |#######
     82.9 |###########
     83.6 |############################
     84.2 |#####################
     84.9 |#########################
     85.5 |#######
     86.2 |########################################
     86.9 |#######################
     87.5 |####
     88.2 |##################
     88.8 |###########
     89.5 |#####################
     90.1 |#######
     90.8 |####
     91.4 |
     92.1 |#########
  (11 below, 15 above range)

```
