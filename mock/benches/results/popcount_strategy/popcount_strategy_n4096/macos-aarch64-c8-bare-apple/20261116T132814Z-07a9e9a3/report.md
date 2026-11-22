# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (512 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 86 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 488% faster than the next best (popcount_swar)

popcount_std (86 ns) leads popcount_swar (507 ns) by 488%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 83% (significant)

popcount_std is -426 ns (83%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 5.9x slower than the field

popcount_handrolled (512 ns) is 5.9x the fastest (86 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 5.7%)

popcount_std wins on median (86 ns) yet has the highest variance (CV 5.7%), while popcount_swar is the steadiest (CV 0.9%, 507 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### popcount_swar shows warm-up / thermal drift (autocorr +0.51)

popcount_swar's per-pass series has lag-1 autocorrelation +0.51, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.9x the fastest

Fastest popcount_std (86 ns) to slowest popcount_handrolled (512 ns): 5.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_swar's edge over baseline is significant but tiny (-5 ns, 0.98%)

popcount_swar differs from baseline popcount_handrolled by -5 ns (0.98%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: popcount_std** at 86.2 ns median (-83.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.94x (fastest 86.2 ns, slowest 512.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Popcount lowering: std count_ones vs Kernighan loop vs SWAR |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x24680123456789ab |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| popcount_handrolled | 2589ns | 2584ns | 2579ns | 2585ns | 2612ns | base |
| popcount_std | 2164ns | 2158ns | 2153ns | 2159ns | 2190ns | -16.42% |
| popcount_swar | 2583ns | 2580ns | 2575ns | 2580ns | 2599ns | -0.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 515ns | 507ns | 530ns | base | 7.955 |
| popcount_std | 87ns | 80ns | 94ns | -83.19% | 47.329 |
| popcount_swar | 508ns | 502ns | 516ns | -1.32% | 8.061 |

## Performance model

- Peak throughput: **51.040 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 7.998 | 15.7% |
| popcount_std | 47.517 | 93.1% |
| popcount_swar | 8.077 | 15.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2589ns | 2589ns | base |
| popcount_std | 2164ns | 2164ns | -16.42% |
| popcount_swar | 2583ns | 2583ns | -0.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 512ns | base | --- | [511, 513] | --- | --- | --- | --- |
| popcount_std | 86ns | -426.2ns (-83.2%) | [-427, -425]ns | [85, 87] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 507ns | -5.0ns (-1.0%) | [-6, -5]ns | [506, 508] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 512ns | -82.8% | -0.9% |
| 1 | 2 | 513ns | -83.3% | -1.0% |
| 1 | 3 | 513ns | -83.1% | -1.0% |
| 1 | 4 | 511ns | -83.1% | -0.7% |
| 1 | 5 | 512ns | -83.3% | -1.1% |
| 1 | 6 | 511ns | -83.1% | -0.9% |
| 1 | 7 | 514ns | -83.1% | -1.3% |
| 1 | 8 | 511ns | -83.5% | -1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.500 | moderate- |
| popcount_std | -0.221 | moderate- |
| popcount_swar | 0.510 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.0ns | 514.9ns | 0.6% |  |
| popcount_std | 2.2ns | 86.5ns | 2.6% |  |
| popcount_swar | 2.9ns | 508.1ns | 0.6% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 506.7-530.3 ns)
    506.7 |##########
    507.9 |############################
    509.1 |########################################
    510.2 |####################################
    511.4 |####################
    512.6 |############################
    513.8 |########################
    515.0 |################
    516.1 |################
    517.3 |############
    518.5 |##################
    519.7 |##########
    520.9 |########
    522.1 |######
    523.2 |
    524.4 |####
    525.6 |######
    526.8 |
    528.0 |
    529.1 |
  (16 below, 3 above range)

popcount_std (n=160, range 80.2-93.8 ns)
     80.2 |########
     80.9 |##
     81.6 |############################
     82.3 |########################################
     83.0 |########
     83.6 |#########################
     84.3 |########
     85.0 |##################################
     85.7 |###############################
     86.3 |##############
     87.0 |####################
     87.7 |##################################
     88.4 |############################
     89.1 |#########################
     89.7 |#################
     90.4 |#####
     91.1 |##############
     91.8 |##
     92.4 |###########
     93.1 |########
  (17 below, 13 above range)

popcount_swar (n=160, range 502.4-515.6 ns)
    502.4 |#######################
    503.1 |###########################
    503.8 |####
    504.4 |################
    505.1 |################
    505.7 |########################################
    506.4 |##########
    507.0 |##################
    507.7 |##################
    508.3 |########
    509.0 |##############
    509.7 |################
    510.3 |################
    511.0 |####
    511.6 |##############
    512.3 |######
    512.9 |##
    513.6 |##############
    514.2 |####
    514.9 |######
  (11 below, 13 above range)

```

## Diagnostics

- **popcount_swar**: autocorrelation=0.51 (measurement drift or warm-up artifact)
