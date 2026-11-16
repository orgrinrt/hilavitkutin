# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (382 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_neon at 42 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_neon dominates: 102% faster than the next best (ema_scalar)

ema_neon (42 ns) leads ema_scalar (86 ns) by 102%, a clear separation rather than a photo finish. CV 11.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_neon beats baseline by 89% (significant)

ema_neon is -338 ns (89%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec is an outlier: 9.0x slower than the field

ema_autovec (382 ns) is 9.0x the fastest (42 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_neon is fastest but the noisiest (CV 11.6%)

ema_neon wins on median (42 ns) yet has the highest variance (CV 11.6%), while ema_autovec is the steadiest (CV 2.3%, 382 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ema_autovec shows warm-up / thermal drift (autocorr +0.52)

ema_autovec's per-pass series has lag-1 autocorrelation +0.52, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 9.0x the fastest

Fastest ema_neon (42 ns) to slowest ema_autovec (382 ns): 9.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ema_neon** at 42.5 ns median (-88.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 8.99x (fastest 42.5 ns, slowest 382.1 ns)

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
| ema_autovec | 2482ns | 2469ns | 2455ns | 2478ns | 2523ns | base |
| ema_neon | 2147ns | 2135ns | 2122ns | 2144ns | 2183ns | -13.49% |
| ema_scalar | 2171ns | 2162ns | 2146ns | 2167ns | 2207ns | -12.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 383ns | 374ns | 394ns | base | 2.676 |
| ema_neon | 43ns | 36ns | 50ns | -88.81% | 23.919 |
| ema_scalar | 86ns | 81ns | 92ns | -77.50% | 11.893 |

## Performance model

- Peak throughput: **28.202 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 2.680 | 9.5% |
| ema_neon | 24.094 | 85.4% |
| ema_scalar | 11.935 | 42.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2482ns | 2482ns | base |
| ema_neon | 2147ns | 2147ns | -13.49% |
| ema_scalar | 2171ns | 2171ns | -12.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 382ns | base | --- | [380, 383] | --- | --- | --- | --- |
| ema_neon | 42ns | -338.3ns (-88.5%) | [-339, -338]ns | [41, 44] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 86ns | -295.0ns (-77.2%) | [-296, -294]ns | [85, 87] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 385ns | -89.4% | -77.5% |
| 1 | 2 | 389ns | -88.8% | -78.0% |
| 1 | 3 | 385ns | -88.9% | -77.8% |
| 1 | 4 | 381ns | -88.7% | -77.2% |
| 1 | 5 | 382ns | -88.3% | -77.5% |
| 1 | 6 | 376ns | -88.4% | -76.9% |
| 1 | 7 | 380ns | -89.1% | -77.3% |
| 1 | 8 | 380ns | -89.2% | -77.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | 0.521 | HIGH+ (drift/warm-up) |
| ema_neon | 0.165 | ok |
| ema_scalar | -0.177 | ok |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 2.4ns | 382.6ns | 0.6% |  |
| ema_neon | 2.7ns | 42.8ns | 6.2% | HIGH |
| ema_scalar | 2.3ns | 86.1ns | 2.7% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 373.9-393.9 ns)
    373.9 |#######
    374.9 |######################
    375.9 |####################
    376.9 |###############
    377.9 |########################################
    378.9 |##########
    379.9 |#########################
    380.9 |#################
    381.9 |###########################
    382.9 |################################
    383.9 |######################
    384.9 |#########################
    385.9 |##########
    386.9 |#########################
    387.9 |##########
    388.9 |#######
    389.9 |###############
    390.9 |##########
    391.9 |##
    392.9 |##
  (15 below, 6 above range)

ema_neon (n=160, range 36.3-49.9 ns)
     36.3 |
     37.0 |####################################
     37.7 |########################
     38.4 |###########################
     39.0 |#####################
     39.7 |######
     40.4 |###########################
     41.1 |########################
     41.8 |###############
     42.4 |#################################
     43.1 |########################################
     43.8 |###
     44.5 |##############################
     45.2 |#################################
     45.9 |###
     46.5 |########################
     47.2 |#####################
     47.9 |#####################
     48.6 |######
     49.3 |######
  (14 below, 13 above range)

ema_scalar (n=160, range 80.9-92.1 ns)
     80.9 |########
     81.4 |########
     82.0 |########################################
     82.5 |######################
     83.1 |##############
     83.7 |############################
     84.2 |###########
     84.8 |###########
     85.3 |########################################
     85.9 |####################
     86.5 |###########
     87.0 |##################################
     87.6 |#####################################
     88.2 |######################
     88.7 |#####
     89.3 |####################
     89.8 |####################
     90.4 |###########
     91.0 |
     91.5 |##
  (16 below, 14 above range)

```

## Diagnostics

- **ema_autovec**: autocorrelation=0.52 (measurement drift or warm-up artifact)
