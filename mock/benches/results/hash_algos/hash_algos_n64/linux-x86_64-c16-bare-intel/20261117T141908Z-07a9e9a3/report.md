# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (78 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 18 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 22% faster than the next best (hash_sip)

hash_xxh (18 ns) leads hash_sip (22 ns) by 22%, a clear separation rather than a photo finish. CV 88.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 76% (significant)

hash_xxh is -59 ns (76%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 4.2x slower than the field

hash_fnv1a (78 ns) is 4.2x the fastest (18 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 88.7%)

hash_xxh wins on median (18 ns) yet has the highest variance (CV 88.7%), while hash_fnv1a is the steadiest (CV 85.2%, 78 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.2x the fastest

Fastest hash_xxh (18 ns) to slowest hash_fnv1a (78 ns): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_xxh is inconsistent: worst-20% is 3.7x its best-20%

hash_xxh's best 20% of batches run at 15 ns but its worst 20% at 56 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 18.5 ns median (-76.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.21x (fastest 18.5 ns, slowest 77.9 ns)

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
| Counter frequency | 2112768575 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hash_fnv1a | 2549ns | 1961ns | 1604ns | 1858ns | 5565ns | base |
| hash_sip | 2594ns | 1906ns | 1559ns | 1864ns | 5818ns | +1.76% |
| hash_xxh | 2635ns | 1903ns | 1554ns | 1926ns | 5843ns | +3.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 101ns | 64ns | 220ns | base | 0.634 |
| hash_sip | 30ns | 18ns | 68ns | -69.84% | 2.102 |
| hash_xxh | 26ns | 15ns | 56ns | -74.58% | 2.494 |

## Performance model

- Peak throughput: **4.224 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.822 | 19.4% |
| hash_sip | 2.844 | 67.3% |
| hash_xxh | 3.459 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2549ns | 2549ns | base |
| hash_sip | 2594ns | 2594ns | +1.76% |
| hash_xxh | 2635ns | 2635ns | +3.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 78ns | base | --- | [78, 78] | --- | --- | --- | --- |
| hash_sip | 22ns | -55.3ns (-71.0%) | [-55, -47]ns | [22, 22] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 18ns | -59.3ns (-76.1%) | [-59, -56]ns | [18, 19] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 78ns | -71.0% | -80.5% |
| 1 | 2 | 64ns | -71.0% | -76.0% |
| 1 | 3 | 64ns | -71.0% | -70.8% |
| 1 | 4 | 78ns | -76.3% | -80.5% |
| 1 | 5 | 78ns | -71.1% | -76.1% |
| 1 | 6 | 64ns | -64.6% | -76.0% |
| 1 | 7 | 64ns | -64.7% | -70.8% |
| 1 | 8 | 78ns | -76.3% | -76.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.126 | ok |
| hash_sip | 0.122 | ok |
| hash_xxh | -0.371 | moderate- |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 30.6ns | 101.0ns | 30.3% | HIGH |
| hash_sip | 32.2ns | 30.5ns | 105.8% | HIGH |
| hash_xxh | 33.0ns | 25.7ns | 128.7% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 63.7-219.5 ns)
     63.7 |###############################
     71.5 |########################################
     79.3 |
     87.1 |
     94.9 |
    102.6 |#
    110.4 |
    118.2 |
    126.0 |
    133.8 |
    141.6 |
    149.4 |
    157.2 |
    165.0 |
    172.8 |
    180.6 |
    188.3 |
    196.1 |#
    203.9 |
    211.7 |##
  (4 below, 17 above range)

hash_sip (n=160, range 18.4-67.9 ns)
     18.4 |#################################
     20.9 |########################################
     23.4 |
     25.8 |
     28.3 |
     30.8 |#
     33.3 |
     35.7 |
     38.2 |
     40.7 |
     43.2 |
     45.6 |
     48.1 |
     50.6 |
     53.1 |#
     55.5 |
     58.0 |#
     60.5 |#
     63.0 |#
     65.4 |##
  (1 below, 14 above range)

hash_xxh (n=160, range 15.1-56.4 ns)
     15.1 |#########################
     17.2 |########################################
     19.3 |
     21.3 |
     23.4 |
     25.5 |
     27.5 |
     29.6 |
     31.7 |
     33.7 |
     35.8 |
     37.8 |
     39.9 |
     42.0 |#
     44.0 |##
     46.1 |#
     48.2 |#
     50.2 |#
     52.3 |
     54.3 |###
  (16 below, 13 above range)

```

## Diagnostics

- **hash_fnv1a**: CV=65.8% (high variance, measurements may be unstable)
- **hash_fnv1a**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **hash_fnv1a**: bridge=30.3% of algo (FFI overhead may distort results)
- **hash_sip**: CV=64.1% (high variance, measurements may be unstable)
- **hash_sip**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **hash_sip**: bridge=104.9% of algo (FFI overhead may distort results)
- **hash_xxh**: CV=63.9% (high variance, measurements may be unstable)
- **hash_xxh**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **hash_xxh**: bridge=128.6% of algo (FFI overhead may distort results)
