# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (5.16 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 1.09 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh beats baseline by 79% (significant)

hash_xxh is -4.07 us (79%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 4.7x slower than the field

hash_fnv1a (5.16 us) is 4.7x the fastest (1.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.7x the fastest

Fastest hash_xxh (1.09 us) to slowest hash_fnv1a (5.16 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_xxh** at 1089.2 ns median (-78.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.74x (fastest 1089.2 ns, slowest 5159.2 ns)

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
| hash_fnv1a | 7278ns | 7288ns | 7169ns | 7289ns | 7357ns | base |
| hash_sip | 3265ns | 3260ns | 3250ns | 3260ns | 3298ns | -55.14% |
| hash_xxh | 3235ns | 3220ns | 3209ns | 3221ns | 3303ns | -55.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 5152ns | 5075ns | 5210ns | base | 0.795 |
| hash_sip | 1147ns | 1140ns | 1162ns | -77.74% | 3.572 |
| hash_xxh | 1095ns | 1085ns | 1123ns | -78.74% | 3.740 |

## Performance model

- Peak throughput: **3.775 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.794 | 21.0% |
| hash_sip | 3.581 | 94.9% |
| hash_xxh | 3.761 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 7278ns | 7278ns | base |
| hash_sip | 3265ns | 3265ns | -55.14% |
| hash_xxh | 3235ns | 3235ns | -55.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 5159ns | base | --- | [5158, 5160] | --- | --- | --- | --- |
| hash_sip | 1144ns | -4014.5ns (-77.8%) | [-4015, -4014]ns | [1143, 1144] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 1089ns | -4070.0ns (-78.9%) | [-4071, -4069]ns | [1089, 1090] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 5159ns | -77.8% | -78.9% |
| 1 | 2 | 5162ns | -77.8% | -78.9% |
| 1 | 3 | 5131ns | -77.7% | -78.8% |
| 1 | 4 | 5159ns | -77.8% | -78.9% |
| 1 | 5 | 5160ns | -77.8% | -78.9% |
| 1 | 6 | 5158ns | -77.8% | -78.9% |
| 1 | 7 | 5159ns | -77.8% | -78.9% |
| 1 | 8 | 5161ns | -77.9% | -78.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.206 | moderate- |
| hash_sip | 0.107 | ok |
| hash_xxh | -0.153 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.9ns | 5152.4ns | 0.1% |  |
| hash_sip | 2.7ns | 1146.7ns | 0.2% |  |
| hash_xxh | 3.4ns | 1095.2ns | 0.3% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 5074.7-5210.4 ns)
   5074.7 |#
   5081.5 |#
   5088.3 |
   5095.0 |
   5101.8 |
   5108.6 |
   5115.4 |
   5122.2 |
   5129.0 |
   5135.8 |
   5142.6 |
   5149.3 |############
   5156.1 |########################################
   5162.9 |##########
   5169.7 |
   5176.5 |
   5183.3 |#
   5190.1 |#
   5196.8 |
   5203.6 |#
  (22 below, 15 above range)

hash_sip (n=160, range 1139.9-1162.0 ns)
   1139.9 |##################
   1141.1 |########################################
   1142.2 |#####################
   1143.3 |#####################################
   1144.4 |########################
   1145.5 |############
   1146.6 |###############
   1147.7 |#######
   1148.8 |###########
   1149.9 |########
   1151.0 |####
   1152.1 |
   1153.2 |
   1154.3 |
   1155.4 |
   1156.5 |
   1157.6 |
   1158.7 |
   1159.8 |##
   1160.9 |
  (11 below, 6 above range)

hash_xxh (n=160, range 1084.9-1123.0 ns)
   1084.9 |############################
   1086.8 |#########################
   1088.7 |########################################
   1090.6 |#######################
   1092.5 |###########
   1094.4 |####
   1096.3 |##
   1098.2 |##
   1100.2 |
   1102.1 |
   1104.0 |###
   1105.9 |
   1107.8 |
   1109.7 |
   1111.6 |#
   1113.5 |#
   1115.4 |##
   1117.3 |
   1119.2 |
   1121.1 |
  (11 below, 7 above range)

```
