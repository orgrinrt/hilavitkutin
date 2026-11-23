# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (292 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 52 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 13% faster than the next best (hash_sip)

hash_xxh (52 ns) leads hash_sip (58 ns) by 13%, a clear separation rather than a photo finish. CV 89.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 82% (significant)

hash_xxh is -241 ns (82%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 5.7x slower than the field

hash_fnv1a (292 ns) is 5.7x the fastest (52 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.7x the fastest

Fastest hash_xxh (52 ns) to slowest hash_fnv1a (292 ns): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_sip is inconsistent: worst-20% is 3.8x its best-20%

hash_sip's best 20% of batches run at 47 ns but its worst 20% at 182 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 51.6 ns median (-82.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.66x (fastest 51.6 ns, slowest 292.2 ns)

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
| Counter frequency | 2112774913 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hash_fnv1a | 2859ns | 2171ns | 1800ns | 2173ns | 5978ns | base |
| hash_sip | 2687ns | 1935ns | 1582ns | 1909ns | 6125ns | -6.03% |
| hash_xxh | 2707ns | 1929ns | 1578ns | 1971ns | 6044ns | -5.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 383ns | 242ns | 797ns | base | 0.668 |
| hash_sip | 80ns | 47ns | 182ns | -79.03% | 3.187 |
| hash_xxh | 72ns | 42ns | 161ns | -81.10% | 3.537 |

## Performance model

- Peak throughput: **6.053 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.876 | 14.5% |
| hash_sip | 4.406 | 72.8% |
| hash_xxh | 4.961 | 82.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2859ns | 2859ns | base |
| hash_sip | 2687ns | 2687ns | -6.03% |
| hash_xxh | 2707ns | 2707ns | -5.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 292ns | base | --- | [292, 292] | --- | --- | --- | --- |
| hash_sip | 58ns | -234.1ns (-80.1%) | [-234, -234]ns | [58, 58] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 52ns | -240.6ns (-82.3%) | [-241, -241]ns | [52, 52] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 292ns | -80.0% | -82.3% |
| 1 | 2 | 239ns | -75.7% | -78.4% |
| 1 | 3 | 292ns | -80.1% | -85.5% |
| 1 | 4 | 239ns | -80.0% | -82.2% |
| 1 | 5 | 292ns | -79.9% | -85.5% |
| 1 | 6 | 292ns | -80.0% | -82.3% |
| 1 | 7 | 292ns | -83.7% | -82.3% |
| 1 | 8 | 292ns | -83.6% | -82.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.373 | moderate- |
| hash_sip | 0.044 | ok |
| hash_xxh | 0.378 | moderate+ |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 31.5ns | 383.1ns | 8.2% | HIGH |
| hash_sip | 32.9ns | 80.3ns | 41.0% | HIGH |
| hash_xxh | 33.0ns | 72.4ns | 45.6% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 242.0-796.9 ns)
    242.0 |
    269.8 |########################################
    297.5 |#
    325.2 |
    353.0 |
    380.7 |
    408.5 |
    436.2 |
    464.0 |
    491.7 |
    519.4 |
    547.2 |
    574.9 |
    602.7 |
    630.4 |
    658.1 |
    685.9 |
    713.6 |
    741.4 |#
    769.1 |
  (30 below, 21 above range)

hash_sip (n=160, range 47.3-181.7 ns)
     47.3 |#################
     54.0 |########################################
     60.7 |
     67.5 |#
     74.2 |
     80.9 |
     87.6 |
     94.3 |
    101.1 |
    107.8 |
    114.5 |
    121.2 |
    127.9 |
    134.6 |#
    141.4 |
    148.1 |
    154.8 |
    161.5 |#
    168.2 |
    174.9 |#
  (14 below, 19 above range)

hash_xxh (n=160, range 42.3-160.8 ns)
     42.3 |###############
     48.2 |########################################
     54.1 |
     60.1 |
     66.0 |#
     71.9 |
     77.9 |
     83.8 |
     89.7 |
     95.6 |
    101.6 |
    107.5 |
    113.4 |
    119.4 |
    125.3 |
    131.2 |##
    137.1 |#
    143.1 |#
    149.0 |###
    154.9 |##
  (13 below, 13 above range)

```

## Diagnostics

- **hash_fnv1a**: CV=61.2% (high variance, measurements may be unstable)
- **hash_fnv1a**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **hash_sip**: CV=66.3% (high variance, measurements may be unstable)
- **hash_sip**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **hash_sip**: bridge=41.0% of algo (FFI overhead may distort results)
- **hash_xxh**: CV=63.5% (high variance, measurements may be unstable)
- **hash_xxh**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **hash_xxh**: bridge=45.7% of algo (FFI overhead may distort results)
