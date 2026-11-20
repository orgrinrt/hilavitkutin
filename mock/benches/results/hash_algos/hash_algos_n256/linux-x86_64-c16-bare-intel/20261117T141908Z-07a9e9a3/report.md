# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (292 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 52 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 12% faster than the next best (hash_sip)

hash_xxh (52 ns) leads hash_sip (58 ns) by 12%, a clear separation rather than a photo finish. CV 89.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_sip beats baseline by 84% (significant)

hash_sip is -244 ns (84%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 5.7x slower than the field

hash_fnv1a (292 ns) is 5.7x the fastest (52 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 89.9%)

hash_xxh wins on median (52 ns) yet has the highest variance (CV 89.9%), while hash_fnv1a is the steadiest (CV 74.4%, 292 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### hash_xxh shows alternating (throttle bounce) (autocorr -0.68)

hash_xxh's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.7x the fastest

Fastest hash_xxh (52 ns) to slowest hash_fnv1a (292 ns): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_xxh is inconsistent: worst-20% is 3.8x its best-20%

hash_xxh's best 20% of batches run at 42 ns but its worst 20% at 161 ns (3.8x) - a bimodal or bursty profile the median hides.

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
| Counter frequency | 2112768575 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hash_fnv1a | 2880ns | 2177ns | 1780ns | 2235ns | 5913ns | base |
| hash_sip | 2745ns | 1943ns | 1588ns | 2103ns | 5829ns | -4.68% |
| hash_xxh | 2751ns | 1937ns | 1583ns | 2027ns | 6092ns | -4.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 384ns | 239ns | 786ns | base | 0.666 |
| hash_sip | 81ns | 47ns | 171ns | -78.85% | 3.147 |
| hash_xxh | 73ns | 42ns | 161ns | -81.01% | 3.506 |

## Performance model

- Peak throughput: **6.065 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.876 | 14.4% |
| hash_sip | 4.414 | 72.8% |
| hash_xxh | 4.961 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2880ns | 2880ns | base |
| hash_sip | 2745ns | 2745ns | -4.68% |
| hash_xxh | 2751ns | 2751ns | -4.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 292ns | base | --- | [292, 292] | --- | --- | --- | --- |
| hash_sip | 58ns | -244.3ns (-83.6%) | [-245, -192]ns | [58, 58] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 52ns | -240.7ns (-82.4%) | [-250, -234]ns | [52, 52] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 239ns | -75.7% | -78.4% |
| 1 | 2 | 292ns | -83.6% | -85.5% |
| 1 | 3 | 239ns | -75.6% | -78.4% |
| 1 | 4 | 239ns | -80.1% | -82.3% |
| 1 | 5 | 239ns | -75.7% | -78.4% |
| 1 | 6 | 292ns | -83.6% | -82.3% |
| 1 | 7 | 292ns | -83.9% | -85.5% |
| 1 | 8 | 610ns | -90.5% | -91.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | 0.068 | ok |
| hash_sip | -0.617 | HIGH- (thermal bounce) |
| hash_xxh | -0.677 | HIGH- (thermal bounce) |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 31.6ns | 384.5ns | 8.2% | HIGH |
| hash_sip | 33.7ns | 81.3ns | 41.4% | HIGH |
| hash_xxh | 33.5ns | 73.0ns | 45.9% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 239.0-785.7 ns)
    239.0 |####################################
    266.4 |########################################
    293.7 |#
    321.0 |
    348.4 |#
    375.7 |
    403.0 |#
    430.4 |
    457.7 |
    485.0 |
    512.4 |##
    539.7 |#
    567.0 |
    594.4 |###
    621.7 |
    649.0 |##
    676.3 |##
    703.7 |
    731.0 |##
    758.3 |
  (16 below, 15 above range)

hash_sip (n=160, range 47.1-171.4 ns)
     47.1 |########################
     53.3 |########################################
     59.5 |
     65.7 |
     72.0 |
     78.2 |
     84.4 |
     90.6 |
     96.8 |
    103.0 |#
    109.3 |
    115.5 |
    121.7 |
    127.9 |
    134.1 |###
    140.3 |##
    146.6 |#
    152.8 |#
    159.0 |###
    165.2 |
  (17 below, 16 above range)

hash_xxh (n=160, range 42.2-160.5 ns)
     42.2 |############
     48.1 |########################################
     54.0 |
     60.0 |#
     65.9 |
     71.8 |
     77.7 |
     83.6 |
     89.5 |
     95.5 |
    101.4 |
    107.3 |
    113.2 |
    119.1 |
    125.0 |#
    131.0 |
    136.9 |
    142.8 |####
    148.7 |
    154.6 |##
  (23 below, 14 above range)

```

## Diagnostics

- **hash_fnv1a**: CV=56.5% (high variance, measurements may be unstable)
- **hash_fnv1a**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **hash_sip**: CV=60.8% (high variance, measurements may be unstable)
- **hash_sip**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **hash_sip**: bridge=41.2% of algo (FFI overhead may distort results)
- **hash_xxh**: CV=63.5% (high variance, measurements may be unstable)
- **hash_xxh**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **hash_xxh**: bridge=45.7% of algo (FFI overhead may distort results)
