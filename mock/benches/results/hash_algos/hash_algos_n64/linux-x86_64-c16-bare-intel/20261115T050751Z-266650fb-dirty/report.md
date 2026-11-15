# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (78 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 19 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 21% faster than the next best (hash_sip)

hash_xxh (19 ns) leads hash_sip (22 ns) by 21%, a clear separation rather than a photo finish. CV 92.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 76% (significant)

hash_xxh is -59 ns (76%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 4.2x slower than the field

hash_fnv1a (78 ns) is 4.2x the fastest (19 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.2x the fastest

Fastest hash_xxh (19 ns) to slowest hash_fnv1a (78 ns): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_sip is inconsistent: worst-20% is 4.0x its best-20%

hash_sip's best 20% of batches run at 18 ns but its worst 20% at 74 ns (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 18.6 ns median (-76.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.19x (fastest 18.6 ns, slowest 77.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x1111222233334444 |
| Counter frequency | 2112808777 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hash_fnv1a | 2534ns | 1955ns | 1600ns | 1908ns | 5349ns | base |
| hash_sip | 2642ns | 1900ns | 1553ns | 1914ns | 5917ns | +4.26% |
| hash_xxh | 2771ns | 1896ns | 1586ns | 2024ns | 6200ns | +9.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 100ns | 64ns | 210ns | base | 0.638 |
| hash_sip | 32ns | 18ns | 74ns | -67.97% | 1.992 |
| hash_xxh | 27ns | 15ns | 60ns | -73.08% | 2.371 |

## Performance model

- Peak throughput: **4.133 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.822 | 19.9% |
| hash_sip | 2.844 | 68.8% |
| hash_xxh | 3.441 | 83.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2534ns | 2534ns | base |
| hash_sip | 2642ns | 2642ns | +4.26% |
| hash_xxh | 2771ns | 2771ns | +9.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 78ns | base | --- | [78, 78] | --- | --- | --- | --- |
| hash_sip | 22ns | -55.3ns (-71.0%) | [-55, -55]ns | [22, 23] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 19ns | -59.3ns (-76.1%) | [-59, -59]ns | [18, 19] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 78ns | -71.1% | -76.2% |
| 1 | 2 | 64ns | -71.0% | -76.2% |
| 1 | 3 | 64ns | -71.0% | -70.2% |
| 1 | 4 | 78ns | -71.0% | -76.3% |
| 1 | 5 | 78ns | -76.3% | -80.4% |
| 1 | 6 | 64ns | -64.6% | -70.8% |
| 1 | 7 | 78ns | -71.0% | -76.1% |
| 1 | 8 | 78ns | -71.1% | -76.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.141 | ok |
| hash_sip | -0.151 | ok |
| hash_xxh | -0.385 | moderate- |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 30.5ns | 100.3ns | 30.4% | HIGH |
| hash_sip | 32.8ns | 32.1ns | 102.1% | HIGH |
| hash_xxh | 34.6ns | 27.0ns | 128.3% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 63.7-210.1 ns)
     63.7 |#########
     71.0 |########################################
     78.3 |
     85.7 |
     93.0 |
    100.3 |
    107.6 |
    114.9 |
    122.3 |
    129.6 |
    136.9 |
    144.2 |
    151.5 |
    158.9 |
    166.2 |
    173.5 |
    180.8 |#
    188.1 |
    195.5 |
    202.8 |
  (26 below, 18 above range)

hash_sip (n=160, range 18.4-74.1 ns)
     18.4 |#########
     21.2 |########################################
     24.0 |
     26.8 |
     29.6 |
     32.3 |
     35.1 |
     37.9 |
     40.7 |
     43.5 |
     46.3 |
     49.1 |
     51.8 |#
     54.6 |
     57.4 |#
     60.2 |
     63.0 |
     65.8 |#
     68.5 |
     71.3 |#
  (23 below, 12 above range)

hash_xxh (n=160, range 15.5-60.0 ns)
     15.5 |
     17.7 |########################################
     19.9 |
     22.2 |
     24.4 |
     26.6 |
     28.8 |
     31.1 |
     33.3 |
     35.5 |
     37.7 |
     40.0 |
     42.2 |
     44.4 |
     46.6 |
     48.9 |
     51.1 |
     53.3 |#
     55.5 |#
     57.8 |
  (29 below, 19 above range)

```

## Diagnostics

- **hash_fnv1a**: CV=61.4% (high variance, measurements may be unstable)
- **hash_fnv1a**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **hash_fnv1a**: bridge=30.3% of algo (FFI overhead may distort results)
- **hash_sip**: CV=71.8% (high variance, measurements may be unstable)
- **hash_sip**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **hash_sip**: bridge=104.9% of algo (FFI overhead may distort results)
- **hash_xxh**: CV=63.9% (high variance, measurements may be unstable)
- **hash_xxh**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **hash_xxh**: bridge=128.0% of algo (FFI overhead may distort results)
