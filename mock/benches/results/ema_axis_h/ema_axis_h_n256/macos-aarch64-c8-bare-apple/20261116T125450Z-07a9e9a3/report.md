# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (76 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_neon at 1 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_neon dominates: 1875% faster than the next best (ema_scalar)

ema_neon (1 ns) leads ema_scalar (24 ns) by 1875%, a clear separation rather than a photo finish. CV 56.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_neon beats baseline by 98% (significant)

ema_neon is -74 ns (98%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec is an outlier: 63.2x slower than the field

ema_autovec (76 ns) is 63.2x the fastest (1 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_neon is fastest but the noisiest (CV 56.5%)

ema_neon wins on median (1 ns) yet has the highest variance (CV 56.5%), while ema_autovec is the steadiest (CV 8.8%, 76 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 63.2x the fastest

Fastest ema_neon (1 ns) to slowest ema_autovec (76 ns): 63.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ema_neon is inconsistent: worst-20% is 6.2x its best-20%

ema_neon's best 20% of batches run at 0 ns but its worst 20% at 2 ns (6.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_neon** at 1.2 ns median (-98.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 63.17x (fastest 1.2 ns, slowest 75.8 ns)

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
| ema_autovec | 2169ns | 2166ns | 2159ns | 2166ns | 2190ns | base |
| ema_neon | 2102ns | 2096ns | 2090ns | 2096ns | 2132ns | -3.10% |
| ema_scalar | 2107ns | 2104ns | 2097ns | 2104ns | 2126ns | -2.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 77ns | 71ns | 84ns | base | 3.346 |
| ema_neon | 1ns | 0ns | 2ns | -98.39% | 208.130 |
| ema_scalar | 24ns | 19ns | 28ns | -68.87% | 10.750 |

## Performance model

- Peak throughput: **706.207 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 3.377 | 0.5% |
| ema_neon | 213.333 | 30.2% |
| ema_scalar | 10.802 | 1.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2169ns | 2169ns | base |
| ema_neon | 2102ns | 2102ns | -3.10% |
| ema_scalar | 2107ns | 2107ns | -2.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 76ns | base | --- | [75, 76] | --- | --- | --- | --- |
| ema_neon | 1ns | -74.3ns (-98.1%) | [-75, -74]ns | [1, 1] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 24ns | -52.1ns (-68.7%) | [-53, -52]ns | [23, 24] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 74ns | -98.4% | -69.5% |
| 1 | 2 | 75ns | -98.4% | -67.6% |
| 1 | 3 | 76ns | -98.4% | -68.6% |
| 1 | 4 | 77ns | -98.4% | -68.5% |
| 1 | 5 | 75ns | -98.4% | -68.1% |
| 1 | 6 | 76ns | -98.7% | -69.3% |
| 1 | 7 | 75ns | -98.7% | -69.0% |
| 1 | 8 | 76ns | -98.4% | -68.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.082 | ok |
| ema_neon | 0.292 | moderate+ |
| ema_scalar | -0.114 | ok |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 3.9ns | 76.5ns | 5.1% | HIGH |
| ema_neon | 6.8ns | 1.2ns | 550.8% | HIGH |
| ema_scalar | 3.4ns | 23.8ns | 14.2% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 70.7-84.5 ns)
     70.7 |##################
     71.4 |##############
     72.1 |#######################
     72.8 |##############
     73.4 |#######
     74.1 |########################################
     74.8 |########################################
     75.5 |################################
     76.2 |#######################
     76.9 |#####################
     77.6 |##############
     78.3 |###########
     79.0 |#######
     79.6 |##############
     80.3 |##################
     81.0 |#######
     81.7 |####
     82.4 |##############
     83.1 |##
     83.8 |##
  (12 below, 7 above range)

ema_neon (n=160, range 0.4-2.2 ns)
      0.4 |########################
      0.5 |
      0.6 |
      0.6 |
      0.7 |######################################
      0.8 |
      0.9 |
      1.0 |
      1.1 |########################################
      1.2 |
      1.3 |
      1.4 |
      1.5 |
      1.6 |
      1.7 |##############################
      1.8 |
      1.9 |
      2.0 |
      2.1 |###############
      2.1 |
  (6 below, 12 above range)

ema_scalar (n=160, range 19.4-28.5 ns)
     19.4 |###########
     19.9 |###########
     20.3 |#########
     20.8 |#########################
     21.2 |
     21.7 |########################################
     22.1 |##################
     22.6 |##################
     23.0 |###########
     23.5 |###########
     24.0 |#######################
     24.4 |##############
     24.9 |##################
     25.3 |##################
     25.8 |############################
     26.2 |
     26.7 |################################
     27.1 |###########
     27.6 |####
     28.0 |#######
  (14 below, 10 above range)

```

## Diagnostics

- **ema_neon**: CV=55.1% (high variance, measurements may be unstable)
- **ema_neon**: worst_20/best_20 = 6.2x (possible bimodal distribution)
- **ema_neon**: bridge=558.3% of algo (FFI overhead may distort results)
- **ema_scalar**: bridge=13.9% of algo (FFI overhead may distort results)
