# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (5.06 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 1.07 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh beats baseline by 79% (significant)

hash_xxh is -3.99 us (79%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 4.7x slower than the field

hash_fnv1a (5.06 us) is 4.7x the fastest (1.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.7x the fastest

Fastest hash_xxh (1.07 us) to slowest hash_fnv1a (5.06 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_xxh** at 1070.0 ns median (-78.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.73x (fastest 1070.0 ns, slowest 5062.9 ns)

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
| hash_fnv1a | 7217ns | 7165ns | 7158ns | 7168ns | 7423ns | base |
| hash_sip | 3216ns | 3210ns | 3204ns | 3210ns | 3245ns | -55.44% |
| hash_xxh | 3183ns | 3172ns | 3166ns | 3172ns | 3231ns | -55.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 5100ns | 5059ns | 5246ns | base | 0.803 |
| hash_sip | 1124ns | 1118ns | 1133ns | -77.96% | 3.644 |
| hash_xxh | 1074ns | 1065ns | 1093ns | -78.95% | 3.815 |

## Performance model

- Peak throughput: **3.845 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.809 | 21.0% |
| hash_sip | 3.646 | 94.8% |
| hash_xxh | 3.828 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 7217ns | 7217ns | base |
| hash_sip | 3216ns | 3216ns | -55.44% |
| hash_xxh | 3183ns | 3183ns | -55.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 5063ns | base | --- | [5062, 5064] | --- | --- | --- | --- |
| hash_sip | 1123ns | -3940.2ns (-77.8%) | [-3941, -3940]ns | [1122, 1124] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 1070ns | -3993.3ns (-78.9%) | [-3994, -3992]ns | [1069, 1070] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 5063ns | -77.8% | -78.4% |
| 1 | 2 | 5062ns | -77.8% | -78.9% |
| 1 | 3 | 5062ns | -77.8% | -78.9% |
| 1 | 4 | 5292ns | -78.8% | -79.8% |
| 1 | 5 | 5062ns | -77.8% | -78.9% |
| 1 | 6 | 5063ns | -77.8% | -78.9% |
| 1 | 7 | 5062ns | -77.8% | -78.9% |
| 1 | 8 | 5062ns | -77.8% | -78.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.160 | ok |
| hash_sip | -0.361 | moderate- |
| hash_xxh | -0.070 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.6ns | 5099.7ns | 0.1% |  |
| hash_sip | 2.4ns | 1124.0ns | 0.2% |  |
| hash_xxh | 3.3ns | 1073.6ns | 0.3% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 5059.1-5246.1 ns)
   5059.1 |########################################
   5068.4 |##
   5077.8 |#
   5087.1 |
   5096.5 |#
   5105.8 |
   5115.2 |
   5124.5 |
   5133.9 |
   5143.2 |
   5152.6 |
   5161.9 |
   5171.3 |
   5180.6 |
   5190.0 |
   5199.3 |
   5208.7 |
   5218.0 |
   5227.4 |
   5236.7 |#
  (12 below, 16 above range)

hash_sip (n=160, range 1118.2-1132.5 ns)
   1118.2 |##############
   1118.9 |##########
   1119.7 |##########
   1120.4 |########################
   1121.1 |######################################
   1121.8 |####################
   1122.5 |##############
   1123.2 |########################################
   1123.9 |####################################
   1124.7 |##########
   1125.4 |##########
   1126.1 |####################
   1126.8 |##########
   1127.5 |####
   1128.2 |######
   1128.9 |######
   1129.7 |##
   1130.4 |
   1131.1 |
   1131.8 |
  (13 below, 10 above range)

hash_xxh (n=160, range 1065.3-1092.8 ns)
   1065.3 |#################
   1066.6 |########################################
   1068.0 |#################################
   1069.4 |########################################
   1070.8 |##############################
   1072.2 |##############
   1073.5 |#########
   1074.9 |###########
   1076.3 |####
   1077.7 |
   1079.0 |######
   1080.4 |
   1081.8 |
   1083.2 |#
   1084.6 |
   1085.9 |###
   1087.3 |
   1088.7 |
   1090.1 |###
   1091.4 |###
  (13 below, 10 above range)

```
