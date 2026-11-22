# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (1.23 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 231 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 17% faster than the next best (hash_sip)

hash_xxh (231 ns) leads hash_sip (269 ns) by 17%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 81% (significant)

hash_xxh is -998 ns (81%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 5.3x slower than the field

hash_fnv1a (1.23 us) is 5.3x the fastest (231 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.3x the fastest

Fastest hash_xxh (231 ns) to slowest hash_fnv1a (1.23 us): 5.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_xxh** at 230.8 ns median (-81.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.32x (fastest 230.8 ns, slowest 1227.7 ns)

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
| hash_fnv1a | 3338ns | 3330ns | 3323ns | 3330ns | 3375ns | base |
| hash_sip | 2360ns | 2356ns | 2349ns | 2356ns | 2385ns | -29.29% |
| hash_xxh | 2345ns | 2332ns | 2325ns | 2332ns | 2402ns | -29.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 1230ns | 1224ns | 1244ns | base | 0.832 |
| hash_sip | 270ns | 265ns | 275ns | -78.07% | 3.796 |
| hash_xxh | 231ns | 226ns | 237ns | -81.23% | 4.434 |

## Performance model

- Peak throughput: **4.533 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.834 | 18.4% |
| hash_sip | 3.804 | 83.9% |
| hash_xxh | 4.437 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 3338ns | 3338ns | base |
| hash_sip | 2360ns | 2360ns | -29.29% |
| hash_xxh | 2345ns | 2345ns | -29.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 1228ns | base | --- | [1227, 1228] | --- | --- | --- | --- |
| hash_sip | 269ns | -958.0ns (-78.0%) | [-958, -958]ns | [269, 270] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 231ns | -997.7ns (-81.3%) | [-998, -996]ns | [230, 231] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 1226ns | -78.1% | -81.3% |
| 1 | 2 | 1228ns | -78.1% | -81.3% |
| 1 | 3 | 1228ns | -78.1% | -81.3% |
| 1 | 4 | 1228ns | -78.0% | -81.2% |
| 1 | 5 | 1228ns | -78.1% | -81.2% |
| 1 | 6 | 1226ns | -78.0% | -81.1% |
| 1 | 7 | 1228ns | -77.9% | -81.2% |
| 1 | 8 | 1227ns | -78.1% | -80.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.226 | moderate- |
| hash_sip | -0.095 | ok |
| hash_xxh | 0.073 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.6ns | 1230.2ns | 0.2% |  |
| hash_sip | 2.5ns | 269.7ns | 0.9% |  |
| hash_xxh | 2.8ns | 230.9ns | 1.2% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 1223.9-1243.9 ns)
   1223.9 |#####
   1224.9 |########################################
   1225.9 |#######################
   1226.9 |#########################
   1227.9 |####################
   1228.9 |##################
   1229.9 |##########
   1230.9 |###
   1231.9 |##########
   1232.9 |##
   1233.9 |##
   1234.9 |#
   1235.9 |#
   1236.9 |
   1237.9 |#
   1238.9 |##
   1239.9 |
   1240.9 |
   1241.9 |#
   1242.9 |#
  (15 below, 12 above range)

hash_sip (n=160, range 265.1-275.0 ns)
    265.1 |####
    265.6 |##############
    266.1 |########
    266.6 |##
    267.1 |#########################
    267.6 |################
    268.1 |##################
    268.6 |#################################
    269.1 |################
    269.6 |########################################
    270.0 |############
    270.5 |########
    271.0 |##########
    271.5 |#####################
    272.0 |########
    272.5 |##########
    273.0 |########
    273.5 |####
    274.0 |############
    274.5 |####
  (14 below, 12 above range)

hash_xxh (n=160, range 225.9-236.9 ns)
    225.9 |######
    226.4 |
    227.0 |######################
    227.5 |###############
    228.1 |#############
    228.6 |#################
    229.2 |########################
    229.7 |#############
    230.3 |########################################
    230.8 |##########################
    231.4 |######
    231.9 |###############################
    232.5 |#############
    233.0 |###########
    233.6 |###########
    234.1 |#############
    234.7 |######
    235.2 |####
    235.8 |####
    236.3 |##
  (17 below, 15 above range)

```
