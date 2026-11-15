# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (1.15 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 190 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh beats baseline by 83% (significant)

hash_xxh is -957 ns (83%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 6.0x slower than the field

hash_fnv1a (1.15 us) is 6.0x the fastest (190 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 87.4%)

hash_xxh wins on median (190 ns) yet has the highest variance (CV 87.4%), while hash_fnv1a is the steadiest (CV 44.7%, 1.15 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 6.0x the fastest

Fastest hash_xxh (190 ns) to slowest hash_fnv1a (1.15 us): 6.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_xxh is inconsistent: worst-20% is 3.4x its best-20%

hash_xxh's best 20% of batches run at 171 ns but its worst 20% at 585 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 190.4 ns median (-83.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 6.03x (fastest 190.4 ns, slowest 1147.6 ns)

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
| hash_fnv1a | 3219ns | 3023ns | 2475ns | 2903ns | 4913ns | base |
| hash_sip | 2991ns | 2088ns | 2078ns | 2293ns | 5999ns | -7.09% |
| hash_xxh | 3152ns | 2073ns | 1863ns | 2496ns | 6409ns | -2.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 1218ns | 939ns | 1851ns | base | 0.841 |
| hash_sip | 281ns | 201ns | 569ns | -76.94% | 3.646 |
| hash_xxh | 289ns | 171ns | 585ns | -76.29% | 3.545 |

## Performance model

- Peak throughput: **5.980 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.892 | 14.9% |
| hash_sip | 5.064 | 84.7% |
| hash_xxh | 5.378 | 89.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 3219ns | 3219ns | base |
| hash_sip | 2991ns | 2991ns | -7.09% |
| hash_xxh | 3152ns | 3152ns | -2.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 1148ns | base | --- | [1148, 1148] | --- | --- | --- | --- |
| hash_sip | 202ns | -899.9ns (-78.4%) | [-945, -900]ns | [202, 203] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 190ns | -957.1ns (-83.4%) | [-957, -796]ns | [190, 190] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 1148ns | -82.4% | -83.4% |
| 1 | 2 | 939ns | -78.5% | -49.8% |
| 1 | 3 | 1148ns | -82.4% | -83.4% |
| 1 | 4 | 1148ns | -78.4% | -83.4% |
| 1 | 5 | 1148ns | -82.3% | -83.4% |
| 1 | 6 | 939ns | -78.6% | -83.5% |
| 1 | 7 | 1148ns | -82.4% | -83.4% |
| 1 | 8 | 939ns | -78.5% | -79.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.475 | moderate- |
| hash_sip | -0.131 | ok |
| hash_xxh | -0.117 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 25.3ns | 1218.3ns | 2.1% |  |
| hash_sip | 34.8ns | 280.9ns | 12.4% | HIGH |
| hash_xxh | 36.0ns | 288.9ns | 12.5% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 938.9-1851.0 ns)
    938.9 |#############
    984.5 |
   1030.1 |
   1075.7 |
   1121.3 |########################################
   1166.9 |
   1212.5 |
   1258.1 |
   1303.7 |
   1349.4 |
   1395.0 |
   1440.6 |
   1486.2 |
   1531.8 |
   1577.4 |
   1623.0 |
   1668.6 |
   1714.2 |
   1759.8 |
   1805.4 |
  (22 below, 13 above range)

hash_sip (n=160, range 200.6-569.1 ns)
    200.6 |########################################
    219.0 |
    237.4 |########
    255.9 |
    274.3 |
    292.7 |
    311.1 |
    329.6 |
    348.0 |
    366.4 |
    384.8 |
    403.3 |
    421.7 |
    440.1 |
    458.5 |
    477.0 |
    495.4 |
    513.8 |
    532.2 |
    550.7 |#
  (12 below, 19 above range)

hash_xxh (n=160, range 171.2-585.5 ns)
    171.2 |########################################
    191.9 |
    212.7 |
    233.4 |
    254.1 |
    274.8 |
    295.5 |
    316.2 |
    336.9 |
    357.6 |
    378.3 |
    399.1 |
    419.8 |
    440.5 |#
    461.2 |
    481.9 |##
    502.6 |#
    523.3 |#
    544.0 |##
    564.7 |#
  (17 below, 14 above range)

```

## Diagnostics

- **hash_fnv1a**: CV=42.1% (high variance, measurements may be unstable)
- **hash_sip**: CV=57.1% (high variance, measurements may be unstable)
- **hash_sip**: bridge=11.8% of algo (FFI overhead may distort results)
- **hash_xxh**: CV=57.6% (high variance, measurements may be unstable)
- **hash_xxh**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **hash_xxh**: bridge=12.4% of algo (FFI overhead may distort results)
