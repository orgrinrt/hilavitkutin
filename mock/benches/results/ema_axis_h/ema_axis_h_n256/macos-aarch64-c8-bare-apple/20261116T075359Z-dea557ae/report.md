# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (79 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_neon at 1 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_neon dominates: 1425% faster than the next best (ema_scalar)

ema_neon (1 ns) leads ema_scalar (18 ns) by 1425%, a clear separation rather than a photo finish. CV 60.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_neon beats baseline by 98% (significant)

ema_neon is -77 ns (98%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec is an outlier: 65.6x slower than the field

ema_autovec (79 ns) is 65.6x the fastest (1 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_neon is fastest but the noisiest (CV 60.6%)

ema_neon wins on median (1 ns) yet has the highest variance (CV 60.6%), while ema_autovec is the steadiest (CV 11.8%, 79 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 65.6x the fastest

Fastest ema_neon (1 ns) to slowest ema_autovec (79 ns): 65.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ema_neon is inconsistent: worst-20% is 7.0x its best-20%

ema_neon's best 20% of batches run at 0 ns but its worst 20% at 2 ns (7.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ema_neon** at 1.2 ns median (-98.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 65.58x (fastest 1.2 ns, slowest 78.7 ns)

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
| ema_autovec | 2177ns | 2172ns | 2147ns | 2173ns | 2218ns | base |
| ema_neon | 2116ns | 2119ns | 2077ns | 2108ns | 2180ns | -2.78% |
| ema_scalar | 2136ns | 2131ns | 2087ns | 2131ns | 2201ns | -1.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 80ns | 74ns | 91ns | base | 3.202 |
| ema_neon | 1ns | 0ns | 2ns | -98.33% | 192.300 |
| ema_scalar | 18ns | 15ns | 22ns | -76.93% | 13.877 |

## Performance model

- Peak throughput: **758.519 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 3.253 | 0.4% |
| ema_neon | 213.333 | 28.1% |
| ema_scalar | 13.989 | 1.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2177ns | 2177ns | base |
| ema_neon | 2116ns | 2116ns | -2.78% |
| ema_scalar | 2136ns | 2136ns | -1.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 79ns | base | --- | [78, 80] | --- | --- | --- | --- |
| ema_neon | 1ns | -77.3ns (-98.2%) | [-78, -76]ns | [1, 2] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 18ns | -60.4ns (-76.7%) | [-61, -60]ns | [18, 19] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 79ns | -98.5% | -77.3% |
| 1 | 2 | 80ns | -98.5% | -77.1% |
| 1 | 3 | 80ns | -98.5% | -76.4% |
| 1 | 4 | 78ns | -98.5% | -76.8% |
| 1 | 5 | 77ns | -99.0% | -77.8% |
| 1 | 6 | 79ns | -97.8% | -76.5% |
| 1 | 7 | 78ns | -98.1% | -75.5% |
| 1 | 8 | 80ns | -97.9% | -75.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.021 | ok |
| ema_neon | 0.002 | ok |
| ema_scalar | 0.205 | moderate+ |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 3.4ns | 80.0ns | 4.3% |  |
| ema_neon | 6.2ns | 1.3ns | 465.0% | HIGH |
| ema_scalar | 3.5ns | 18.4ns | 19.0% | HIGH |

## Distribution (algo ns)

```
ema_autovec (n=160, range 73.5-90.6 ns)
     73.5 |###########
     74.4 |#########################
     75.2 |############################
     76.1 |#####################
     76.9 |##############################
     77.8 |#########################
     78.7 |##############################
     79.5 |########################################
     80.4 |########################################
     81.2 |#########
     82.1 |############################
     82.9 |##
     83.8 |##
     84.6 |#########
     85.5 |####
     86.3 |
     87.2 |####
     88.0 |####
     88.9 |####
     89.7 |####
  (16 below, 4 above range)

ema_neon (n=160, range 0.3-2.4 ns)
      0.3 |########################
      0.4 |
      0.5 |
      0.6 |
      0.7 |###############################
      0.8 |
      0.9 |
      1.0 |
      1.1 |########################################
      1.2 |
      1.3 |
      1.4 |
      1.5 |
      1.6 |#####################################
      1.7 |
      1.8 |
      1.9 |
      2.0 |###########################
      2.1 |
      2.2 |
  (8 below, 15 above range)

ema_scalar (n=160, range 15.1-22.3 ns)
     15.1 |###########
     15.5 |############################
     15.8 |
     16.2 |#################
     16.6 |#########################
     16.9 |##################################
     17.3 |#########################
     17.6 |############################
     18.0 |########################################
     18.3 |
     18.7 |#####################################
     19.1 |########################################
     19.4 |######################
     19.8 |##############
     20.1 |#########################
     20.5 |###########
     20.9 |########
     21.2 |
     21.6 |#################
     21.9 |###########
  (13 below, 7 above range)

```

## Diagnostics

- **ema_neon**: CV=54.6% (high variance, measurements may be unstable)
- **ema_neon**: worst_20/best_20 = 7.0x (possible bimodal distribution)
- **ema_neon**: bridge=516.7% of algo (FFI overhead may distort results)
- **ema_scalar**: bridge=18.0% of algo (FFI overhead may distort results)
