# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (78 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 19 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 22% faster than the next best (hash_sip)

hash_xxh (19 ns) leads hash_sip (23 ns) by 22%, a clear separation rather than a photo finish. CV 88.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_sip beats baseline by 71% (significant)

hash_sip is -55 ns (71%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 4.2x slower than the field

hash_fnv1a (78 ns) is 4.2x the fastest (19 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.2x the fastest

Fastest hash_xxh (19 ns) to slowest hash_fnv1a (78 ns): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_fnv1a is inconsistent: worst-20% is 3.7x its best-20%

hash_fnv1a's best 20% of batches run at 64 ns but its worst 20% at 236 ns (3.7x) - a bimodal or bursty profile the median hides.

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
| Counter frequency | 2112774913 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hash_fnv1a | 2724ns | 1952ns | 1597ns | 2013ns | 5984ns | base |
| hash_sip | 2795ns | 1902ns | 1737ns | 2051ns | 6084ns | +2.62% |
| hash_xxh | 2928ns | 1900ns | 1596ns | 2372ns | 5930ns | +7.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 108ns | 64ns | 236ns | base | 0.591 |
| hash_sip | 33ns | 21ns | 72ns | -69.40% | 1.931 |
| hash_xxh | 28ns | 16ns | 57ns | -73.76% | 2.251 |

## Performance model

- Peak throughput: **4.107 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.822 | 20.0% |
| hash_sip | 2.832 | 69.0% |
| hash_xxh | 3.441 | 83.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2724ns | 2724ns | base |
| hash_sip | 2795ns | 2795ns | +2.62% |
| hash_xxh | 2928ns | 2928ns | +7.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 78ns | base | --- | [64, 78] | --- | --- | --- | --- |
| hash_sip | 23ns | -55.2ns (-70.9%) | [-55, -45]ns | [23, 23] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 19ns | -48.7ns (-62.5%) | [-59, -49]ns | [19, 19] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 78ns | -71.0% | -76.1% |
| 1 | 2 | 64ns | -64.6% | -76.1% |
| 1 | 3 | 64ns | -71.0% | -76.2% |
| 1 | 4 | 64ns | -64.6% | -70.8% |
| 1 | 5 | 78ns | -71.0% | -46.9% |
| 1 | 6 | 64ns | -64.7% | -70.8% |
| 1 | 7 | 78ns | -71.0% | -76.3% |
| 1 | 8 | 64ns | -64.6% | -70.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.475 | moderate- |
| hash_sip | -0.165 | ok |
| hash_xxh | -0.048 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 32.7ns | 108.3ns | 30.2% | HIGH |
| hash_sip | 34.8ns | 33.2ns | 104.8% | HIGH |
| hash_xxh | 36.7ns | 28.4ns | 129.1% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 63.7-236.3 ns)
     63.7 |########################################
     72.3 |############################
     81.0 |
     89.6 |
     98.2 |
    106.8 |
    115.5 |
    124.1 |
    132.7 |
    141.4 |
    150.0 |
    158.6 |
    167.3 |
    175.9 |
    184.5 |#
    193.2 |###
    201.8 |#
    210.4 |##
    219.1 |##
    227.7 |#
  (3 below, 13 above range)

hash_sip (n=160, range 20.6-72.3 ns)
     20.6 |########################################
     23.2 |
     25.8 |
     28.3 |
     30.9 |
     33.5 |
     36.1 |
     38.7 |#
     41.3 |
     43.8 |
     46.4 |
     49.0 |
     51.6 |
     54.2 |
     56.8 |#
     59.3 |
     61.9 |#
     64.5 |
     67.1 |#
     69.7 |
  (15 below, 16 above range)

hash_xxh (n=160, range 15.6-57.1 ns)
     15.6 |
     17.7 |########################################
     19.7 |
     21.8 |
     23.9 |#
     26.0 |
     28.0 |#
     30.1 |
     32.2 |#
     34.2 |
     36.3 |#
     38.4 |
     40.5 |#
     42.5 |##
     44.6 |#
     46.7 |#
     48.8 |#
     50.8 |####
     52.9 |#
     55.0 |###
  (28 below, 12 above range)

```

## Diagnostics

- **hash_fnv1a**: CV=64.3% (high variance, measurements may be unstable)
- **hash_fnv1a**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **hash_fnv1a**: bridge=30.2% of algo (FFI overhead may distort results)
- **hash_sip**: CV=62.4% (high variance, measurements may be unstable)
- **hash_sip**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **hash_sip**: bridge=104.4% of algo (FFI overhead may distort results)
- **hash_xxh**: CV=57.7% (high variance, measurements may be unstable)
- **hash_xxh**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **hash_xxh**: bridge=128.5% of algo (FFI overhead may distort results)
