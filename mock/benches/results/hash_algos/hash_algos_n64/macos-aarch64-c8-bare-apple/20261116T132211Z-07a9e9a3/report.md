# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (35 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 4 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 148% faster than the next best (hash_sip)

hash_xxh (4 ns) leads hash_sip (10 ns) by 148%, a clear separation rather than a photo finish. CV 27.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 89% (significant)

hash_xxh is -31 ns (89%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 8.3x slower than the field

hash_fnv1a (35 ns) is 8.3x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 27.2%)

hash_xxh wins on median (4 ns) yet has the highest variance (CV 27.2%), while hash_fnv1a is the steadiest (CV 10.8%, 35 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 8.3x the fastest

Fastest hash_xxh (4 ns) to slowest hash_fnv1a (35 ns): 8.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_xxh is inconsistent: worst-20% is 2.3x its best-20%

hash_xxh's best 20% of batches run at 2 ns but its worst 20% at 6 ns (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 4.2 ns median (-87.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 8.29x (fastest 4.2 ns, slowest 34.8 ns)

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
| hash_fnv1a | 2133ns | 2129ns | 2123ns | 2129ns | 2151ns | base |
| hash_sip | 2110ns | 2100ns | 2093ns | 2100ns | 2158ns | -1.04% |
| hash_xxh | 2111ns | 2098ns | 2092ns | 2098ns | 2170ns | -1.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 35ns | 30ns | 40ns | base | 1.829 |
| hash_sip | 10ns | 8ns | 13ns | -70.41% | 6.183 |
| hash_xxh | 4ns | 2ns | 6ns | -88.56% | 15.985 |

## Performance model

- Peak throughput: **25.859 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 1.839 | 7.1% |
| hash_sip | 6.154 | 23.8% |
| hash_xxh | 15.238 | 58.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2133ns | 2133ns | base |
| hash_sip | 2110ns | 2110ns | -1.04% |
| hash_xxh | 2111ns | 2111ns | -1.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 35ns | base | --- | [34, 35] | --- | --- | --- | --- |
| hash_sip | 10ns | -24.6ns (-70.7%) | [-25, -24]ns | [10, 11] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 4ns | -30.9ns (-88.6%) | [-32, -30]ns | [4, 4] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 33ns | -71.0% | -89.4% |
| 1 | 2 | 36ns | -70.8% | -89.6% |
| 1 | 3 | 35ns | -71.2% | -88.8% |
| 1 | 4 | 34ns | -69.0% | -90.4% |
| 1 | 5 | 36ns | -70.2% | -88.2% |
| 1 | 6 | 35ns | -69.9% | -86.9% |
| 1 | 7 | 34ns | -70.3% | -86.6% |
| 1 | 8 | 35ns | -69.7% | -88.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.321 | moderate- |
| hash_sip | -0.038 | ok |
| hash_xxh | 0.326 | moderate+ |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.4ns | 35.0ns | 6.9% | HIGH |
| hash_sip | 2.6ns | 10.4ns | 25.1% | HIGH |
| hash_xxh | 12.5ns | 4.0ns | 313.4% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 29.6-40.3 ns)
     29.6 |#####
     30.2 |########
     30.7 |#################
     31.2 |##############
     31.8 |###########
     32.3 |#################
     32.8 |##############
     33.4 |##################################
     33.9 |#####################################
     34.4 |#########################
     35.0 |########################################
     35.5 |####################
     36.0 |####################
     36.6 |###########
     37.1 |########################################
     37.6 |##############
     38.2 |#################
     38.7 |###########
     39.2 |#####
     39.8 |#####
  (15 below, 15 above range)

hash_sip (n=160, range 7.7-13.0 ns)
      7.7 |#########################
      8.0 |
      8.2 |##############
      8.5 |
      8.8 |#######################
      9.0 |############################
      9.3 |
      9.6 |#######################
      9.8 |##############################
     10.1 |
     10.4 |########################################
     10.6 |#########################
     10.9 |
     11.2 |##############################
     11.4 |
     11.7 |##################
     12.0 |#######################
     12.2 |
     12.5 |############################
     12.8 |####
  (12 below, 13 above range)

hash_xxh (n=160, range 2.5-5.6 ns)
      2.5 |###################
      2.6 |
      2.8 |###########################
      2.9 |
      3.1 |
      3.3 |#################################
      3.4 |
      3.6 |########################################
      3.7 |
      3.9 |
      4.0 |######################################
      4.2 |
      4.4 |
      4.5 |##################################
      4.7 |
      4.8 |
      5.0 |###############################
      5.1 |
      5.3 |#############
      5.5 |
  (10 below, 13 above range)

```

## Diagnostics

- **hash_sip**: bridge=24.0% of algo (FFI overhead may distort results)
- **hash_xxh**: CV=28.5% (high variance, measurements may be unstable)
- **hash_xxh**: bridge=297.6% of algo (FFI overhead may distort results)
