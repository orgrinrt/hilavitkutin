# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (1.23 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 230 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 18% faster than the next best (hash_sip)

hash_xxh (230 ns) leads hash_sip (270 ns) by 18%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 81% (significant)

hash_xxh is -998 ns (81%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 5.3x slower than the field

hash_fnv1a (1.23 us) is 5.3x the fastest (230 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.3x the fastest

Fastest hash_xxh (230 ns) to slowest hash_fnv1a (1.23 us): 5.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_xxh** at 229.6 ns median (-81.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.34x (fastest 229.6 ns, slowest 1226.9 ns)

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
| hash_fnv1a | 3347ns | 3330ns | 3324ns | 3331ns | 3420ns | base |
| hash_sip | 2367ns | 2354ns | 2348ns | 2355ns | 2421ns | -29.28% |
| hash_xxh | 2332ns | 2328ns | 2322ns | 2328ns | 2356ns | -30.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 1233ns | 1223ns | 1260ns | base | 0.830 |
| hash_sip | 271ns | 266ns | 281ns | -77.99% | 3.774 |
| hash_xxh | 230ns | 225ns | 234ns | -81.38% | 4.461 |

## Performance model

- Peak throughput: **4.541 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.835 | 18.4% |
| hash_sip | 3.793 | 83.5% |
| hash_xxh | 4.460 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 3347ns | 3347ns | base |
| hash_sip | 2367ns | 2367ns | -29.28% |
| hash_xxh | 2332ns | 2332ns | -30.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 1227ns | base | --- | [1226, 1228] | --- | --- | --- | --- |
| hash_sip | 270ns | -957.5ns (-78.0%) | [-958, -957]ns | [270, 271] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 230ns | -998.1ns (-81.4%) | [-999, -997]ns | [229, 230] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 1226ns | -78.1% | -81.3% |
| 1 | 2 | 1227ns | -78.0% | -81.3% |
| 1 | 3 | 1226ns | -78.0% | -81.4% |
| 1 | 4 | 1228ns | -78.0% | -81.3% |
| 1 | 5 | 1227ns | -78.0% | -81.3% |
| 1 | 6 | 1226ns | -78.0% | -81.2% |
| 1 | 7 | 1270ns | -78.8% | -81.9% |
| 1 | 8 | 1227ns | -77.2% | -81.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.159 | ok |
| hash_sip | -0.040 | ok |
| hash_xxh | 0.055 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.6ns | 1233.0ns | 0.2% |  |
| hash_sip | 2.5ns | 271.3ns | 0.9% |  |
| hash_xxh | 2.7ns | 229.6ns | 1.2% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 1223.3-1259.5 ns)
   1223.3 |#############################
   1225.1 |########################################
   1226.9 |###########################
   1228.8 |############
   1230.6 |##############
   1232.4 |
   1234.2 |#
   1236.0 |
   1237.8 |#
   1239.6 |
   1241.4 |
   1243.2 |
   1245.0 |##
   1246.8 |#
   1248.7 |
   1250.5 |#
   1252.3 |#
   1254.1 |
   1255.9 |#
   1257.7 |
  (14 below, 20 above range)

hash_sip (n=160, range 265.5-280.8 ns)
    265.5 |######################
    266.3 |###########
    267.0 |######################
    267.8 |##########################
    268.6 |########################################
    269.3 |#####################################
    270.1 |########################################
    270.9 |###########
    271.6 |############################
    272.4 |###############
    273.2 |######
    273.9 |####
    274.7 |###########
    275.5 |####
    276.2 |
    277.0 |######
    277.7 |########
    278.5 |####
    279.3 |######
    280.0 |##
  (12 below, 8 above range)

hash_xxh (n=160, range 225.5-233.8 ns)
    225.5 |#######
    225.9 |##########
    226.3 |#####################
    226.7 |#####################
    227.1 |#########################
    227.6 |########################################
    228.0 |#############################
    228.4 |########################################
    228.8 |#########################
    229.2 |####################################
    229.6 |#####################
    230.1 |########################################
    230.5 |##################
    230.9 |####################################
    231.3 |##################
    231.7 |##############
    232.1 |#####################
    232.6 |##########
    233.0 |##################
    233.4 |##############
  (17 below, 13 above range)

```
