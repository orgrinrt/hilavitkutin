# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (392 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_neon at 45 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_neon dominates: 109% faster than the next best (ema_scalar)

ema_neon (45 ns) leads ema_scalar (95 ns) by 109%, a clear separation rather than a photo finish. CV 11.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_neon beats baseline by 89% (significant)

ema_neon is -349 ns (89%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec is an outlier: 8.6x slower than the field

ema_autovec (392 ns) is 8.6x the fastest (45 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_neon is fastest but the noisiest (CV 11.5%)

ema_neon wins on median (45 ns) yet has the highest variance (CV 11.5%), while ema_autovec is the steadiest (CV 6.3%, 392 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 8.6x the fastest

Fastest ema_neon (45 ns) to slowest ema_autovec (392 ns): 8.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ema_neon** at 45.4 ns median (-88.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 8.65x (fastest 45.4 ns, slowest 392.5 ns)

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
| ema_autovec | 2595ns | 2525ns | 2457ns | 2555ns | 2852ns | base |
| ema_neon | 2325ns | 2295ns | 2133ns | 2340ns | 2472ns | -10.40% |
| ema_scalar | 2416ns | 2483ns | 2219ns | 2450ns | 2512ns | -6.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 401ns | 376ns | 443ns | base | 2.551 |
| ema_neon | 46ns | 38ns | 53ns | -88.66% | 22.502 |
| ema_scalar | 96ns | 85ns | 107ns | -76.17% | 10.707 |

## Performance model

- Peak throughput: **26.756 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 2.609 | 9.8% |
| ema_neon | 22.555 | 84.3% |
| ema_scalar | 10.779 | 40.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 2595ns | 2595ns | base |
| ema_neon | 2325ns | 2325ns | -10.40% |
| ema_scalar | 2416ns | 2416ns | -6.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 392ns | base | --- | [386, 401] | --- | --- | --- | --- |
| ema_neon | 45ns | -349.1ns (-89.0%) | [-354, -339]ns | [44, 46] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 95ns | -298.7ns (-76.1%) | [-310, -292]ns | [94, 97] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 406ns | -89.2% | -76.7% |
| 1 | 2 | 382ns | -87.6% | -74.5% |
| 1 | 3 | 380ns | -87.9% | -74.4% |
| 1 | 4 | 399ns | -88.8% | -78.3% |
| 1 | 5 | 403ns | -89.5% | -77.1% |
| 1 | 6 | 439ns | -89.8% | -77.4% |
| 1 | 7 | 415ns | -88.7% | -76.5% |
| 1 | 8 | 381ns | -87.1% | -74.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | 0.240 | moderate+ |
| ema_neon | 0.212 | moderate+ |
| ema_scalar | 0.123 | ok |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 2.7ns | 401.3ns | 0.7% |  |
| ema_neon | 2.7ns | 45.5ns | 5.8% | HIGH |
| ema_scalar | 2.5ns | 95.6ns | 2.6% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 376.2-442.8 ns)
    376.2 |#################################
    379.6 |########################################
    382.9 |#################################
    386.2 |############
    389.6 |############
    392.9 |######
    396.2 |##########
    399.5 |############
    402.9 |#####################
    406.2 |##################
    409.5 |##
    412.9 |############
    416.2 |##
    419.5 |
    422.8 |##
    426.2 |######
    429.5 |######
    432.8 |####
    436.2 |##############
    439.5 |#####################
  (18 below, 12 above range)

ema_neon (n=160, range 38.3-53.0 ns)
     38.3 |##########
     39.0 |#######################
     39.7 |####################
     40.5 |######
     41.2 |#############
     42.0 |########################################
     42.7 |##############################
     43.4 |##############################
     44.2 |################
     44.9 |#################################
     45.7 |##############################
     46.4 |########################################
     47.1 |#############
     47.9 |####################
     48.6 |####################
     49.3 |##########################
     50.1 |#######################
     50.8 |######
     51.6 |####################
     52.3 |######
  (16 below, 15 above range)

ema_scalar (n=160, range 84.7-107.4 ns)
     84.7 |#####
     85.8 |
     87.0 |###############
     88.1 |#################
     89.2 |############
     90.4 |###############
     91.5 |#####################################
     92.6 |############
     93.8 |########################################
     94.9 |############
     96.0 |######################
     97.2 |###########################
     98.3 |##############################
     99.4 |##############################
    100.6 |##########
    101.7 |######################
    102.8 |############
    104.0 |#####
    105.1 |#####
    106.2 |
  (17 below, 10 above range)

```
