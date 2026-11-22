# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (1.25 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 233 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 18% faster than the next best (hash_sip)

hash_xxh (233 ns) leads hash_sip (274 ns) by 18%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 81% (significant)

hash_xxh is -1.02 us (81%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 5.4x slower than the field

hash_fnv1a (1.25 us) is 5.4x the fastest (233 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.4x the fastest

Fastest hash_xxh (233 ns) to slowest hash_fnv1a (1.25 us): 5.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_xxh** at 232.7 ns median (-81.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.38x (fastest 232.7 ns, slowest 1251.2 ns)

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
| hash_fnv1a | 3400ns | 3381ns | 3371ns | 3383ns | 3477ns | base |
| hash_sip | 2386ns | 2381ns | 2375ns | 2382ns | 2407ns | -29.82% |
| hash_xxh | 2363ns | 2356ns | 2349ns | 2357ns | 2394ns | -30.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 1258ns | 1247ns | 1287ns | base | 0.814 |
| hash_sip | 274ns | 269ns | 279ns | -78.20% | 3.735 |
| hash_xxh | 233ns | 229ns | 238ns | -81.48% | 4.397 |

## Performance model

- Peak throughput: **4.481 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.818 | 18.3% |
| hash_sip | 3.737 | 83.4% |
| hash_xxh | 4.401 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 3400ns | 3400ns | base |
| hash_sip | 2386ns | 2386ns | -29.82% |
| hash_xxh | 2363ns | 2363ns | -30.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 1251ns | base | --- | [1250, 1252] | --- | --- | --- | --- |
| hash_sip | 274ns | -977.1ns (-78.1%) | [-978, -976]ns | [273, 275] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 233ns | -1017.9ns (-81.4%) | [-1019, -1018]ns | [232, 233] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 1249ns | -78.1% | -81.4% |
| 1 | 2 | 1250ns | -78.2% | -81.3% |
| 1 | 3 | 1251ns | -78.1% | -81.4% |
| 1 | 4 | 1252ns | -78.1% | -81.5% |
| 1 | 5 | 1250ns | -78.0% | -81.3% |
| 1 | 6 | 1250ns | -78.1% | -81.4% |
| 1 | 7 | 1278ns | -78.5% | -81.9% |
| 1 | 8 | 1250ns | -78.1% | -81.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.198 | ok |
| hash_sip | 0.137 | ok |
| hash_xxh | -0.391 | moderate- |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.7ns | 1257.6ns | 0.2% |  |
| hash_sip | 2.6ns | 274.2ns | 1.0% |  |
| hash_xxh | 3.1ns | 232.9ns | 1.3% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 1246.6-1287.1 ns)
   1246.6 |#####################################
   1248.6 |########################################
   1250.6 |################################
   1252.7 |###################
   1254.7 |############
   1256.7 |####
   1258.7 |
   1260.8 |
   1262.8 |##
   1264.8 |##
   1266.8 |#
   1268.9 |###
   1270.9 |##
   1272.9 |######
   1274.9 |#
   1277.0 |##
   1279.0 |
   1281.0 |
   1283.0 |#
   1285.1 |
  (10 below, 10 above range)

hash_sip (n=160, range 269.4-279.4 ns)
    269.4 |######
    269.9 |##########
    270.4 |############
    270.9 |##########
    271.4 |########
    271.9 |##############
    272.4 |########################################
    272.9 |############
    273.4 |#######################
    273.9 |#####################
    274.4 |################
    274.9 |#############################
    275.4 |##############
    275.9 |############
    276.4 |##################
    276.9 |####
    277.4 |##################
    277.9 |##
    278.4 |####
    278.9 |##
  (14 below, 11 above range)

hash_xxh (n=160, range 228.5-237.6 ns)
    228.5 |####
    229.0 |######
    229.4 |############
    229.9 |####################
    230.3 |##########
    230.8 |########################
    231.2 |
    231.7 |########################################
    232.2 |################
    232.6 |########
    233.1 |########################
    233.5 |##########
    234.0 |################
    234.4 |####################
    234.9 |######
    235.3 |##############
    235.8 |##########################
    236.2 |########
    236.7 |########
    237.2 |######
  (14 below, 7 above range)

```
