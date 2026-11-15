# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (5.26 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 1.15 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh beats baseline by 79% (significant)

hash_xxh is -4.14 us (79%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 4.6x slower than the field

hash_fnv1a (5.26 us) is 4.6x the fastest (1.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 5.9%)

hash_xxh wins on median (1.15 us) yet has the highest variance (CV 5.9%), while hash_fnv1a is the steadiest (CV 2.8%, 5.26 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.6x the fastest

Fastest hash_xxh (1.15 us) to slowest hash_fnv1a (5.26 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_xxh** at 1145.8 ns median (-78.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.59x (fastest 1145.8 ns, slowest 5262.5 ns)

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
| hash_fnv1a | 7459ns | 7449ns | 7163ns | 7475ns | 7706ns | base |
| hash_sip | 3386ns | 3392ns | 3237ns | 3383ns | 3543ns | -54.60% |
| hash_xxh | 3407ns | 3395ns | 3163ns | 3389ns | 3706ns | -54.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 5277ns | 5068ns | 5458ns | base | 0.776 |
| hash_sip | 1187ns | 1133ns | 1243ns | -77.51% | 3.452 |
| hash_xxh | 1150ns | 1068ns | 1249ns | -78.21% | 3.562 |

## Performance model

- Peak throughput: **3.834 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.778 | 20.3% |
| hash_sip | 3.435 | 89.6% |
| hash_xxh | 3.575 | 93.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 7459ns | 7459ns | base |
| hash_sip | 3386ns | 3386ns | -54.60% |
| hash_xxh | 3407ns | 3407ns | -54.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 5262ns | base | --- | [5245, 5304] | --- | --- | --- | --- |
| hash_sip | 1192ns | -4091.1ns (-77.7%) | [-4134, -4065]ns | [1174, 1201] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 1146ns | -4137.3ns (-78.6%) | [-4193, -4100]ns | [1144, 1148] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 5124ns | -77.3% | -77.6% |
| 1 | 2 | 5239ns | -78.0% | -78.0% |
| 1 | 3 | 5254ns | -77.6% | -79.6% |
| 1 | 4 | 5428ns | -77.8% | -79.0% |
| 1 | 5 | 5081ns | -76.3% | -77.4% |
| 1 | 6 | 5428ns | -78.4% | -77.3% |
| 1 | 7 | 5405ns | -79.0% | -78.8% |
| 1 | 8 | 5429ns | -77.8% | -80.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.146 | ok |
| hash_sip | -0.013 | ok |
| hash_xxh | -0.010 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.8ns | 5277.3ns | 0.1% |  |
| hash_sip | 2.8ns | 1186.7ns | 0.2% |  |
| hash_xxh | 3.4ns | 1150.0ns | 0.3% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 5068.5-5457.6 ns)
   5068.5 |###
   5087.9 |##
   5107.4 |#########
   5126.8 |####
   5146.3 |#
   5165.7 |
   5185.2 |
   5204.7 |##
   5224.1 |#############
   5243.6 |#############
   5263.0 |#####
   5282.5 |####
   5302.0 |##
   5321.4 |
   5340.9 |
   5360.3 |####
   5379.8 |
   5399.3 |#
   5418.7 |########################################
   5438.2 |###
  (24 below, 9 above range)

hash_sip (n=160, range 1132.8-1242.6 ns)
   1132.8 |
   1138.3 |##
   1143.8 |###
   1149.2 |
   1154.7 |###########
   1160.2 |####################
   1165.7 |###############
   1171.2 |###########
   1176.7 |##
   1182.2 |###
   1187.7 |##
   1193.2 |######
   1198.7 |###########################
   1204.2 |########################################
   1209.7 |#####
   1215.2 |##
   1220.7 |
   1226.2 |#
   1231.7 |
   1237.1 |#
  (22 below, 13 above range)

hash_xxh (n=160, range 1068.2-1249.4 ns)
   1068.2 |####################
   1077.3 |#
   1086.4 |#
   1095.4 |
   1104.5 |#
   1113.5 |##
   1122.6 |####
   1131.7 |##
   1140.7 |########################################
   1149.8 |#########
   1158.8 |
   1167.9 |
   1177.0 |
   1186.0 |
   1195.1 |#
   1204.1 |
   1213.2 |
   1222.3 |
   1231.3 |############################
   1240.4 |#
  (14 below, 3 above range)

```
