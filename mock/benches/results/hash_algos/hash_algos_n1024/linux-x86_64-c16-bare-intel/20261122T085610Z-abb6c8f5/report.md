# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (948 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_sip at 165 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_sip dominates: 15% faster than the next best (hash_xxh)

hash_sip (165 ns) leads hash_xxh (189 ns) by 15%, a clear separation rather than a photo finish. CV 98.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 83% (significant)

hash_xxh is -784 ns (83%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 5.7x slower than the field

hash_fnv1a (948 ns) is 5.7x the fastest (165 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_sip is fastest but the noisiest (CV 98.0%)

hash_sip wins on median (165 ns) yet has the highest variance (CV 98.0%), while hash_fnv1a is the steadiest (CV 43.0%, 948 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.7x the fastest

Fastest hash_sip (165 ns) to slowest hash_fnv1a (948 ns): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_xxh is inconsistent: worst-20% is 3.4x its best-20%

hash_xxh's best 20% of batches run at 155 ns but its worst 20% at 526 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_sip** at 165.1 ns median (-82.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.74x (fastest 165.1 ns, slowest 948.2 ns)

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
| hash_fnv1a | 2937ns | 2492ns | 2473ns | 2694ns | 4132ns | base |
| hash_sip | 2474ns | 1706ns | 1697ns | 1760ns | 5392ns | -15.78% |
| hash_xxh | 2705ns | 2069ns | 1692ns | 2017ns | 5783ns | -7.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 1111ns | 939ns | 1559ns | base | 0.922 |
| hash_sip | 239ns | 164ns | 519ns | -78.52% | 4.290 |
| hash_xxh | 247ns | 155ns | 526ns | -77.77% | 4.145 |

## Performance model

- Peak throughput: **6.617 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 1.080 | 16.3% |
| hash_sip | 6.200 | 93.7% |
| hash_xxh | 5.409 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2937ns | 2937ns | base |
| hash_sip | 2474ns | 2474ns | -15.78% |
| hash_xxh | 2705ns | 2705ns | -7.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 948ns | base | --- | [939, 1067] | --- | --- | --- | --- |
| hash_sip | 165ns | -775.1ns (-81.7%) | [-783, -775]ns | [164, 166] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 189ns | -784.1ns (-82.7%) | [-785, -759]ns | [189, 190] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 1148ns | -85.5% | -83.5% |
| 1 | 2 | 939ns | -82.5% | -79.7% |
| 1 | 3 | 939ns | -82.5% | -79.7% |
| 1 | 4 | 1148ns | -85.7% | -86.4% |
| 1 | 5 | 939ns | -82.2% | -83.5% |
| 1 | 6 | 939ns | -78.6% | -79.8% |
| 1 | 7 | 1148ns | -85.5% | -83.4% |
| 1 | 8 | 939ns | -82.4% | -83.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.475 | moderate- |
| hash_sip | -0.076 | ok |
| hash_xxh | 0.059 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 23.1ns | 1111.2ns | 2.1% |  |
| hash_sip | 28.4ns | 238.7ns | 11.9% | HIGH |
| hash_xxh | 30.9ns | 247.0ns | 12.5% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 938.9-1559.2 ns)
    938.9 |########################################
    969.9 |#
   1000.9 |
   1032.0 |#
   1063.0 |
   1094.0 |
   1125.0 |################################
   1156.0 |###
   1187.0 |
   1218.0 |
   1249.1 |
   1280.1 |
   1311.1 |
   1342.1 |
   1373.1 |
   1404.1 |
   1435.1 |
   1466.2 |
   1497.2 |
   1528.2 |
  (24 below, 8 above range)

hash_sip (n=160, range 163.9-518.6 ns)
    163.9 |########################################
    181.6 |
    199.3 |#######
    217.1 |
    234.8 |
    252.5 |
    270.3 |
    288.0 |
    305.7 |
    323.5 |
    341.2 |
    358.9 |
    376.7 |
    394.4 |
    412.1 |
    429.9 |
    447.6 |
    465.3 |
    483.1 |
    500.8 |
  (14 below, 19 above range)

hash_xxh (n=160, range 154.8-526.5 ns)
    154.8 |###########
    173.3 |########################################
    191.9 |
    210.5 |
    229.1 |
    247.7 |
    266.3 |
    284.9 |
    303.4 |
    322.0 |
    340.6 |
    359.2 |
    377.8 |
    396.4 |
    414.9 |
    433.5 |
    452.1 |
    470.7 |#
    489.3 |
    507.9 |
  (20 below, 19 above range)

```

## Diagnostics

- **hash_fnv1a**: CV=36.7% (high variance, measurements may be unstable)
- **hash_sip**: CV=67.8% (high variance, measurements may be unstable)
- **hash_sip**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **hash_sip**: bridge=11.8% of algo (FFI overhead may distort results)
- **hash_xxh**: CV=65.7% (high variance, measurements may be unstable)
- **hash_xxh**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **hash_xxh**: bridge=12.5% of algo (FFI overhead may distort results)
