# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (292 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 52 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 13% faster than the next best (hash_sip)

hash_xxh (52 ns) leads hash_sip (58 ns) by 13%, a clear separation rather than a photo finish. CV 88.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 82% (significant)

hash_xxh is -241 ns (82%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 5.7x slower than the field

hash_fnv1a (292 ns) is 5.7x the fastest (52 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 88.9%)

hash_xxh wins on median (52 ns) yet has the highest variance (CV 88.9%), while hash_fnv1a is the steadiest (CV 73.1%, 292 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.7x the fastest

Fastest hash_xxh (52 ns) to slowest hash_fnv1a (292 ns): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_sip is inconsistent: worst-20% is 3.8x its best-20%

hash_sip's best 20% of batches run at 48 ns but its worst 20% at 179 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 51.7 ns median (-82.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.65x (fastest 51.7 ns, slowest 292.3 ns)

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
| hash_fnv1a | 2861ns | 2175ns | 1779ns | 2303ns | 5620ns | base |
| hash_sip | 2720ns | 1937ns | 1586ns | 1979ns | 6078ns | -4.92% |
| hash_xxh | 2750ns | 1930ns | 1772ns | 1992ns | 6005ns | -3.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 386ns | 239ns | 756ns | base | 0.663 |
| hash_sip | 81ns | 48ns | 179ns | -79.04% | 3.164 |
| hash_xxh | 74ns | 47ns | 162ns | -80.83% | 3.459 |

## Performance model

- Peak throughput: **5.389 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.876 | 16.3% |
| hash_sip | 4.399 | 81.6% |
| hash_xxh | 4.952 | 91.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2861ns | 2861ns | base |
| hash_sip | 2720ns | 2720ns | -4.92% |
| hash_xxh | 2750ns | 2750ns | -3.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 292ns | base | --- | [292, 292] | --- | --- | --- | --- |
| hash_sip | 58ns | -234.2ns (-80.1%) | [-235, -234]ns | [58, 58] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 52ns | -240.6ns (-82.3%) | [-241, -240]ns | [52, 52] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 483ns | -87.9% | -89.3% |
| 1 | 2 | 292ns | -80.0% | -82.3% |
| 1 | 3 | 292ns | -80.1% | -82.3% |
| 1 | 4 | 292ns | -80.0% | -82.3% |
| 1 | 5 | 293ns | -83.7% | -82.3% |
| 1 | 6 | 239ns | -80.0% | -78.3% |
| 1 | 7 | 239ns | -75.6% | -82.3% |
| 1 | 8 | 292ns | -80.0% | -82.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | 0.093 | ok |
| hash_sip | 0.278 | moderate+ |
| hash_xxh | -0.175 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 31.3ns | 386.0ns | 8.1% | HIGH |
| hash_sip | 33.2ns | 80.9ns | 41.1% | HIGH |
| hash_xxh | 33.5ns | 74.0ns | 45.2% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 239.1-755.8 ns)
    239.1 |#######
    265.0 |
    290.8 |########################################
    316.6 |
    342.5 |
    368.3 |
    394.1 |
    420.0 |
    445.8 |
    471.6 |#########
    497.5 |
    523.3 |
    549.1 |
    575.0 |
    600.8 |
    626.6 |
    652.5 |
    678.3 |
    704.1 |
    730.0 |
  (19 below, 20 above range)

hash_sip (n=160, range 47.5-179.2 ns)
     47.5 |########
     54.1 |########################################
     60.7 |
     67.3 |
     73.9 |
     80.4 |
     87.0 |
     93.6 |
    100.2 |
    106.8 |
    113.3 |
    119.9 |
    126.5 |
    133.1 |
    139.7 |
    146.3 |
    152.8 |
    159.4 |#
    166.0 |
    172.6 |##
  (13 below, 17 above range)

hash_xxh (n=160, range 47.5-161.6 ns)
     47.5 |########################################
     53.2 |
     58.9 |
     64.6 |
     70.3 |
     76.0 |
     81.7 |
     87.4 |
     93.1 |
     98.8 |
    104.6 |
    110.3 |
    116.0 |
    121.7 |#
    127.4 |
    133.1 |
    138.8 |#
    144.5 |#
    150.2 |#
    155.9 |
  (14 below, 14 above range)

```

## Diagnostics

- **hash_fnv1a**: CV=55.4% (high variance, measurements may be unstable)
- **hash_fnv1a**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **hash_sip**: CV=63.0% (high variance, measurements may be unstable)
- **hash_sip**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **hash_sip**: bridge=40.9% of algo (FFI overhead may distort results)
- **hash_xxh**: CV=62.1% (high variance, measurements may be unstable)
- **hash_xxh**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **hash_xxh**: bridge=45.6% of algo (FFI overhead may distort results)
