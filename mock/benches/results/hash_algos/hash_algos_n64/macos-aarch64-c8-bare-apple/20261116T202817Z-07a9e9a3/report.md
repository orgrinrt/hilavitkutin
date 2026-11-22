# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (35 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 4 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 181% faster than the next best (hash_sip)

hash_xxh (4 ns) leads hash_sip (10 ns) by 181%, a clear separation rather than a photo finish. CV 32.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 89% (significant)

hash_xxh is -31 ns (89%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 9.5x slower than the field

hash_fnv1a (35 ns) is 9.5x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 32.6%)

hash_xxh wins on median (4 ns) yet has the highest variance (CV 32.6%), while hash_fnv1a is the steadiest (CV 10.9%, 35 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### hash_xxh shows warm-up / thermal drift (autocorr +0.53)

hash_xxh's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 9.5x the fastest

Fastest hash_xxh (4 ns) to slowest hash_fnv1a (35 ns): 9.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_xxh is inconsistent: worst-20% is 2.4x its best-20%

hash_xxh's best 20% of batches run at 2 ns but its worst 20% at 6 ns (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 3.7 ns median (-89.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 9.51x (fastest 3.7 ns, slowest 35.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hash_fnv1a | 2153ns | 2133ns | 2126ns | 2144ns | 2206ns | base |
| hash_sip | 2121ns | 2102ns | 2094ns | 2107ns | 2189ns | -1.47% |
| hash_xxh | 2109ns | 2098ns | 2092ns | 2102ns | 2148ns | -2.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 35ns | 30ns | 41ns | base | 1.810 |
| hash_sip | 10ns | 7ns | 13ns | -71.03% | 6.248 |
| hash_xxh | 4ns | 2ns | 6ns | -88.96% | 16.397 |

## Performance model

- Peak throughput: **27.527 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 1.818 | 6.6% |
| hash_sip | 6.154 | 22.4% |
| hash_xxh | 17.297 | 62.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2153ns | 2153ns | base |
| hash_sip | 2121ns | 2121ns | -1.47% |
| hash_xxh | 2109ns | 2109ns | -2.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 35ns | base | --- | [35, 36] | --- | --- | --- | --- |
| hash_sip | 10ns | -25.4ns (-72.2%) | [-26, -25]ns | [10, 11] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 4ns | -31.3ns (-88.9%) | [-32, -31]ns | [4, 4] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 35ns | -70.6% | -89.5% |
| 1 | 2 | 34ns | -69.6% | -90.2% |
| 1 | 3 | 35ns | -72.2% | -89.5% |
| 1 | 4 | 35ns | -71.8% | -89.4% |
| 1 | 5 | 37ns | -72.5% | -89.9% |
| 1 | 6 | 35ns | -69.1% | -88.0% |
| 1 | 7 | 35ns | -70.5% | -88.1% |
| 1 | 8 | 36ns | -70.9% | -87.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.210 | moderate- |
| hash_sip | 0.407 | moderate+ |
| hash_xxh | 0.526 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.6ns | 35.4ns | 7.4% | HIGH |
| hash_sip | 2.6ns | 10.2ns | 25.5% | HIGH |
| hash_xxh | 12.5ns | 3.9ns | 319.5% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 29.9-40.7 ns)
     29.9 |##
     30.4 |############
     30.9 |#####
     31.5 |############
     32.0 |######################
     32.6 |#######
     33.1 |####################
     33.6 |###############
     34.2 |########################################
     34.7 |##############################
     35.3 |################################
     35.8 |############
     36.3 |####################
     36.9 |#################
     37.4 |################################
     38.0 |###############
     38.5 |##########
     39.1 |##
     39.6 |###############
     40.1 |#######
  (13 below, 14 above range)

hash_sip (n=160, range 7.4-13.3 ns)
      7.4 |###############
      7.7 |###############
      8.0 |
      8.3 |########################################
      8.6 |#################
      8.9 |
      9.1 |#########################
      9.4 |#########################
      9.7 |#########################
     10.0 |
     10.3 |###########################
     10.6 |########################################
     10.9 |####################
     11.2 |
     11.5 |##############################
     11.8 |#########################
     12.1 |
     12.4 |###############
     12.7 |############
     13.0 |
  (13 below, 14 above range)

hash_xxh (n=160, range 2.3-5.6 ns)
      2.3 |
      2.5 |##############################
      2.7 |
      2.8 |##############################
      3.0 |
      3.2 |####################################
      3.3 |
      3.5 |
      3.6 |##################################
      3.8 |
      4.0 |
      4.1 |#########################
      4.3 |
      4.5 |########################################
      4.6 |
      4.8 |
      5.0 |#######################
      5.1 |
      5.3 |####################
      5.5 |
  (13 below, 14 above range)

```

## Diagnostics

- **hash_sip**: CV=21.0% (high variance, measurements may be unstable)
- **hash_sip**: bridge=24.0% of algo (FFI overhead may distort results)
- **hash_xxh**: CV=30.9% (high variance, measurements may be unstable)
- **hash_xxh**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **hash_xxh**: bridge=337.8% of algo (FFI overhead may distort results)
