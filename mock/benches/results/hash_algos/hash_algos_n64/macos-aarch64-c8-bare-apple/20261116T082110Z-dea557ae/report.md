# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (36 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 4 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 114% faster than the next best (hash_sip)

hash_xxh (4 ns) leads hash_sip (8 ns) by 114%, a clear separation rather than a photo finish. CV 30.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 90% (significant)

hash_xxh is -32 ns (90%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 9.7x slower than the field

hash_fnv1a (36 ns) is 9.7x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 30.9%)

hash_xxh wins on median (4 ns) yet has the highest variance (CV 30.9%), while hash_fnv1a is the steadiest (CV 11.0%, 36 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 9.7x the fastest

Fastest hash_xxh (4 ns) to slowest hash_fnv1a (36 ns): 9.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_xxh is inconsistent: worst-20% is 2.4x its best-20%

hash_xxh's best 20% of batches run at 2 ns but its worst 20% at 6 ns (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 3.7 ns median (-89.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 9.68x (fastest 3.7 ns, slowest 35.8 ns)

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
| hash_fnv1a | 2165ns | 2159ns | 2151ns | 2159ns | 2199ns | base |
| hash_sip | 2130ns | 2127ns | 2121ns | 2128ns | 2146ns | -1.64% |
| hash_xxh | 2126ns | 2125ns | 2114ns | 2125ns | 2142ns | -1.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 36ns | 30ns | 41ns | base | 1.777 |
| hash_sip | 8ns | 6ns | 11ns | -76.99% | 7.725 |
| hash_xxh | 4ns | 2ns | 6ns | -89.27% | 16.564 |

## Performance model

- Peak throughput: **27.380 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 1.788 | 6.5% |
| hash_sip | 8.101 | 29.6% |
| hash_xxh | 17.297 | 63.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2165ns | 2165ns | base |
| hash_sip | 2130ns | 2130ns | -1.64% |
| hash_xxh | 2126ns | 2126ns | -1.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 36ns | base | --- | [35, 37] | --- | --- | --- | --- |
| hash_sip | 8ns | -27.9ns (-77.9%) | [-28, -27]ns | [8, 8] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 4ns | -32.1ns (-89.7%) | [-33, -31]ns | [4, 4] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 36ns | -75.1% | -88.4% |
| 1 | 2 | 35ns | -77.4% | -89.4% |
| 1 | 3 | 36ns | -77.7% | -90.3% |
| 1 | 4 | 36ns | -79.5% | -89.6% |
| 1 | 5 | 36ns | -79.2% | -89.7% |
| 1 | 6 | 37ns | -72.8% | -89.9% |
| 1 | 7 | 36ns | -78.5% | -89.0% |
| 1 | 8 | 36ns | -79.3% | -89.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.409 | moderate- |
| hash_sip | -0.227 | moderate- |
| hash_xxh | -0.026 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.9ns | 36.0ns | 8.0% | HIGH |
| hash_sip | 2.5ns | 8.3ns | 29.6% | HIGH |
| hash_xxh | 13.2ns | 3.9ns | 342.8% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 30.5-41.4 ns)
     30.5 |##
     31.0 |######
     31.6 |##
     32.1 |########
     32.6 |############
     33.2 |############
     33.7 |########################################
     34.3 |##########
     34.8 |########
     35.4 |################################
     35.9 |############
     36.5 |##################
     37.0 |##############
     37.6 |################
     38.1 |########
     38.7 |############
     39.2 |############################
     39.7 |##############
     40.3 |########
     40.8 |####
  (16 below, 11 above range)

hash_sip (n=160, range 5.7-11.2 ns)
      5.7 |######################
      6.0 |########################################
      6.3 |
      6.6 |############################
      6.8 |#####################################
      7.1 |
      7.4 |########################################
      7.7 |#####################################
      7.9 |
      8.2 |########################################
      8.5 |
      8.8 |######################
      9.0 |###############################
      9.3 |
      9.6 |########################################
      9.9 |######################
     10.1 |
     10.4 |###########
     10.7 |###########
     10.9 |####################
  (9 below, 9 above range)

hash_xxh (n=160, range 2.3-5.5 ns)
      2.3 |
      2.5 |#################
      2.7 |
      2.8 |#########################
      3.0 |
      3.1 |
      3.3 |#########################
      3.4 |
      3.6 |########################################
      3.8 |
      3.9 |
      4.1 |######################
      4.2 |
      4.4 |
      4.6 |###########################
      4.7 |
      4.9 |#####################
      5.0 |
      5.2 |
      5.3 |###############
  (14 below, 9 above range)

```

## Diagnostics

- **hash_sip**: CV=24.0% (high variance, measurements may be unstable)
- **hash_sip**: bridge=29.1% of algo (FFI overhead may distort results)
- **hash_xxh**: CV=29.6% (high variance, measurements may be unstable)
- **hash_xxh**: bridge=348.6% of algo (FFI overhead may distort results)
