# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (77 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_neon at 1 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_neon dominates: 1875% faster than the next best (ema_scalar)

ema_neon (1 ns) leads ema_scalar (24 ns) by 1875%, a clear separation rather than a photo finish. CV 61.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_neon beats baseline by 98% (significant)

ema_neon is -75 ns (98%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec is an outlier: 63.9x slower than the field

ema_autovec (77 ns) is 63.9x the fastest (1 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_neon is fastest but the noisiest (CV 61.3%)

ema_neon wins on median (1 ns) yet has the highest variance (CV 61.3%), while ema_autovec is the steadiest (CV 6.7%, 77 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 63.9x the fastest

Fastest ema_neon (1 ns) to slowest ema_autovec (77 ns): 63.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ema_neon is inconsistent: worst-20% is 5.3x its best-20%

ema_neon's best 20% of batches run at 0 ns but its worst 20% at 2 ns (5.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_neon** at 1.2 ns median (-98.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 63.92x (fastest 1.2 ns, slowest 76.7 ns)

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
| ema_autovec | 2193ns | 2202ns | 2163ns | 2191ns | 2232ns | base |
| ema_neon | 2127ns | 2104ns | 2092ns | 2114ns | 2200ns | -3.02% |
| ema_scalar | 2136ns | 2142ns | 2103ns | 2132ns | 2181ns | -2.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 77ns | 72ns | 84ns | base | 3.317 |
| ema_neon | 1ns | 0ns | 2ns | -98.32% | 197.303 |
| ema_scalar | 24ns | 19ns | 29ns | -69.11% | 10.740 |

## Performance model

- Peak throughput: **553.514 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 3.338 | 0.6% |
| ema_neon | 213.333 | 38.5% |
| ema_scalar | 10.802 | 2.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2193ns | 2193ns | base |
| ema_neon | 2127ns | 2127ns | -3.02% |
| ema_scalar | 2136ns | 2136ns | -2.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 77ns | base | --- | [76, 77] | --- | --- | --- | --- |
| ema_neon | 1ns | -75.4ns (-98.3%) | [-76, -74]ns | [1, 1] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 24ns | -53.3ns (-69.5%) | [-54, -52]ns | [23, 24] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 76ns | -98.1% | -68.7% |
| 1 | 2 | 77ns | -98.4% | -70.2% |
| 1 | 3 | 77ns | -98.4% | -68.3% |
| 1 | 4 | 76ns | -98.4% | -69.3% |
| 1 | 5 | 75ns | -98.9% | -69.5% |
| 1 | 6 | 75ns | -98.9% | -67.0% |
| 1 | 7 | 77ns | -98.7% | -68.5% |
| 1 | 8 | 77ns | -97.8% | -69.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | 0.255 | moderate+ |
| ema_neon | 0.162 | ok |
| ema_scalar | -0.313 | moderate- |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 3.9ns | 77.2ns | 5.1% | HIGH |
| ema_neon | 6.5ns | 1.3ns | 502.6% | HIGH |
| ema_scalar | 3.4ns | 23.8ns | 14.2% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 71.6-84.4 ns)
     71.6 |######
     72.3 |#######################
     72.9 |######
     73.5 |############
     74.2 |########################################
     74.8 |##############
     75.5 |##############
     76.1 |#################################
     76.7 |############
     77.4 |#####################
     78.0 |########
     78.7 |##################
     79.3 |##########
     79.9 |##################
     80.6 |##########
     81.2 |##########
     81.9 |##
     82.5 |######
     83.1 |######
     83.8 |########
  (16 below, 8 above range)

ema_neon (n=160, range 0.5-2.4 ns)
      0.5 |
      0.6 |
      0.7 |
      0.8 |########################################
      0.9 |
      1.0 |
      1.1 |
      1.2 |############################
      1.3 |
      1.4 |
      1.5 |
      1.5 |
      1.6 |#########################
      1.7 |
      1.8 |
      1.9 |
      2.0 |##############
      2.1 |
      2.2 |
      2.3 |
  (23 below, 15 above range)

ema_scalar (n=160, range 19.1-29.1 ns)
     19.1 |###########
     19.6 |#############
     20.1 |########
     20.6 |######
     21.1 |###########
     21.6 |########################################
     22.1 |######################
     22.6 |######################
     23.1 |######
     23.6 |###############
     24.1 |#################################
     24.6 |########################
     25.1 |########
     25.6 |#################
     26.1 |####################
     26.6 |###########
     27.1 |########
     27.6 |##
     28.1 |####
     28.6 |####
  (14 below, 14 above range)

```

## Diagnostics

- **ema_neon**: CV=56.7% (high variance, measurements may be unstable)
- **ema_neon**: worst_20/best_20 = 5.3x (possible bimodal distribution)
- **ema_neon**: bridge=558.3% of algo (FFI overhead may distort results)
- **ema_scalar**: bridge=13.9% of algo (FFI overhead may distort results)
